use chrono::Local;
use rig::{ completion::ToolDefinition, tool::Tool };
use serde::{ Deserialize, Serialize };
use serde_json::json;

#[derive(Debug, thiserror::Error)]
#[error("Date retrieval error")]
pub struct DateError;

#[derive(Deserialize, Serialize)]
pub struct GetDate;

impl Tool for GetDate {
    const NAME: &'static str = "get_date";
    type Error = DateError;
    type Args = GetDateArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "get_date".to_string(),
            description: "Returns today's date and time in YYYY-MM-DD HH:MM:SS format".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {},
                "required": []
            }),
        }
    }

    async fn call(&self, _args: Self::Args) -> Result<Self::Output, Self::Error> {
        let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        Ok(now)
    }
}

#[derive(Deserialize)]
pub struct GetDateArgs;
