use async_trait::async_trait;
use chrono_tz::America::Toronto;
use rig::{
    agent::Agent,
    client::{ CompletionClient, ProviderClient },
    completion::{ CompletionModel, Prompt, PromptError },
    providers::{ anthropic, gemini },
};

use serde::{ Deserialize, Serialize };
use std::sync::Arc;
use crate::{ LinkToMarkdown, RestApiTool, ShellTool, WebSearch, agent::hooks::HandleAgentResponse };
use crate::hooks::log_tool_call;

#[async_trait]
pub trait RunnableAgent: Send + Sync {
    async fn run(
        &self,
        prompt: &str,
        messages: &Vec<rig::message::Message>,
        max_turns: usize,
        nemembory_hook: &HandleAgentResponse
    ) -> Result<String, PromptError>;
}

#[async_trait]
impl<M: CompletionModel + Send + Sync> RunnableAgent for rig::agent::Agent<M> {
    async fn run(
        &self,
        prompt: &str,
        messages: &Vec<rig::message::Message>,
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

pub fn get_agent(provider: ModelProvider, task: String) -> Box<dyn RunnableAgent> {
    let todays_date = chrono::Utc::now().with_timezone(&Toronto);
    let preamble = format!(
        r#"
            # Goal:
            You are an assistant here to help the user accomplish the following task: 
            {} 
            You have access to some tools that can help you with with performing your goal
            Select the tool is most appropriate to perform the task specified by the user.
            Follow these instructions closely.
            1. Consider the user's request carefully and identify the core elements of the request.
            2. Select which tool among those made available to you is appropriate given the context.
            3. This is very important: never perform the operation yourself.
            
            # Context: 
            Todays date is: {}"#,
        &task,
        todays_date
    );

    match provider {
        ModelProvider::Anthropic => {
            let client: anthropic::Client = anthropic::Client::from_env();
            let agent = client
                .agent(anthropic::Claude4Sonnet)
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
                .agent(gemini::completion::Gemini25Pro)
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

#[derive(Clone, Debug)]
pub struct Message {
    pub role: MessageRole,
    pub message: String,
    pub time_stamp: chrono::DateTime<chrono::Utc>,
}

impl Message {
    pub fn new(role: MessageRole, message: String) -> Self {
        Self {
            role,
            message,
            time_stamp: chrono::Utc::now(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum MessageRole {
    User,
    Assistant,
}

#[async_trait]
pub trait MessageHandler {
    async fn handle_message(&self, message: Message) -> Result<(), std::io::Error>;
}
pub struct NememboryAgent {
    pub messages: Vec<Message>,
    pub handlers: Vec<Arc<dyn MessageHandler + Send + Sync>>,
    pub agent: Box<dyn RunnableAgent + Send + Sync + 'static>,
}

impl NememboryAgent {
    pub fn new(task: String, model: ModelProvider) -> Self {
        Self {
            messages: Vec::new(),
            handlers: Vec::new(),
            agent: get_agent(model, task.to_string()),
        }
    }
    pub async fn run(&mut self, prompt: &str, max_turns: usize) -> Result<String, std::io::Error> {
        let messages = self.messages
            .clone()
            .into_iter()
            .map(|m| m.into())
            .collect::<Vec<rig::message::Message>>();

        let mut hook: HandleAgentResponse = HandleAgentResponse::new();
        hook.add_callback(log_tool_call);

        match self.agent.run(prompt, &messages, max_turns, &hook).await {
            Ok(result) => {
                self.add_message(Message::new(MessageRole::User, prompt.to_string())).await;
                self.add_message(Message::new(MessageRole::Assistant, result.clone())).await;

                Ok(result)
            }
            Err(e) =>
                Err(
                    std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Agent run failed: {}", e)
                    )
                ),
        }
    }

    pub async fn add_message(&mut self, message: Message) {
        self.messages.push(message.clone());

        //todo: move this to a seperate function
        for handler in &self.handlers {
            let handler = Arc::clone(handler);
            if let Err(e) = handler.handle_message(message.clone()).await {
                // log the error but continue processing other handlers
                eprintln!("Error handling message: {}", e);
                continue;
            }
        }
    }
}
