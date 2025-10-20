use serde::{ Deserialize, Serialize };
use rig::{ tool::Tool, completion::ToolDefinition };
use anyhow::Result;

#[derive(Deserialize, Serialize)]
pub struct RestApiTool;

#[derive(Debug, Deserialize, Serialize)]
pub struct RestApiArgs {
    pub url: String,
    #[serde(default)]
    pub method: Option<String>, // GET, POST, etc.
    #[serde(default)]
    pub body: Option<String>, // For POST/PUT
}

#[derive(Debug, thiserror::Error)]
pub enum RestApiError {
    #[error("Request failed: {0}")] RequestError(String),
    #[error(transparent)] Other(#[from] anyhow::Error),
}

impl Tool for RestApiTool {
    const NAME: &'static str = "rest_api";
    type Error = RestApiError;
    type Args = RestApiArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "rest_api".to_string(),
            description: "Calls a RESTful endpoint using reqwest.".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "url": {
                        "type": "string",
                        "description": "The URL of the REST endpoint"
                    },
                    "method": {
                        "type": "string",
                        "description": "HTTP method (GET, POST, etc.)",
                        "default": "GET"
                    },
                    "body": {
                        "type": "string",
                        "description": "Request body for POST/PUT",
                        "default": ""
                    }
                },
                "required": ["url", "body","method"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let client = reqwest::Client::new();
        println!("Args: {:?}", &args);
        let method = args.method.unwrap_or_else(|| "GET".to_string()).to_uppercase();
        let resp = match method.as_str() {
            "GET" => client.get(&args.url).send().await,
            "POST" => client.post(&args.url).body(args.body.unwrap_or_default()).send().await,
            "PUT" => client.put(&args.url).body(args.body.unwrap_or_default()).send().await,
            "DELETE" => client.delete(&args.url).send().await,
            _ => {
                return Err(RestApiError::RequestError(format!("Unsupported method: {}", method)));
            }
        };
        let response = resp.map_err(|e| RestApiError::RequestError(e.to_string()))?;
        let text = response.text().await.map_err(|e| RestApiError::RequestError(e.to_string()))?;
        Ok(text)
    }
}
