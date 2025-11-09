use async_trait::async_trait;
use serde::{ Deserialize, Serialize };
use thiserror::Error;

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

pub struct ChatSession {
    pub messages: Vec<Message>,
    pub handlers: Vec<Box<dyn MessageHandler + Send + Sync>>,
}

impl ChatSession {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            handlers: Vec::new(),
        }
    }

    pub async fn add_message(&mut self, message: Message) {
        self.messages.push(message.clone());
        for handler in &self.handlers {
            if let Err(e) = handler.handle_message(message.clone()).await {
                // log the error but continue processing other handlers
                eprintln!("Error handling message: {}", e);
                continue;
            }
        }
    }
}
