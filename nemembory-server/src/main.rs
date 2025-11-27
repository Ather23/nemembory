use tokio::sync::Mutex;
use clap::Parser;
use nemembory_core::{ ModelProvider, NememboryAgent };
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use futures_util::{ SinkExt, StreamExt };
use anyhow::Result;
use std::sync::Arc;

struct SessionContext {
    agent: NememboryAgent,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    model: String,

    #[arg(short, long)]
    task: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "127.0.0.1:3000".to_string();
    // 1. Bind the TCP listener
    let listener = TcpListener::bind(&addr).await?;
    println!("WebSocket server started on ws://{}", addr);

    let args = Args::parse();

    let model = match args.model.to_lowercase().as_str() {
        "anthropic" => ModelProvider::Anthropic,
        "gemini" => ModelProvider::Gemini,
        other => {
            eprintln!("Invalid model provider: {other}. Use 'anthropic' or 'gemini'.");
            std::process::exit(1);
        }
    };

    let task = args.task;
    let agent = NememboryAgent::new(task, model).default_handlers();
    let session_ctx = Arc::new(
        Mutex::new(SessionContext {
            agent,
        })
    );

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream, session_ctx.clone()));
    }

    Ok(())
}

async fn handle_connection(
    stream: tokio::net::TcpStream,
    context: Arc<Mutex<SessionContext>>
) -> Result<()> {
    let mut ws_stream = accept_async(stream).await?;
    println!(" WebSocket connection established");

    while let Some(msg) = ws_stream.next().await {
        let msg = msg?;
        if msg.is_text() {
            let received_text = msg.to_text()?;
            println!(" Received message: {}", received_text);

            let mut ctx = context.lock().await;
            if let Ok(response) = ctx.agent.run(received_text, 4).await {
                ws_stream.send(Message::Text(response.into())).await?;
            }
        } else if msg.is_close() {
            break; // Exit the loop on close
        }
    }

    println!(" WebSocket connection closed");
    Ok(())
}
