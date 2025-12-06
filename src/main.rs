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
    Ok(())
}
