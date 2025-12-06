use rig::{
    client::{ CompletionClient, ProviderClient },
    providers::{ anthropic, gemini, openrouter },
};

use chrono_tz::America::Toronto;
use crate::RunnableAgent;
use crate::{ LinkToMarkdown, RestApiTool, ShellTool, WebSearch };

#[derive(Debug, Clone)]
pub enum ModelProvider {
    Anthropic,
    Gemini,
    OpenRouter(String),
}

pub fn build_runnable_agent(provider: ModelProvider, task: String) -> Box<dyn RunnableAgent> {
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
        ModelProvider::OpenRouter(model) => open_router_agent(&model, task),
    }
}

fn open_router_agent(provider: &str, task: String) -> Box<dyn RunnableAgent> {
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

    let client = openrouter::Client::from_env();
    let agent = client
        .agent(provider)
        .preamble(&preamble)
        .max_tokens(1024)
        .tool(RestApiTool)
        .tool(WebSearch)
        .tool(ShellTool)
        .tool(LinkToMarkdown)
        .build();

    Box::new(agent)
}
