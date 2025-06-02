// network.rs
// Handles libp2p setup, swarm, and event loop

use libp2p::{
    core::upgrade,
    gossipsub::{Behaviour as Gossipsub, Config as GossipsubConfig, Event as GossipsubEvent, IdentTopic, MessageAuthenticity},
    identity,
    noise::{NoiseConfig, X25519Spec, Keypair as NoiseKeypair},
    swarm::{Swarm, SwarmBuilder},
    tcp::{Transport, Config as TcpConfig},
    yamux::YamuxConfig,
    PeerId,
    Transport as Libp2pTransport,
};
use tokio::sync::mpsc;
use tracing::{info, error};
use lazy_static::lazy_static;
use prometheus::{IntGauge, Registry};
use std::time::SystemTime;

lazy_static! {
    static ref NETWORK_LATENCY: IntGauge = IntGauge::new("network_latency", "Network latency in ms").unwrap();
}

pub async fn setup_network() -> (mpsc::UnboundedSender<Vec<u8>>, mpsc::UnboundedReceiver<Vec<u8>>, PeerId) {
    info!("Setting up network...");
    let registry = Registry::new();
    registry.register(Box::new(NETWORK_LATENCY.clone())).unwrap();

    let id_keys = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(id_keys.public());

    let noise_keys = NoiseKeypair::<X25519Spec>::new().into_authentic(&id_keys).unwrap();
    let transport = Transport::new(TcpConfig::default())
        .upgrade(upgrade::Version::V1)
        .authenticate(NoiseConfig::xx(noise_keys).unwrap())
        .multiplex(YamuxConfig::default())
        .boxed();

    let gossipsub_config = GossipsubConfig::default();
    let mut behaviour = Gossipsub::new(MessageAuthenticity::Signed(id_keys.clone()), gossipsub_config).unwrap();
    let topic = IdentTopic::new("xelarius-blocks");
    behaviour.subscribe(&topic).unwrap();

    let mut swarm = SwarmBuilder::with_executor(transport, behaviour, peer_id).build();

    let (net_tx, mut net_rx) = mpsc::unbounded_channel();

    use futures::StreamExt;
    tokio::spawn(async move {
        loop {
            tokio::select! {
                Some(msg) = net_rx.recv() => {
                    let start = SystemTime::now();
                    if let Err(e) = swarm.behaviour_mut().publish(topic.clone(), msg) {
                        error!("Failed to publish message: {:?}", e);
                    } else {
                        let latency = SystemTime::now().duration_since(start).unwrap().as_millis() as i64;
                        NETWORK_LATENCY.set(latency);
                        info!("Message published successfully. Latency: {} ms", latency);
                    }
                }
                event = swarm.next() => {
                    if let Some(libp2p::swarm::SwarmEvent::Behaviour(GossipsubEvent::Message { message, .. })) = event {
                        info!("Received gossipsub message: {:?}", message.data);
                        // TODO: Deserialize and process block/tx
                    }
                }
            }
        }
    });

    info!("Network setup complete.");
    (net_tx, net_rx, peer_id)
}
