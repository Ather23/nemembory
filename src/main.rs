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

    let model = match args.model.to_lowercase().as_str() {
        "anthropic" => ModelProvider::Anthropic,
        "gemini" => ModelProvider::Gemini,
        other => {
            eprintln!("Invalid model provider: {other}. Use 'anthropic' or 'gemini'.");
            std::process::exit(1);
        }
    };

    let task = "Help me with different questions that I have".to_string();
    let mut chat = NememboryAgent::new(task, model);

    let answer = chat.run(&args.prompt, 10).await?;
    println!("\n\nReasoning Agent: {answer}");

    Ok(())
}
