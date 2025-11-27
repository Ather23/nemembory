use std::sync::Arc;

use clap::{ Parser };
use nemembory_core::{ ModelProvider, agent::{ agent::NememboryAgent } };
use tokio::sync::Mutex;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    model: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let model = match args.model.to_lowercase().as_str() {
        "anthropic" => ModelProvider::Anthropic,
        "gemini" => ModelProvider::Gemini,
        other => {
            eprintln!("Invalid model provider: {other}. Use 'anthropic' or 'gemini'.");
            std::process::exit(1);
        }
    };

    let task = "Help me with different questions that I have".to_string();
    let agent = NememboryAgent::new(task, model);
    let agent = Arc::new(Mutex::new(agent));

    nemembory_core::agent::connections
        ::start_websocket_server(agent, "127.0.0.1:8080").await
        .expect("Failed to start WebSocket server");
    Ok(())
}
