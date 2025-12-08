use async_trait::async_trait;
use futures::{ Stream, StreamExt };
use rig::{
    agent::MultiTurnStreamItem,
    completion::{ CompletionModel, Prompt, PromptError },
    streaming::StreamingChat,
};
use std::pin::Pin;

use serde::{ Deserialize, Serialize };
use std::sync::Arc;
use crate::{ ModelProvider, agent::{ FileHandler, build_runnable_agent, hooks::LlmResponseHooks } };
use crate::hooks::{
    log_tool_call,
    log_tool_call_result,
    WriteToolLogToFile,
    WriteToolResultToFile,
};

/// A boxed error type for streaming operations
pub type StreamError = Box<dyn std::error::Error + Send + Sync>;

/// A pinned, boxed stream that yields string chunks or errors
pub type AgentStream<'a> = Pin<Box<dyn Stream<Item = Result<String, StreamError>> + Send + 'a>>;

pub struct NememboryAgent {
    pub messages: Vec<Message>,
    pub message_handlers: Vec<Arc<dyn MessageHandler + Send + Sync>>,
    pub name: String,
    pub agent: Box<dyn RunnableAgent + Send + Sync + 'static>,
    pub working_dir: Option<String>,
    pub has_working_dir: bool,
    pub hooks: Option<LlmResponseHooks>,
}

impl NememboryAgent {
    pub fn new(name: &str, task: String, model: ModelProvider) -> Self {
        Self {
            hooks: None,
            working_dir: None,
            has_working_dir: false,
            name: name.to_owned(),
            messages: Vec::new(),
            message_handlers: Vec::new(),
            agent: build_runnable_agent(model, task.to_string()),
        }
    }

    pub fn create_working_directory(mut self, working_dir: &str) -> Self {
        self.working_dir = Some(working_dir.to_string());
        if let Err(result) = std::fs::DirBuilder::new().recursive(true).create(working_dir) {
            panic!("Unable to create agent directory {}", result.to_string());
        }
        self.has_working_dir = true;
        self
    }

    pub fn with_handlers(mut self, handlers: Vec<Arc<dyn MessageHandler + Send + Sync>>) -> Self {
        self.message_handlers = handlers;
        self
    }

    pub fn with_hooks(mut self, hooks: LlmResponseHooks) -> Self {
        self.hooks = Some(hooks);
        self
    }

    pub fn default_handlers(mut self) -> Self {
        let has_dir = self.has_working_dir.to_owned();
        if has_dir {
            let working_dir = &self.working_dir.as_ref().unwrap();
            let chat_log_handler = Arc::new(
                FileHandler::new(format!("{}/{}", working_dir, "chat.log".to_string()))
            );
            self.message_handlers.push(chat_log_handler);
        }
        self
    }

    pub fn default_hooks(mut self) -> Self {
        // Agent task: Add directory check here

        let has_dir = self.has_working_dir.to_owned();
        if has_dir {
            let working_dir = &self.working_dir.as_ref().unwrap();
            let path = format!("{}/{}", working_dir, "tool.log".to_string());
            let file_handler = WriteToolLogToFile::new(&path);
            let result_path = format!("{}/{}", working_dir, "tool_result.log".to_string());
            let result_file_handler = WriteToolResultToFile::new(&result_path);
            let mut hooks = self.hooks.take().unwrap_or_else(LlmResponseHooks::new);
            hooks.add_tool_call_callback(move |params| file_handler.write_to_file(params));
            hooks.add_tool_call_result_callback(move |params|
                result_file_handler.write_to_file(params)
            );
            self.hooks = Some(hooks);
        }
        self
    }

    pub async fn run(&mut self, prompt: &str, max_turns: usize) -> Result<String, std::io::Error> {
        let messages = self.messages
            .clone()
            .into_iter()
            .map(|m| m.into())
            .collect::<Vec<rig::message::Message>>();

        let mut hooks = self.hooks.clone().unwrap_or_else(LlmResponseHooks::new);
        hooks.add_tool_call_callback(log_tool_call);
        hooks.add_tool_call_result_callback(log_tool_call_result);

        match self.agent.run(prompt, &messages, max_turns, &hooks).await {
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

    pub fn run_stream(&self, prompt: &str, max_turns: usize) -> AgentStream<'_> {
        let messages = self.messages
            .iter()
            .map(|m| m.clone().into())
            .collect::<Vec<rig::message::Message>>();

        self.agent.run_stream(prompt, &messages, max_turns)
    }

    pub async fn add_message(&mut self, message: Message) {
        self.messages.push(message.clone());

        //todo: move this to a seperate function
        for handler in &self.message_handlers {
            let handler = Arc::clone(handler);
            if let Err(e) = handler.handle_message(message.clone()).await {
                // log the error but continue processing other handlers
                eprintln!("Error handling message: {}", e);
                continue;
            }
        }
    }
}

#[async_trait]
pub trait RunnableAgent: Send + Sync {
    async fn run(
        &self,
        prompt: &str,
        messages: &Vec<rig::message::Message>,
        max_turns: usize,
        nemembory_hook: &LlmResponseHooks
    ) -> Result<String, PromptError>;

    fn run_stream(
        &self,
        prompt: &str,
        messages: &Vec<rig::message::Message>,
        max_turns: usize
    ) -> AgentStream<'_>;
}

#[async_trait]
impl<M: CompletionModel + Send + Sync + 'static> RunnableAgent for rig::agent::Agent<M> {
    async fn run(
        &self,
        prompt: &str,
        messages: &Vec<rig::message::Message>,
        max_turns: usize,
        nemembory_hook: &LlmResponseHooks
    ) -> Result<String, PromptError> {
        self
            .prompt(prompt)
            .with_hook(nemembory_hook.clone())
            .with_history(&mut messages.clone())
            .multi_turn(max_turns).await
    }

    fn run_stream(
        &self,
        prompt: &str,
        messages: &Vec<rig::message::Message>,
        _max_turns: usize
    ) -> AgentStream<'_> {
        let messages = messages.to_vec();
        let prompt = prompt.to_string();

        Box::pin(
            async_stream::stream! {
                let mut stream = self.stream_chat(&prompt, messages).await;

                while let Some(result) = stream.next().await {
                    match result {
                        Ok(multi) => {
                            match multi {
                                MultiTurnStreamItem::StreamAssistantItem(
                                    streamed_assistant_content,
                                ) => {
                                    match streamed_assistant_content {
                                        rig::streaming::StreamedAssistantContent::Text(text) => {
                                            yield Ok(text.text().to_string());
                                        }
                                        rig::streaming::StreamedAssistantContent::ToolCall(_) => {}
                                        rig::streaming::StreamedAssistantContent::ToolCallDelta {
                                            ..
                                        } => {}
                                        rig::streaming::StreamedAssistantContent::Reasoning(_) => {}
                                        rig::streaming::StreamedAssistantContent::Final(_) => {}
                                    }
                                }
                                MultiTurnStreamItem::StreamUserItem(_) => {}
                                MultiTurnStreamItem::FinalResponse(_) => {}
                                _ => {}
                            }
                        }
                        Err(err) => {
                            yield Err(Box::new(err) as Box<dyn std::error::Error + Send + Sync>);
                        }
                    }
                }
            }
        )
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
