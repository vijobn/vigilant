use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio_tungstenite::{accept_async, tungstenite::Message};
use futures_util::{SinkExt, StreamExt}; // Keep these imports

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .setup(|_app| {
            tokio::spawn(start_websocket_server());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn start_websocket_server() {
    let addr = "127.0.0.1:8080".parse::<SocketAddr>().unwrap();
    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("WebSocket server listening on {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        let ws_stream = accept_async(stream).await.unwrap();
        println!("New WebSocket connection");

        // Split the WebSocket stream into a sender and receiver
        let (mut writer, mut reader) = ws_stream.split();

        // Send a welcome message to the client
        let welcome_message = "Welcome to the WebSocket server!";
        writer.send(Message::Text(welcome_message.to_string())).await.unwrap();

        // Handle incoming messages
        while let Some(message) = reader.next().await {
            match message {
                Ok(Message::Text(text)) => {
                    println!("Received: {}", text);
                    // Echo the received message back
                    writer.send(Message::Text(format!("Echo: {}", text))).await.unwrap();
                }
                Err(e) => {
                    eprintln!("Error processing message: {}", e);
                    break;
                }
                _ => {}
            }
        }
    }
}
