// src/websocket.rs

use rocket::tokio::sync::broadcast;
use rocket::State;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::connect_async;
use futures_util::{StreamExt, SinkExt};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Block {
    id: String,
    data: String,
}

#[get("/blockstream")]
pub async fn blockstream(tx: &State<Arc<broadcast::Sender<Block>>>) -> Result<(), rocket::http::Status> {
    let (ws_stream, _) = match connect_async("ws://localhost:8000/blockstream").await {
        Ok(ws) => ws,
        Err(_) => {
            eprintln!("Failed to connect to WebSocket");
            return Err(rocket::http::Status::InternalServerError);
        },
    };

    let (mut write, mut read) = ws_stream.split();
    let mut rx = tx.subscribe();

    tokio::select! {
        _ = async {
            while let Ok(block) = rx.recv().await {
                let message = match serde_json::to_string(&block) {
                    Ok(msg) => msg,
                    Err(_) => {
                        eprintln!("Failed to serialize block to JSON");
                        continue;
                    }
                };
                if let Err(_) = write.send(Message::Text(message)).await {
                    eprintln!("Failed to send message");
                    break;
                }
            }
        } => {},
        _ = async {
            while let Some(result) = read.next().await {
                match result {
                    Ok(_message) => {
                      // Lida com msgs recebidas, caso necessario
                    },
                    Err(e) => eprintln!("WebSocket read error: {:?}", e),
                }
            }
        } => {},
    }

    Ok(())
}

pub async fn start_blockstream() -> Arc<broadcast::Sender<Block>> {
    let (tx, _) = broadcast::channel(100);
    let tx = Arc::new(tx);

    let tx_clone = Arc::clone(&tx);
    tokio::spawn(async move {
        let mut count = 0;
        loop {
            let block = Block {
                id: format!("block_{}", count),
                data: "Block data".to_string(),
            };
            if let Err(_) = tx_clone.send(block) {
                eprintln!("Failed to send block to channel");
            }
            count += 1;
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    });

    tx
}
