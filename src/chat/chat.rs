use async_trait::async_trait;
use nemembory::RunnableAgent;
use serde::{ Deserialize, Serialize };
use std::sync::Arc;

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
pub struct AgentChat {
    pub messages: Vec<Message>,
    pub handlers: Vec<Arc<dyn MessageHandler + Send + Sync>>,
    pub agent: Box<dyn RunnableAgent + Send + Sync + 'static>,
}

impl AgentChat {
    pub fn new(model: nemembory::ModelProvider) -> Self {
        Self {
            messages: Vec::new(),
            handlers: Vec::new(),
            agent: nemembory::get_agent(model),
        }
    }
    pub async fn run(&mut self, prompt: &str, max_turns: usize) -> Result<String, std::io::Error> {
        let messages = self.messages
            .clone()
            .into_iter()
            .map(|m| m.into())
            .collect::<Vec<rig::message::Message>>();
        match self.agent.run(prompt, &messages, max_turns).await {
            Ok(result) => {
                self.add_message(Message::new(MessageRole::Assistant, result.clone())).await;
                self.add_message(Message::new(MessageRole::User, prompt.to_string())).await;
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
