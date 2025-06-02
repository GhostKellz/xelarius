// rpc.rs
// Handles JSON-RPC server and request/response logic

use std::sync::Arc;
use tokio::net::TcpListener;
use xelarius_core::Blockchain;

pub async fn start_rpc(_chain: Arc<std::sync::Mutex<Blockchain>>) {
    // TODO: Implement JSON-RPC server
    unimplemented!("start_rpc not yet implemented")
}
