use futures::{ SinkExt, StreamExt };
use tokio_tungstenite::{ connect_async, tungstenite::Message };
use tokio::{ io::{ AsyncBufReadExt, BufReader } };

#[tokio::main]
async fn main() {
    let url = "ws://127.0.0.1:8080";

    match connect_async(url).await {
        Ok((ws_stream, _)) => {
            println!("Connected to {}", url);
            println!("Type your messages below (Ctrl+C to exit):\n");

            let (mut write, mut read) = ws_stream.split();

            // Spawn a task to read from WebSocket
            let read_task = tokio::spawn(async move {
                while let Some(msg) = read.next().await {
                    match msg {
                        Ok(Message::Text(text)) => {
                            println!("[Server]: {}", text);
                        }
                        Ok(Message::Close(_)) => {
                            println!("\nConnection closed by server");
                            break;
                        }
                        Ok(_) => {
                            // Handle other message types if needed
                        }
                        Err(e) => {
                            eprintln!("Error receiving message: {}", e);
                            break;
                        }
                    }
                }
            });

            // Read from stdin in the main task
            let reader = BufReader::new(tokio::io::stdin());
            let mut lines = reader.lines();

            while let Ok(Some(line)) = lines.next_line().await {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }

                if trimmed.eq_ignore_ascii_case("exit") || trimmed.eq_ignore_ascii_case("quit") {
                    println!("Closing connection...");
                    break;
                }

                match write.send(Message::Text(trimmed.to_string().into())).await {
                    Ok(_) => {
                        println!("[You]: {}", trimmed);
                    }
                    Err(e) => {
                        eprintln!("Failed to send message: {}", e);
                        break;
                    }
                }
            }

            let _ = read_task.await;
        }
        Err(e) => {
            eprintln!("Failed to connect to {}: {}", url, e);
        }
    }
}
