use tokio::{ net::{ TcpListener, TcpStream }, sync::Mutex };
use tokio_tungstenite::{ accept_async, tungstenite::Message };
use futures::{ StreamExt, SinkExt };
use crate::agent::NememboryAgent;
use std::sync::Arc;

pub async fn start_websocket_server(
    agent: Arc<Mutex<NememboryAgent>>,
    addr: &str
) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(addr).await?;
    println!("WebSocket server listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        let agent = Arc::clone(&agent);
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, agent).await {
                eprintln!("Error handling connection: {}", e);
            }
        });
    }
    Ok(())
}

async fn handle_connection(
    stream: TcpStream,
    agent: Arc<Mutex<NememboryAgent>>
) -> Result<(), Box<dyn std::error::Error>> {
    let ws_stream = accept_async(stream).await?;
    let (mut write, mut read) = ws_stream.split();

    while let Some(msg) = read.next().await {
        let msg = msg?;
        if msg.is_text() {
            let text = msg.to_text()?;
            let mut agent_guard = agent.lock().await;
            let response = agent_guard.run(text, 2).await?;
            let response_json = serde_json::to_string(&response)?;

            write.send(Message::Text(response_json.into())).await?;
        }
    }
    Ok(())
}
