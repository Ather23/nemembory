# Nemembory Server

A WebSocket-based server that wraps the `NememboryAgent` core logic.

## Architecture

The server listens for WebSocket connections on `127.0.0.1:3000`. It maintains a shared `SessionContext` containing the agent instance, which processes incoming text messages using a configured LLM provider (Anthropic or Gemini).

```mermaid
sequenceDiagram
    participant Client as WebSocket Client
    participant Server as WebSocket Server (main.rs)
    participant Context as SessionContext (Mutex)
    participant Agent as NememboryAgent (Core)
    participant LLM as Model Provider (Anthropic/Gemini)

    Note over Server: Server starts on 127.0.0.1:3000
    Note over Server: Initializes Agent with Task & Model

    Client->>Server: Connect (Handshake)
    Server-->>Client: Connection Established

    loop Message Loop
        Client->>Server: Send Text Message (Input)
        Server->>Context: Lock Context
        Context->>Agent: agent.run(input, max_steps=4)

        activate Agent
        Agent->>LLM: Query Model
        LLM-->>Agent: Response / Action
        Agent-->>Context: Final Response
        deactivate Agent

        Context-->>Server: Unlock
        Server-->>Client: Send Text Message (Response)
    end

    Client->>Server: Close Connection
```

## Usage

Run the server by specifying the model provider and the initial task:

```bash
cargo run -- --model <anthropic|gemini> --task "Your task description"
```
