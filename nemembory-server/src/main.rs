use axum::{
    Router,
    extract::{ Path, ws::{ Message, WebSocket, WebSocketUpgrade } },
    response::IntoResponse,
    routing::get,
};
use nemembory_core::NememboryAgent;
use tokio::net::TcpListener;

/*
struct Agent {
    name: String,
}
*/

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/healtcheck", get(health_check))
        .route("/ws/{agent_id}", get(ws_handler));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
    "Healthy"
}

async fn ws_handler(ws: WebSocketUpgrade, Path(agent_id): Path<String>) -> impl IntoResponse {
    // Pass the ID to the socket handler
    ws.on_upgrade(move |socket| handle_socket(socket, agent_id))
}
/*
async fn activate_remote_agent() -> impl IntoResponse {
    let agent = Agent { name: "test_agent".to_string() };
    agent
}
*/

fn get_agent(agent_name: &str) -> NememboryAgent {
    let task = "Help me with different questions that I have".to_string();
    let agent = NememboryAgent::new(task, nemembory_core::ModelProvider::Anthropic);
    return agent;
}

async fn handle_socket(mut socket: WebSocket, agent_id: String) {
    let mut agent = get_agent(&agent_id);
    while let Some(Ok(msg)) = socket.recv().await {
        match msg {
            Message::Text(text) => {
                println!("Received: {}", text);
                let result = agent.run(text.as_str(), 4).await;
                if let Err(error) = result {
                    eprintln!("Error generating response: {}", error.to_string());
                } else {
                    if socket.send(Message::Text(result.unwrap().into())).await.is_err() {
                        // Client disconnected
                        break;
                    }
                }
            }
            Message::Close(_) => {
                println!("Client disconnected");
                break;
            }
            _ => {
                // Ignore other message types (Binary, Ping, Pong)
            }
        }
    }
}
