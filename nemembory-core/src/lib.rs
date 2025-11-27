pub mod agent;
pub mod tools;
pub mod data;
pub mod hooks;
pub mod handlers;

pub use agent::{ get_agent, ModelProvider, RunnableAgent, NememboryAgent };
pub use tools::{ RestApiTool, WebSearch, ShellTool, LinkToMarkdown };
pub use data::{ Agent, Tool, AgentPersistence };
use rig::completion::{ CompletionModel, PromptError };
use rig::providers::{ anthropic, gemini };
