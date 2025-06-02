// rpc.rs
// Handles JSON-RPC server and request/response logic

use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::io::AsyncWriteExt;
use xelarius_core::Blockchain;
use serde_json::json;

pub async fn start_rpc(chain: Arc<std::sync::Mutex<Blockchain>>) {
    let listener = TcpListener::bind("127.0.0.1:8545").await.expect("bind");
    loop {
        let (mut socket, _) = listener.accept().await.expect("accept");
        let chain = chain.clone();
        tokio::spawn(async move {
            let chain = chain.lock().unwrap();
            let resp = json!({"jsonrpc":"2.0","result":chain.chain.len(),"id":1});
            let _ = socket.write_all(resp.to_string().as_bytes()).await;
        });
    }
}
