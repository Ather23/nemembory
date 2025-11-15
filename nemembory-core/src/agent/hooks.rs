use async_trait::async_trait;
use rig::{ agent::{ CancelSignal, PromptHook }, completion::CompletionModel };
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AgentHookError {
    #[error("Agent Hook on tool call error: {0}")] AgentHookError(String),
}

#[async_trait]
pub trait AgentHooks {
    async fn on_tool_call(&self, tool_name: &str, args: &str) -> Result<(), AgentHookError>;
    async fn on_tool_result(
        &self,
        tool_name: &str,
        args: &str,
        result: &str
    ) -> Result<(), AgentHookError>;
}

#[derive(Clone, Debug)]
pub struct HandleAgentResponse;

#[async_trait]
trait AgentResponseHandler {
    async fn handle_tool_call(&self, tool_name: &str, args: &str) -> Result<(), AgentHookError>;
    async fn handle_tool_result(
        &self,
        tool_name: &str,
        args: &str,
        result: &str
    ) -> Result<(), AgentHookError>;
}

#[async_trait]
impl AgentResponseHandler for HandleAgentResponse {
    async fn handle_tool_call(&self, tool_name: &str, args: &str) -> Result<(), AgentHookError> {
        println!("Calling tool: {} with args: {}", tool_name, args);
        Ok(())
    }

    async fn handle_tool_result(
        &self,
        tool_name: &str,
        args: &str,
        result: &str
    ) -> Result<(), AgentHookError> {
        println!("Tool name {} Result :{}", &tool_name, &result);
        Ok(())
    }
}

impl<M: CompletionModel> PromptHook<M> for HandleAgentResponse {
    async fn on_tool_call(&self, tool_name: &str, args: &str, _cancel_sig: CancelSignal) {
        let _ = self.handle_tool_call(tool_name, args).await;
    }

    async fn on_tool_result(
        &self,
        tool_name: &str,
        args: &str,
        result: &str,
        cancel_sig: CancelSignal
    ) {
        let _ = self.handle_tool_result(&tool_name, &args, &result).await;
    }
}
