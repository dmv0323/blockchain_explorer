// src/websocket.rs
use rocket::tokio::sync::broadcast;
use rocket::State;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::connect_async;
use futures_util::{StreamExt, SinkExt};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Block {
    id: String,
    data: String,
}

#[get("/blockstream")]
async fn blockstream(tx: &State<Arc<broadcast::Sender<Block>>>) -> Result<(), rocket::http::Status> {
    let (ws_stream, _) = connect_async("ws://localhost:8000/blockstream").await.map_err(|_| rocket::http::Status::InternalServerError)?;
    let (write, read) = ws_stream.split();
    let mut rx = tx.subscribe();
    let write = write.fuse();
    let read = read.fuse();

    tokio::select! {
        _ = async {
            while let Ok(block) = rx.recv().await {
                let message = serde_json::to_string(&block).unwrap();
                write.send(Message::Text(message)).await.unwrap();
            }
        } => {},
        _ = async {
            while let Some(Ok(message)) = read.next().await {
            }
        } => {},
    }

    Ok(())
}

pub async fn start_blockstream() -> Arc<broadcast::Sender<Block>> {
    let (tx, _) = broadcast::channel(100);
    tokio::spawn(async move {
        let mut count = 0;
        loop {
            let block = Block {
                id: format!("block_{}", count),
                data: "Block data".to_string(),
            };
            tx.send(block).unwrap();
            count += 1;
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    });

    Arc::new(tx)
}
