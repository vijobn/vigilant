use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio_tungstenite::{accept_async, tungstenite::Message};
use futures_util::{SinkExt, StreamExt}; // Keep these imports
use serde_json::Value;
use serde::{Serialize};
use serde_json::json;
use std::sync::{Arc, Mutex};

#[derive(Serialize)]
struct WelcomeMessage {
    message: String,
    timestamp: String,
}

#[derive(Serialize)]
struct SetTitle {
    command: String,
    //left: String,
    center: String,
    //right: String,
}

#[derive(Serialize)]
struct SetHeaders {
    command: String,
    headers: Vec<String>,
}

#[derive(Serialize)]
struct SetDataRow {
    command: String,
    index: i32,
    value: String,
    name: String,
    country: String,
    age: i32,
}

use serde::{Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct GConf {
    cmdline: Vec<String>,  // Command line arguments
}

mod cmd;

impl GConf {
    // Constructor to create a new GConf instance
    fn new() -> Self {
        GConf {
            cmdline: Vec::new(),  // Initially, no command line arguments
        }
    }

    // Method to set cmdline as a string
    fn set_cmdline(&mut self, cmdline: &str) {
        self.cmdline = cmdline.split_whitespace().map(String::from).collect();
    }

    // Method to add a single argument to cmdline
    fn add_to_cmdline(&mut self, arg: &str) {
        self.cmdline.push(arg.to_string());
    }

    // Method to get the cmdline as a string (for debugging or other purposes)
    fn get_cmdline(&self) -> String {
        self.cmdline.join(" ")
    }
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
    // Create a new configuration
    let gconf = Arc::new(Mutex::new(GConf::new()));

    // Set the cmdline with a string
    gconf.lock().unwrap().set_cmdline(&args.command.to_string());

    tauri::Builder::default()
        .setup(move |_app| {
            let gconf_clone = Arc::clone(&gconf);

            // Spawn the WebSocket server with access to the shared configuration
            tokio::spawn(async move {
                start_websocket_server(gconf_clone).await;
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
}

use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::tungstenite::error::Error; // Correct error import
use futures_util::stream::SplitSink;

async fn send_json_message(
    //writer: &mut WebSocketStream<tokio::net::TcpStream>,
    writer: &mut SplitSink<WebSocketStream<tokio::net::TcpStream>, Message>,
    rdata: SetDataRow,
) -> Result<(), Error> {
    let json_message = serde_json::to_string(&rdata).unwrap();
    writer.send(Message::Text(json_message.clone())).await?;
    println!("Sent message {}", json_message);
    Ok(())
}

async fn start_websocket_server(gconf: Arc<Mutex<GConf>>) {
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


        // Create a SetTitle struct
        let mut title_message = SetTitle {
            command: "SetTitle".to_string(),
            center: "Center".to_string(),
        };
        let cmdline = gconf.lock().unwrap().get_cmdline();
        title_message.center = cmdline.clone();

        println!("Updated cmdline: {}", cmdline);
        // Serialize the struct to a JSON string
        let json_msg2 = serde_json::to_string(&title_message).unwrap();

        // Send the serialized JSON message as a WebSocket message
        writer.send(Message::Text(json_msg2)).await.unwrap();


        // Create a SetHeaders struct
        let hdr_message = SetHeaders {
            command: "SetHeaders".to_string(),
            headers: vec!["RustName".to_string(), "RustAge".to_string(), "RustCountry".to_string()],
        };

        // Serialize the struct to a JSON string
        let json_msg3 = serde_json::to_string(&hdr_message).unwrap();

        // Send the serialized JSON message as a WebSocket message
        writer.send(Message::Text(json_msg3)).await.unwrap();



        let mut colines = cmd::CmdOutput::new(&cmdline.clone());

        while let Some(ref line) = colines.as_mut().expect("Reee").next() {
            println!("Cmd Output: {:?}", line);
        }

        // Send data rows
        let rows = vec![
            SetDataRow {
                command: "SetDataRow".to_string(),
                index: 0,
                value: "Rusty Rust".to_string(),
                name: "Rusty Lane".to_string(),
                country: "USA".to_string(),
                age: 40,
            },
            SetDataRow {
                command: "SetDataRow".to_string(),
                index: 1,
                value: "John Doe".to_string(),
                name: "John Doe".to_string(),
                country: "USA".to_string(),
                age: 25,
            },
            SetDataRow {
                command: "SetDataRow".to_string(),
                index: 2,
                value: "Jane Smith".to_string(),
                name: "Jane Smith".to_string(),
                country: "Canada".to_string(),
                age: 30,
            },
            SetDataRow {
                command: "SetDataRow".to_string(),
                index: 3,
                value: "Rusty  Rust".to_string(),
                name: "Sam Johnson".to_string(),
                country: "UK".to_string(),
                age: 22,
            },
            SetDataRow {
                command: "SetDataRow".to_string(),
                index: 4,
                value: "Rusty  Rust".to_string(),
                name: "Aby Thomas".to_string(),
                country: "India".to_string(),
                age: 36,
            },
        ];

        for row in rows {
            if let Err(e) = send_json_message(&mut writer, row).await {
                eprintln!("Failed to send data row: {}", e);
            }
        }

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
