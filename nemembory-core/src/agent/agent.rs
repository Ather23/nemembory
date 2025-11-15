use async_trait::async_trait;
use chrono_tz::America::Toronto;
use rig::{
    agent::Agent,
    client::{ CompletionClient },
    completion::{ CompletionModel, Prompt, PromptError },
    message::Message,
    providers::{ anthropic, gemini },
};

use crate::{ LinkToMarkdown, RestApiTool, ShellTool, WebSearch, agent::hooks::HandleAgentResponse };

#[async_trait]
pub trait RunnableAgent: Send + Sync {
    async fn run(
        &self,
        prompt: &str,
        messages: &Vec<Message>,
        max_turns: usize,
        nemembory_hook: &HandleAgentResponse
    ) -> Result<String, PromptError>;
}

#[async_trait]
impl<M: CompletionModel + Send + Sync> RunnableAgent for Agent<M> {
    async fn run(
        &self,
        prompt: &str,
        messages: &Vec<Message>,
        max_turns: usize,
        nemembory_hook: &HandleAgentResponse
    ) -> Result<String, PromptError> {
        self
            .prompt(prompt)
            .with_hook(nemembory_hook.clone())
            .with_history(&mut messages.clone())
            .multi_turn(max_turns).await
    }
}

#[derive(Debug, Clone)]
pub enum ModelProvider {
    Anthropic,
    Gemini,
}

pub fn get_agent(provider: ModelProvider) -> Box<dyn RunnableAgent> {
    let todays_date = chrono::Utc::now().with_timezone(&Toronto);
    let preamble =
        format!(r#"
            # Goal:
            You are an assistant here to help the user select which tool is most appropriate to perform the task specified by the user.
            Follow these instructions closely.
            1. Consider the user's request carefully and identify the core elements of the request.
            2. Select which tool among those made available to you is appropriate given the context.
            3. This is very important: never perform the operation yourself.
            
            # Context: 
            Todays date is: {}"#, todays_date);

    match provider {
        ModelProvider::Anthropic => {
            let client: anthropic::Client = anthropic::Client::from_env();
            let agent = client
                .agent(anthropic::CLAUDE_4_SONNET)
                .preamble(&preamble)
                .max_tokens(1024)
                .tool(RestApiTool)
                .tool(WebSearch)
                .tool(ShellTool)
                .tool(LinkToMarkdown)
                .build();
            Box::new(agent)
        }
        ModelProvider::Gemini => {
            let client: gemini::Client = gemini::Client::from_env();
            let agent = client
                .agent(gemini::completion::GEMINI_1_0_PRO)
                .preamble(&preamble)
                .max_tokens(1024)
                .tool(RestApiTool)
                .tool(WebSearch)
                .tool(ShellTool)
                .tool(LinkToMarkdown)
                .build();
            Box::new(agent)
        }
    }
}
