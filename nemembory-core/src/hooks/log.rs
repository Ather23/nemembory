use std::collections::HashMap;

// Callback to handle tool call metadata from the agent hook
pub fn log_tool_call(params: HashMap<String, String>) {
    if let Some(tool_name) = params.get("tool_name") {
        if let Some(args) = params.get("args") {
            println!("Tool called: {} with args: {}", tool_name, args);
        }
    }
}

// Callback to handle tool call result metadata from the agent hook
pub fn log_tool_call_result(params: HashMap<String, String>) {
    if let Some(tool_name) = params.get("tool_name") {
        if let Some(result) = params.get("result") {
            println!("Tool result: {} returned: {}", tool_name, result);
        }
    }
}
