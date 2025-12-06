use async_trait::async_trait;
use rig::{ completion::{ CompletionModel, Prompt, PromptError } };

use serde::{ Deserialize, Serialize };
use std::sync::Arc;
use crate::{
    ModelProvider,
    agent::{ FileHandler, hooks::HandleAgentResponse, build_runnable_agent },
};
use crate::hooks::{ log_tool_call, write_to_file };

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

#[derive(Clone, Serialize, Deserialize, Debug)]
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
            agent: build_runnable_agent(model, task.to_string()),
        }
    }

    pub fn with_handlers(mut self, handlers: Vec<Arc<dyn MessageHandler + Send + Sync>>) -> Self {
        self.handlers = handlers;
        self
    }

    pub fn default_handlers(mut self) -> Self {
        let logging_handler = Arc::new(FileHandler::new("agent_messages.log".to_string()));
        self.handlers.push(logging_handler);
        self
    }

    pub async fn run(&mut self, prompt: &str, max_turns: usize) -> Result<String, std::io::Error> {
        let messages = self.messages
            .clone()
            .into_iter()
            .map(|m| m.into())
            .collect::<Vec<rig::message::Message>>();

        let mut hook: HandleAgentResponse = HandleAgentResponse::new();
        hook.add_callback(log_tool_call);
        hook.add_callback(write_to_file);

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
