use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio_tungstenite::{accept_async, tungstenite::Message};
use futures_util::{SinkExt, StreamExt}; // Keep these imports
use serde_json::Value;
use serde::{Serialize};
use serde_json::json;

#[derive(Serialize)]
struct WelcomeMessage {
    message: String,
    timestamp: String,
}

/*
 * Rust clap documentation
 * https://www.shuttle.dev/blog/2023/12/08/clap-rust
 */
use clap::{Parser, Arg};

/// Watch for command output with various options.
#[derive(Parser)]
#[command(name = "watch", about = "A command-line tool for monitoring and interacting with commands")]
struct Args {
    /// Beep if command has a non-zero exit
    #[arg(short = 'b', long = "beep")]
    beep: bool,

    /// Interpret ANSI color and style sequences
    #[arg(short = 'c', long = "color")]
    color: bool,

    /// Highlight changes between updates (optionally specify a permanent flag)
    #[arg(short = 'd', long = "differences", default_value_t = false)]
    differences: bool,

    /// Exit if command has a non-zero exit code
    #[arg(short = 'e', long = "errexit")]
    errexit: bool,

    /// Exit when output from command changes
    #[arg(short = 'g', long = "chgexit")]
    chgexit: bool,

    /// Exit when output from command does not change after specified cycles
    #[arg(short = 'q', long = "equexit", value_name = "cycles")]
    equexit: Option<u64>,

    /// Seconds to wait between updates
    #[arg(short = 'n', long = "interval", value_name = "secs")]
    interval: Option<u64>,

    /// Attempt to run the command in precise intervals
    #[arg(short = 'p', long = "precise")]
    precise: bool,

    /// Turn off header/title
    #[arg(short = 't', long = "no-title")]
    no_title: bool,

    /// Turn off line wrapping
    #[arg(short = 'w', long = "no-wrap")]
    no_wrap: bool,

    /// Pass command to exec instead of "sh -c"
    #[arg(short = 'x', long = "exec")]
    exec: bool,

    /// The command to run
    #[arg(required = true)]
    command: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    println!("argument to run {}", args.command);

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

        // Create a WelcomeMessage struct
        let welcome_message = WelcomeMessage {
            message: "Welcome to the WebSocket server!".to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(), // Example timestamp
        };

        // Serialize the struct to a JSON string
        let json_message = serde_json::to_string(&welcome_message).unwrap();

        // Send the serialized JSON message as a WebSocket message
        writer.send(Message::Text(json_message)).await.unwrap();

        // Handle incoming messages
        while let Some(message) = reader.next().await {
            match message {
                Ok(Message::Text(text)) => {
                    // Parse the received JSON message
                    match serde_json::from_str::<Value>(&text) {
                        Ok(json_message) => {
                            println!("Received message: {:?}", json_message);
                        }
                        Err(err) => {
                            println!("Error parsing JSON: {}", err);
                        }
                    }

                    // Respond back to the client
                    let response = "Hello from Rust WebSocket server!";
                    writer.send(Message::Text(response.to_string())).await.unwrap();
                }
                Ok(Message::Close(_)) => break,
                _ => {}
            }
        }
    }
}
