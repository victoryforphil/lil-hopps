use futures::{SinkExt, StreamExt};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tokio::sync::{broadcast, mpsc};
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};

pub async fn websocket_server_task(
    tcp_tx: broadcast::Sender<Vec<u8>>,
    ws_tx: mpsc::Sender<String>,
) {
    let listener = TcpListener::bind("0.0.0.0:3030").await.unwrap();
    println!("WebSocket server listening on ws://localhost:3030");

    let clients = Arc::new(Mutex::new(Vec::new()));

    // Accept WebSocket connections
    while let Ok((stream, _)) = listener.accept().await {
        let ws_tx = ws_tx.clone();
        let clients = clients.clone();
        let tcp_tx = tcp_tx.clone();

        tokio::spawn(async move {
            if let Ok(ws_stream) = accept_async(stream).await {
                let (ws_sender, mut ws_receiver) = ws_stream.split();
                let ws_sender = Arc::new(Mutex::new(ws_sender));
                let mut client_rx = tcp_tx.subscribe();

                clients.lock().await.push(ws_sender.clone());

                let ws_tx_clone = ws_tx.clone();
                tokio::spawn(async move {
                    while let Some(msg) = ws_receiver.next().await {
                        if let Ok(msg) = msg {
                            if let Ok(text) = msg.into_text() {
                                if ws_tx_clone.send(text).await.is_err() {
                                    break;
                                }
                            }
                        }
                    }
                });

                // Handle incoming TCP messages and forward them to WebSocket clients
                loop {
                    thread::sleep(Duration::from_millis(500));
                    if let Ok(msg) = client_rx.recv().await {
                        let mut client_guard = clients.lock().await;
                        let mut i = 0;

                        while i < client_guard.len() {
                            let client = client_guard[i].clone();
                            let mut client = client.lock().await;
                            if (client.send(Message::binary(msg.clone())).await).is_err(){
                                // Remove client if send fails (indicating closed connection)
                                println!("Removing dead client {0}", i);
                                client_guard.remove(i);
                            } else {
                                i += 1;
                            }
                        }
                    }
                }
            }
        });
    }
}
