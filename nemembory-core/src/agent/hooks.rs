use async_trait::async_trait;
use rig::{ agent::{ CancelSignal, PromptHook }, completion::CompletionModel };
use thiserror::Error;
use std::sync::Arc;
use std::sync::Mutex;
use std::future::Future;
use std::pin::Pin;
use std::collections::HashMap;

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

#[derive(Clone)]
pub struct HandleAgentResponse {
    pub(crate) callbacks: Vec<Arc<dyn Fn(HashMap<String, String>) + Send + Sync>>,
}

impl<M: CompletionModel> PromptHook<M> for HandleAgentResponse {
    async fn on_tool_call(&self, tool_name: &str, args: &str, _cancel_sig: CancelSignal) {
        for callback in &self.callbacks.clone() {
            let mut params = HashMap::new();
            params.insert("tool_name".to_string(), tool_name.to_string());
            params.insert("args".to_string(), args.to_string());
            callback(params);
        }
    }

    async fn on_tool_result(
        &self,
        tool_name: &str,
        args: &str,
        result: &str,
        _cancel_sig: CancelSignal
    ) {
        println!("hi");
    }

    fn on_completion_call(
        &self,
        prompt: &rig::message::Message,
        history: &[rig::message::Message],
        cancel_sig: CancelSignal
    ) -> impl Future<Output = ()> + rig::wasm_compat::WasmCompatSend {
        async {}
    }

    fn on_completion_response(
        &self,
        prompt: &rig::message::Message,
        response: &rig::completion::CompletionResponse<<M as CompletionModel>::Response>,
        cancel_sig: CancelSignal
    ) -> impl Future<Output = ()> + rig::wasm_compat::WasmCompatSend {
        async {}
    }
}

impl HandleAgentResponse {
    pub fn new() -> Self {
        Self {
            callbacks: Vec::new(),
        }
    }

    pub fn add_callback<F>(&mut self, callback: F)
        where F: Fn(HashMap<String, String>) + Send + Sync + 'static
    {
        self.callbacks.push(Arc::new(callback));
    }

    pub fn call_callbacks(&self, params: HashMap<String, String>) {
        for callback in &self.callbacks {
            callback(params.clone());
        }
    }
}
