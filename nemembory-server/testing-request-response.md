# Testing Request Response

## Get Session

Retrieve the current session configuration:

```powershell
Invoke-RestMethod -Uri "http://127.0.0.1:3000/session" -Method Get
```

Example response:

```
model     task
-----     ----
Anthropic New task description
```

## Update Session

Update the session with a new task and/or model:

```powershell
Invoke-RestMethod -Uri "http://127.0.0.1:3000/session" -Method Post -ContentType "application/json" -Body '{"task": "You are a helpful coding assistant. Help users write, debug, and explain code in various programming languages.", "model": "anthropic"}'
```

Example response:

```
model     status task
-----     ------ ----
Anthropic ok     You are a helpful coding assistant...
```

### Real-World Task Examples

```powershell
# Research Assistant
Invoke-RestMethod -Uri "http://127.0.0.1:3000/session" -Method Post -ContentType "application/json" -Body '{"task": "You are a research assistant. Help users find, summarize, and analyze information from various sources. Provide accurate citations and highlight key findings.", "model": "gemini"}'

# Customer Support Agent
Invoke-RestMethod -Uri "http://127.0.0.1:3000/session" -Method Post -ContentType "application/json" -Body '{"task": "You are a customer support agent for a software company. Help users troubleshoot issues, explain product features, and guide them through common workflows.", "model": "anthropic"}'

# Writing Coach
Invoke-RestMethod -Uri "http://127.0.0.1:3000/session" -Method Post -ContentType "application/json" -Body '{"task": "You are a writing coach. Help users improve their writing by providing feedback on grammar, style, clarity, and structure. Suggest improvements while maintaining their voice.", "model": "gemini"}'

# Data Analyst
Invoke-RestMethod -Uri "http://127.0.0.1:3000/session" -Method Post -ContentType "application/json" -Body '{"task": "You are a data analyst assistant. Help users interpret data, create SQL queries, suggest visualizations, and explain statistical concepts in simple terms.", "model": "anthropic"}'
```

### Available Models

- `anthropic`
- `gemini`

### Request Body Parameters

| Parameter | Type   | Required | Description                        |
| --------- | ------ | -------- | ---------------------------------- |
| task      | string | No       | The task description for the agent |
| model     | string | No       | The model provider to use          |
