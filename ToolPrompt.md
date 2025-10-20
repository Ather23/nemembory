# AI Prompt for Generating a Rust Tool

Write Rust code that defines a new struct and implements the [`Tool`](https://docs.rs/rig-core/latest/rig/tool/trait.Tool.html) trait for it, following the examples below.

- The tool should have its own name, arguments, output type, and error handling.
- Implement the [`definition`](src/main.rs) and [`call`](src/main.rs) async methods.
- Use serde for argument serialization/deserialization.
- The tool should be similar in structure to [`WebSearch`](src/main.rs) and `Adder`:

```rust
#[derive(Deserialize, Serialize)]
struct WebSearch;
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
        // ...implementation...
    }
}
```

Replace the struct, argument, and logic with your own tool’s purpose.  
Make sure to define the argument struct, error type, and implement both required methods.
