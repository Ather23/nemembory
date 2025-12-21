use futures::{ StreamExt, stream };
use nemembory_core::{ ModelProvider, NememboryAgent };
use std::io::{ self, Write };

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define the task for the agent
    let task =
        "You are a helpful assistant that can answer questions and perform tasks.".to_string();

    // Create a new NememboryAgent with Anthropic as the model provider
    let mut agent = NememboryAgent::new(
        "cli_agent",
        task,
        ModelProvider::OpenRouter("anthropic/claude-haiku-4.5".to_string())
    )
        .create_working_directory("D:\\agents")
        .default_handlers()
        .default_hooks();

    // Run the agent with a sample prompt that triggers a tool call
    let prompt = "What is the weather today?";
    println!("Running agent with prompt: {}", prompt);
    let stream = agent.run_stream(prompt, 5);
    tokio::pin!(stream);

    while let Some(result) = stream.next().await {
        match result {
            Ok(text) => {
                println!("\n--- Agent Response ---");
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
    // match agent.run(prompt, 5).await {
    //     Ok(response) => {
    //         println!("\n--- Agent Response ---");
    //         println!("{}", response);
    //     }
    //     Err(e) => {
    //         eprintln!("Error running agent: {}", e);
    //     }
    // }

    Ok(())
}
