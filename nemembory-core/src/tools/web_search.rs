use brave_rs::{ BraveClient, WebSearchApiResponse, brave::BraveClientError };
use rig::{ completion::ToolDefinition, tool::Tool };
use serde::{ Deserialize, Serialize };
use serde_json::json;

#[derive(Debug, thiserror::Error)]
#[error("Search error")]
pub struct SearchError;

#[derive(Deserialize, Serialize)]
pub struct WebSearch;
impl Tool for WebSearch {
    const NAME: &'static str = "web_search";
    type Error = SearchError;
    type Args = WebSearchArgs;
    type Output = Vec<String>;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "web_search".to_string(),
            description: "Searches the web".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "The query to search the web"
                    },
                },
                "required": ["query"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let api_key = std::env::var("BRAVE_API_KEY").expect("BRAVE_API_KEY not set");
        let client = BraveClient::new(&api_key);

        println!("Query: {}", &args.query);
        let result = client.web_search_by_query(&args.query).await;
        match result {
            Ok(response) => {
                let search_result = response.web
                    .unwrap()
                    .results.iter()
                    .map(|r| r.description.clone())
                    .collect();
                Ok(search_result)
            }
            Err(er) => {
                eprintln!("Brave client error: {}", er.to_string());
                Err(er.into())
            }
        }
    }
}

impl From<BraveClientError> for SearchError {
    fn from(_: BraveClientError) -> Self {
        SearchError
    }
}

#[derive(Deserialize)]
pub struct WebSearchArgs {
    query: String,
}
