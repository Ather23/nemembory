use nemembory_core::{ ModelProvider, NememboryAgent };

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define the task for the agent
    let task =
        "You are a helpful assistant that can answer questions and perform tasks.".to_string();

    // Create a new NememboryAgent with Anthropic as the model provider
    let mut agent = NememboryAgent::new(
        task,
        ModelProvider::OpenRouter("anthropic/claude-haiku-4.5".to_string())
    ).default_handlers();

    // Run the agent with a sample prompt
    let prompt = "What is the current date?";
    println!("Running agent with prompt: {}", prompt);

    match agent.run(prompt, 5).await {
        Ok(response) => {
            println!("\n--- Agent Response ---");
            println!("{}", response);
        }
        Err(e) => {
            eprintln!("Error running agent: {}", e);
        }
    }

    Ok(())
}
