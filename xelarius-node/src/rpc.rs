// rpc.rs
// Handles JSON-RPC server and request/response logic

use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::io::AsyncWriteExt;
use xelarius_core::{Blockchain, Wallet, Token};
use serde_json::json;
use tracing::{info, error};
use lazy_static::lazy_static;

pub async fn start_rpc(chain: Arc<std::sync::Mutex<Blockchain>>, token: Arc<std::sync::Mutex<Token>>) {
    let listener = TcpListener::bind("127.0.0.1:8545").await.expect("bind");
    loop {
        let (mut socket, _) = listener.accept().await.expect("accept");
        let chain = chain.clone();
        let token = token.clone();
        tokio::spawn(async move {
            let request = "dummy_request"; // Replace with actual request parsing
            match request {
                "create_wallet" => {
                    let wallet = Wallet::new("ckelley.eth", "private_key_stub");
                    let resp = json!({"wallet": wallet});
                    let _ = socket.write_all(resp.to_string().as_bytes()).await;
                }
                "mint_token" => {
                    let mut token_data = {
                        let mut token = token.lock().unwrap();
                        token.transfer("genesis", "ckelley.eth", 1000).unwrap();
                        token.clone()
                    };
                    let resp = json!({"status": "success", "token": token_data});
                    let _ = socket.write_all(resp.to_string().as_bytes()).await;
                }
                "submit_tx" => {
                    let len = {
                        let chain = chain.lock().unwrap();
                        chain.chain.len()
                    };
                    let resp = json!({"jsonrpc":"2.0","result":len,"id":1});
                    let _ = socket.write_all(resp.to_string().as_bytes()).await;
                }
                _ => {
                    let resp = json!({"error": "unknown method"});
                    let _ = socket.write_all(resp.to_string().as_bytes()).await;
                }
            }
        });
    }
}
