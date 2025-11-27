use futures::{ SinkExt, StreamExt };
use tokio::io::{ AsyncBufReadExt, BufReader };
use tokio_tungstenite::{ connect_async, tungstenite::protocol::Message };

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Connect to the nemembory-server WebSocket endpoint
    let connect_addr = "ws://127.0.0.1:3000/ws";

    println!("Connecting to {}", connect_addr);

    let (ws_stream, _) = connect_async(connect_addr).await.expect("Failed to connect");
    println!("WebSocket handshake has been successfully completed");

    let (mut write, mut read) = ws_stream.split();

    // Spawn a task to read messages
    let mut read_handle = tokio::spawn(async move {
        while let Some(message) = read.next().await {
            match message {
                Ok(msg) => {
                    if msg.is_text() || msg.is_binary() {
                        println!("Received: {}", msg);
                    }
                }
                Err(e) => {
                    eprintln!("Error receiving message: {}", e);
                    break;
                }
            }
        }
    });

    let mut stdin = BufReader::new(tokio::io::stdin());
    let mut line = String::new();

    println!("Type a message and press Enter to send. Press Ctrl+C to exit.");

    loop {
        tokio::select! {
            _ = &mut read_handle => {
                println!("Connection closed by server");
                break;
            }
            _ = tokio::signal::ctrl_c() => {
                println!("Shutting down...");
                break;
            }
            res = stdin.read_line(&mut line) => {
                match res {
                    Ok(0) => break, // EOF
                    Ok(_) => {
                        let msg = line.trim();
                        if !msg.is_empty() {
                             if let Err(e) = write.send(Message::Text(msg.to_string().into())).await {
                                 eprintln!("Error sending message: {}", e);
                                 break;
                             }
                        }
                        line.clear();
                    }
                    Err(e) => {
                        eprintln!("Error reading stdin: {}", e);
                        break;
                    }
                }
            }
        }
    }

    Ok(())
}
