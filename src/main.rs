use clap::{ Parser };
use nemembory::{ get_agent, ModelProvider };

use crate::chat::chat::Message;
mod tools;
mod chat;
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

    let mut chat = chat::chat::AgentChat::new(
        if args.model.to_lowercase() == "anthropic" {
            ModelProvider::Anthropic
        } else {
            ModelProvider::Gemini
        }
    );

    let answer = chat.run(&args.prompt, 10).await?;
    println!("\n\nReasoning Agent: {answer}");

    // let agent = match args.model.as_str() {
    //     "anthropic" => get_agent(ModelProvider::Anthropic),
    //     "gemini" => get_agent(ModelProvider::Gemini),
    //     _ => unreachable!(),
    // };
    // let messages = vec![
    //     rig::message::Message::user("What is Rust?"),
    //     rig::message::Message::assistant("Rust is a systems programming language...")
    // ];
    // let result = agent.run(&args.prompt, &messages, 20).await?;

    // println!("\n\nReasoning Agent: {result}");

    // println!("\n\nMESSAGES: {:?}", messages);

    Ok(())
}
