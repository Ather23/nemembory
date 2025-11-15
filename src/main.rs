use clap::{ Parser };
use nemembory_core::{ ModelProvider, agent::agent::NememboryAgent };

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    prompt: String,

    #[arg(short, long)]
    model: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    if args.model != "anthropic" && args.model != "gemini" {
        panic!("Invalid model provider. Use 'Anthropic' or 'Gemini'.");
    }

    let mut chat = NememboryAgent::new(
        if args.model.to_lowercase() == "anthropic" {
            ModelProvider::Anthropic
        } else {
            ModelProvider::Gemini
        }
    );

    let answer = chat.run(&args.prompt, 10).await?;
    println!("\n\nReasoning Agent: {answer}");

    Ok(())
}
