use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio_tungstenite::{accept_async, tungstenite::Message};
use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use serde_json::json;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};
use std::thread;
use std::process;
use tokio::time::Duration;
use tauri::Env;

#[derive(Serialize)]
struct WelcomeMessage {
    message: String,
    timestamp: String,
}

#[derive(Serialize, Debug)]
struct SetTitle {
    command: String,
    center: String,
}

#[derive(Serialize)]
struct SetHeaders {
    command: String,
    headers: Vec<String>,
}

#[derive(Serialize, Debug)]
struct SetDataRow {
    command: String,
    index: i32,
    values: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct GConf {
    cmdline: Vec<String>,  // Command line arguments
}

mod cmd;

impl GConf {
    fn new() -> Self {
        GConf {
            cmdline: Vec::new(),
        }
    }

    fn set_cmdline(&mut self, cmdline: &str) {
        self.cmdline = cmdline.split_whitespace().map(String::from).collect();
    }

    fn add_to_cmdline(&mut self, arg: &str) {
        self.cmdline.push(arg.to_string());
    }

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
    #[arg(default_value = "/usr/bin/lsattr")]
    command: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let gconf = Arc::new(Mutex::new(GConf::new()));

    println!("arguments to run {}", args.command);
    gconf.lock().unwrap().set_cmdline(&args.command.to_string());
    tauri::Builder::default()
        .setup(move |_app| {
            let gconf_clone = Arc::clone(&gconf);
            tokio::spawn(async move {
                start_websocket_server(gconf_clone).await;
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
}

use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::tungstenite::error::Error;
use futures_util::stream::SplitSink;

pub async fn send_json_message<T>(
    writer: &mut SplitSink<WebSocketStream<tokio::net::TcpStream>, Message>,
    rdata: T,
) -> Result<(), Box<dyn std::error::Error>>
where
    T: Serialize,
{
    // Serialize the data to JSON
    let json_message = serde_json::to_string(&rdata).map_err(|e| {
        eprintln!("Serialization error: {}", e);
        e
    })?;

    // Send the message
    writer.send(Message::Text(json_message.clone())).await.map_err(|e| {
        eprintln!("Error sending message: {}", e);
        e
    })?;

    println!("Sent message: {}", json_message);
    Ok(())
}

async fn start_websocket_server(gconf: Arc<Mutex<GConf>>) {
    let addr = "127.0.0.1:8080".parse::<SocketAddr>().unwrap();
    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("WebSocket server listening on {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        let ws_stream = accept_async(stream).await.unwrap();
        println!("New WebSocket connection");

        let (mut writer, mut reader) = ws_stream.split();

        // Sending the welcome message
        let welcome_message = WelcomeMessage {
            message: "Welcome to the WebSocket server!".to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        };

        let json_message = serde_json::to_string(&welcome_message).unwrap();
        //writer.send(Message::Text(json_message)).await.unwrap();
        if let Err(e) = writer.send(Message::Text(json_message)).await {
            // Handle the error (e.g., logging)
            eprintln!("Error sending message: {}", e);

            process::exit(1);
        }

        // Sending the title message
        let mut title_message = SetTitle {
            command: "SetTitle".to_string(),
            center: "Center".to_string(),
        };

        let cmdline = gconf.lock().unwrap().get_cmdline();
        title_message.center = cmdline.clone();
        println!("Cmdline: {}", cmdline);
        println!("title msg : {:?}", title_message);

        if let Err(e) = send_json_message(&mut writer, title_message).await {
            eprintln!("Failed to send title message: {}", e);
        }

        // Sending headers message
        let hdr_message = SetHeaders {
            command: "SetHeaders".to_string(),
            headers: vec!["name".to_string(), "age".to_string(), "country".to_string()],
        };

        if let Err(e) = send_json_message(&mut writer, hdr_message).await {
            eprintln!("Failed to send headers: {}", e);
        }

        // Sending data rows
        let mut colines = cmd::CmdOutput::new(&cmdline.clone());
        let mut idx = 0;
        while let Some(ref line) = colines.as_mut().expect("Reee").next() {
            let r = SetDataRow {
                command: "SetDataRow".to_string(),
                index: idx,
                values: vec![line.to_string(), 40.to_string(), "USA".to_string()],
            };
            if let Err(e) = send_json_message(&mut writer, r).await {
                eprintln!("Failed to send data row: {}", e);
            }
            idx += 1;
        }

        loop {
            println!("10 seconds have passed");
            thread::sleep(Duration::from_secs(10));
            let mut oplines: Vec<String>;
            match colines.as_mut().expect("cmd bad").clone().execute(&cmdline.clone()) {
                Ok(oplines) => {
                    //println!("{} Outputs {:?}", cmdline.clone(), oplines);
                    println!("No of output lines {}", oplines.len());
                    let changed = colines.as_mut().expect("cmd bad 2").update_lines(oplines.clone()).expect("update bad");
                    for idx in 0..changed.len() {
                        let oline = colines.as_ref().expect("cmd bad 3").clone().get_output_line(changed[idx]).expect("bad outputline");
                        let r = SetDataRow {
                            command: "SetDataRow".to_string(),
                            index: idx as i32,
                            values: vec![oline.to_string(), 40.to_string(), "USA".to_string()],
                        };
                        if let Err(e) = send_json_message(&mut writer, r).await {
                            eprintln!("Failed to send data row: {}", e);
                        }
                    }
                },
                Err(e) => println!("Error executing command {:?}", e),
            }
        }

        // Reading and handling incoming messages
        while let Some(message) = reader.next().await {
            match message {
                Ok(Message::Text(text)) => {
                    match serde_json::from_str::<Value>(&text) {
                        Ok(json_message) => {
                            println!("Received message: {:?}", json_message);
                        }
                        Err(err) => {
                            println!("Error parsing JSON: {}", err);
                        }
                    }

                    let response = "Hello from Rust WebSocket server!";
                    if let Err(e) = writer.send(Message::Text(response.to_string())).await {
                        eprintln!("Failed to send response: {}", e);
                    }
                }
                Ok(Message::Close(_)) => break,
                _ => {}
            }
        }
    }
}