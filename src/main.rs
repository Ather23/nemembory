use clap::{ Parser };
use rig_Test::{ get_agent, ModelProvider };
mod tools;

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

    let agent = match args.model.as_str() {
        "anthropic" => get_agent(ModelProvider::Anthropic),
        "gemini" => get_agent(ModelProvider::Gemini),
        _ => unreachable!(),
    };

    let result = agent.run(&args.prompt, 20).await?;

    println!("\n\nReasoning Agent: {result}");

    Ok(())
}
