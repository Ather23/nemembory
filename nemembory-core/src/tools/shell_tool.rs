use serde::{ Deserialize, Serialize };
use rig::{ tool::Tool, completion::ToolDefinition };
use anyhow::Result;
use tokio::process::Command;

#[derive(Deserialize, Serialize)]
pub struct ShellTool;

#[derive(Deserialize, Serialize)]
pub struct ShellArgs {
    pub command: String,
}

#[derive(Debug, thiserror::Error)]
pub enum ShellError {
    #[error("Failed to execute command: {0}")] ExecutionError(String),
    #[error(transparent)] Other(#[from] anyhow::Error),
}

#[allow(unused_imports)]
use std::future::Future;

impl Tool for ShellTool {
    const NAME: &'static str = "shell_tool";
    type Error = ShellError;
    type Args = ShellArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "shell_tool".to_string(),
            description: "Executes a shell command on **Windows** and returns the output.".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "command": {
                        "type": "string",
                        "description": "The shell command to execute"
                    }
                },
                "required": ["command"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        // Use powershell for Windows
        let output = Command::new("powershell")
            .arg("-Command")
            .arg(&args.command)
            .output().await
            .map_err(|e| ShellError::ExecutionError(e.to_string()))?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(ShellError::ExecutionError(String::from_utf8_lossy(&output.stderr).to_string()))
        }
    }
}
