use futures::StreamExt;
use nemembory_core::{ ModelProvider, NememboryAgent };
use std::io::{ self, Write };

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define the task for the agent
    let task = "You are a helpful assistant that can answer questions.".to_string();

    // Create a new NememboryAgent with Anthropic as the model provider
    let agent = NememboryAgent::new(
        "stream_agent",
        task,
        ModelProvider::OpenRouter("anthropic/claude-haiku-4.5".to_string())
    );

    // Run the agent with streaming enabled
    let prompt = "Write a short poem about Rust programming language";
    println!("Running agent with prompt: {}\n", prompt);
    println!("--- Streaming Response ---");

    // Get the stream and pin it
    let stream = agent.run_stream(prompt, 5);
    tokio::pin!(stream);

    // Process each chunk as it arrives
    while let Some(result) = stream.next().await {
        match result {
            Ok(text) => {
                // Print each chunk immediately without newline
                print!("{}", text);
                // Flush stdout to ensure immediate display
                io::stdout().flush()?;
            }
            Err(e) => {
                eprintln!("\nError during streaming: {}", e);
            }
        }
    }

    println!("\n\n--- Stream Complete ---");

    Ok(())
}
