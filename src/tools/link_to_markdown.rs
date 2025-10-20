use rig::{ completion::ToolDefinition, tool::Tool };
use serde::{ Deserialize, Serialize };
use serde_json::json;
use thiserror::Error;
use reqwest;
use html2md;

#[derive(Debug, Error)]
#[error("Failed to fetch or convert link contents")]
pub struct LinkToMarkdownError;

#[derive(Deserialize, Serialize)]
pub struct LinkToMarkdown;

#[derive(Deserialize, Serialize)]
pub struct LinkToMarkdownArgs {
    pub url: String,
}

impl Tool for LinkToMarkdown {
    const NAME: &'static str = "link_to_markdown";
    type Error = LinkToMarkdownError;
    type Args = LinkToMarkdownArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "link_to_markdown".to_string(),
            description: "Fetches the contents of a link for better context and converts it to markdown".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "url": {
                        "type": "string",
                        "description": "The URL to fetch and convert to markdown"
                    },
                },
                "required": ["url"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let resp = reqwest::get(&args.url).await.map_err(|_| LinkToMarkdownError)?;
        let html = resp.text().await.map_err(|_| LinkToMarkdownError)?;
        let markdown = html2md::parse_html(&html);
        Ok(markdown)
    }
}
