mod config;
mod listener;
mod trader;
mod types;
use crate::types::{BotEvent, TokenEvent};

use crate::{config::Config, listener::Listener, trader::Trader};
use ethers::providers::{Provider, Ws};
use std::sync::Arc;
use tokio::{io::{self, AsyncBufReadExt, BufReader}, sync::mpsc, task};

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    // Create channel to communicate user commands
    let (cmd_tx, mut cmd_rx) = mpsc::unbounded_channel::<String>();

    let cfg = Config::from_env();
    let (tx, rx) = mpsc::channel(100);

    let ws = Ws::connect(cfg.wss_url.clone()).await?;
    println!("✅ WS connected!");
    let provider = Arc::new(Provider::new(ws));
    // Create listener
    let listener = Listener::new(provider.clone(), tx);

    // Await trader creation
    let mut trader: Trader = Trader::new(rx).await?; // <-- await and unwrap Result

    // Spawn listener task
    let listener_task = task::spawn(async move {
        if let Err(e) = listener.run().await {
            eprintln!("[Listener task error] {:?}", e);
        }
    });

    // Spawn trader task
    let trader_task = task::spawn(async move {
        if let Err(e) = trader.run().await {
            eprintln!("Trader task error: {:?}", e);
        }
    });
    
    // Task 2: Command listener (reads from stdin)
    let cmd_tx_clone = cmd_tx.clone();
    let stdin_task: task::JoinHandle<()> = task::spawn(async move {
        let mut reader = BufReader::new(io::stdin());
        let mut line = String::new();
        loop {
            line.clear();
            if reader.read_line(&mut line).await.unwrap() == 0 {
                break; // EOF
            }
            let input = line.trim().to_string();
            if !input.is_empty() {
                if cmd_tx_clone.send(input).is_err() {
                    println!("Command channel closed");
                    break;
                }
            }
        }
    });

    // Task 3: Handle user commands
    let command_handler = task::spawn(async move {
        while let Some(cmd) = cmd_rx.recv().await {
            match cmd.as_str() {
                "status" => println!("✅ Program is running fine."),
                "pause" => println!("⏸️ Pausing operations..."),
                "resume" => println!("▶️ Resuming operations..."),
                "exit" => {
                    println!("👋 Exiting program...");
                    std::process::exit(0);
                }
                _ => println!("❓ Unknown command: {}", cmd),
            }
        }
    });
    tokio::try_join!(listener_task, trader_task, stdin_task, command_handler)?;
    Ok(())
}
