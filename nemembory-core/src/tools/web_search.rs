use brave_rs::{ brave::BraveClientError, BraveClient };
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
    type Output = String;

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
        let result = client.web_search_by_query(&args.query).await?;

        let first_result = match &result.web {
            Some(web_results) =>
                web_results.results
                    .first()
                    .map(|r| r.description.clone())
                    .unwrap_or_else(|| "No results found".to_string()),
            None => "No web results".to_string(),
        };

        println!("search result {:?}", &first_result);
        Ok(first_result)
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
