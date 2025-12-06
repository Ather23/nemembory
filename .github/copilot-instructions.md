# Nemembory - AI Coding Agent Instructions

## Architecture Overview

Nemembory is a Rust workspace with two crates:

- **`nemembory-core`**: Core library containing agent logic, tools, handlers, and hooks
- **`nemembory-server`**: Axum-based WebSocket server exposing the agent via HTTP

```
User → WebSocket (3000) → NememboryAgent → rig::Agent (Anthropic/Gemini) → Tools
                                ↓
                          MessageHandlers (FileHandler → JSON log)
                                ↓
                          PromptHooks (log_tool_call, write_to_file)
```

## Key Components

### Agent System (`nemembory-core/src/agent/`)

- **`NememboryAgent`**: Main orchestrator wrapping `rig::Agent` with handlers/hooks
- **`RunnableAgent` trait**: Abstraction over LLM providers (Anthropic, Gemini)
- **`HandleAgentResponse`**: Implements `rig::PromptHook` for intercepting tool calls and completions

### Tool Implementation Pattern (`nemembory-core/src/tools/`)

Tools implement `rig::tool::Tool`. Follow this pattern:

```rust
impl Tool for YourTool {
    const NAME: &'static str = "tool_name";
    type Error = YourError;    // Must implement thiserror::Error
    type Args = YourArgs;      // Must derive Deserialize, Serialize
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition { ... }
    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> { ... }
}
```

Available tools: `RestApiTool`, `WebSearch`, `ShellTool`, `LinkToMarkdown`, `GetDate`

### Handler Pattern (`nemembory-core/src/handlers/`)

Handlers implement `MessageHandler` trait for side-effects on message events:

```rust
#[async_trait]
impl MessageHandler for YourHandler {
    async fn handle_message(&self, message: Message) -> Result<(), std::io::Error>;
}
```

### Hooks (`nemembory-core/src/hooks/`)

Callbacks registered via `HandleAgentResponse::add_callback()` receive `HashMap<String, String>` with event data.

## Build & Run Commands

```powershell
# Start PostgreSQL (required for data module)
docker compose up -d

# Build entire workspace
cargo build

# Run the WebSocket server
cargo run -p nemembory-server -- --model anthropic --task "Your task description"

# Run WebSocket test client
cargo run --example ws_client

# Run database example
cargo run --example db
```

## Environment Variables

- `ANTHROPIC_API_KEY` - Required for Anthropic/Claude models
- `GEMINI_API_KEY` - Required for Gemini models
- Database: `postgres://myuser:mypassword@localhost:5432/nemembory` (from docker-compose)

## Code Conventions

### Error Handling

- Use `thiserror` for custom error types (see `RestApiError`, `DbError`, `AgentHookError`)
- Tools return `Result<Output, CustomError>` where error implements `thiserror::Error`

### Async Patterns

- All I/O operations use `tokio` runtime with `#[tokio::main]`
- Use `async_trait` macro for async trait methods
- WebSocket handlers use `futures_util::{SinkExt, StreamExt}`

### Serialization

- All data structures derive `Serialize, Deserialize` from `serde`
- Database models also derive `sqlx::FromRow`

### Module Structure

Each module has a `mod.rs` that re-exports public items:

```rust
pub mod feature;
pub use feature::FeatureType;
```

## Database Schema

Table `agents` in PostgreSQL:

- `id: SERIAL PRIMARY KEY`
- `code: VARCHAR(100) UNIQUE`
- `display_name: VARCHAR(100)`
- `system_prompt: TEXT`

Use `AgentPersistence` trait and `DbAgentStore` for database operations.

## Server Endpoints

- `GET /ws` - WebSocket connection for agent interaction
- `POST /session` - Update task/model (`{ "task": "...", "model": "anthropic|gemini" }`)
- `GET /session` - Get current session state

## Dependencies to Know

- **`rig-core`**: LLM agent framework (git dependency from 0xPlaygrounds/rig)
- **`brave-rs`**: Brave Search API client (git dependency)
- **`sqlx`**: Async PostgreSQL driver with compile-time query checking
- **`axum`**: Web framework for the server
