use axum::{
    Router,
    extract::{ State, WebSocketUpgrade },
    response::IntoResponse,
    routing::{ get, post },
    Json,
};
use tokio::sync::Mutex;
use clap::Parser;
use nemembory_core::{ ModelProvider, NememboryAgent };
use tokio::net::TcpListener;
use futures_util::{ SinkExt, StreamExt };
use anyhow::Result;
use serde::Deserialize;
use std::sync::Arc;

type AppState = Arc<Mutex<SessionContext>>;

struct SessionContext {
    agent: NememboryAgent,
    task: String,
    model: ModelProvider,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    model: String,

    #[arg(short, long)]
    task: String,
}

#[derive(Deserialize)]
struct UpdateSessionRequest {
    task: Option<String>,
    model: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
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

    let agent = NememboryAgent::new(task.clone(), model.clone()).default_handlers();

    let session_ctx: AppState = Arc::new(
        Mutex::new(SessionContext {
            agent,
            task,
            model,
        })
    );

    let app = Router::new()
        .route("/ws", get(ws_handler))
        .route("/session", post(update_session))
        .route("/session", get(get_session))
        .with_state(session_ctx.clone());

    let addr = "127.0.0.1:3000";
    let listener = TcpListener::bind(addr).await?;
    println!("Server started on http://{}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}

async fn ws_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_websocket(socket, state))
}

async fn handle_websocket(socket: axum::extract::ws::WebSocket, context: AppState) {
    let (mut sender, mut receiver) = socket.split();

    while let Some(Ok(msg)) = receiver.next().await {
        if let axum::extract::ws::Message::Text(text) = msg {
            println!("Received message: {}", text);

            let mut ctx = context.lock().await;
            if let Ok(response) = ctx.agent.run(&text, 4).await {
                let _ = sender.send(axum::extract::ws::Message::Text(response.into())).await;
            }
        }
    }

    println!("WebSocket connection closed");
}

async fn update_session(
    State(state): State<AppState>,
    Json(payload): Json<UpdateSessionRequest>
) -> Json<serde_json::Value> {
    let mut ctx = state.lock().await;

    if let Some(model_str) = &payload.model {
        let new_model = match model_str.to_lowercase().as_str() {
            "anthropic" => Some(ModelProvider::Anthropic),
            "gemini" => Some(ModelProvider::Gemini),
            _ => None,
        };
        if let Some(model) = new_model {
            ctx.model = model;
        }
    }

    if let Some(task) = payload.task {
        ctx.task = task;
    }

    ctx.agent = NememboryAgent::new(ctx.task.clone(), ctx.model.clone()).default_handlers();

    Json(
        serde_json::json!({
        "status": "ok",
        "task": ctx.task,
        "model": format!("{:?}", ctx.model)
    })
    )
}

async fn get_session(State(state): State<AppState>) -> Json<serde_json::Value> {
    let ctx = state.lock().await;
    Json(
        serde_json::json!({
        "task": ctx.task,
        "model": format!("{:?}", ctx.model)
    })
    )
}
