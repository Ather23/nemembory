use std::collections::HashMap;
use std::fs;
use std::path::Path;

use chrono::{ DateTime, Utc };
use rig::embeddings::tool;
use serde::{ Deserialize, Serialize };

pub struct WriteToolLogToFile {
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ToolLog {
    name: String,
    args: String,
    timestamp: DateTime<Utc>,
}

impl ToolLog {
    pub fn new(name: &str, args: &str) -> Self {
        Self { name: name.to_owned(), args: args.to_owned(), timestamp: Utc::now() }
    }
}

impl WriteToolLogToFile {
    pub fn new(path: &str) -> Self {
        let path = Path::new(path);
        let file_path = if path.is_dir() || !path.to_string_lossy().contains('.') {
            path.join("tool.log")
        } else {
            path.to_path_buf()
        };
        Self { path: file_path.to_string_lossy().to_string() }
    }

    // Callback to handle tool call metadata from the agent hook
    pub fn write_to_file(&self, params: HashMap<String, String>) {
        dbg!(&self.path);
        let mut tool_logs: Vec<ToolLog> = if Path::new(&self.path).exists() {
            match fs::read_to_string(&self.path) {
                Ok(contents) if !contents.trim().is_empty() => {
                    serde_json::from_str(&contents).unwrap_or_else(|e| {
                        eprintln!("Failed to deserialize tool logs: {}", e);
                        Vec::new()
                    })
                }
                Ok(_) => Vec::new(),
                Err(e) => {
                    eprintln!("Failed to read file {}: {}", &self.path, e);
                    Vec::new()
                }
            }
        } else {
            Vec::new()
        };

        dbg!(&params);
        // Build new ToolLog from tool_name and content
        if let (Some(tool_name), Some(args)) = (params.get("tool_name"), params.get("args")) {
            let new_log = ToolLog::new(tool_name, args);

            // Update the vector with the new ToolLog
            tool_logs.push(new_log);

            // Save the file with the updated vector
            match serde_json::to_string_pretty(&tool_logs) {
                Ok(json) => {
                    if let Err(e) = fs::write(&self.path, json) {
                        eprintln!("Failed to write to file {}: {}", &self.path, e);
                    }
                }
                Err(e) => {
                    eprintln!("Failed to serialize tool logs: {}", e);
                }
            }
        }
    }
}

pub struct WriteToolResultToFile {
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ToolResultLog {
    name: String,
    args: String,
    result: String,
    timestamp: DateTime<Utc>,
}

impl ToolResultLog {
    pub fn new(name: &str, args: &str, result: &str) -> Self {
        Self {
            name: name.to_owned(),
            args: args.to_owned(),
            result: result.to_owned(),
            timestamp: Utc::now(),
        }
    }
}

impl WriteToolResultToFile {
    pub fn new(path: &str) -> Self {
        let path = Path::new(path);
        let file_path = if path.is_dir() || !path.to_string_lossy().contains('.') {
            path.join("tool_result.log")
        } else {
            path.to_path_buf()
        };
        Self { path: file_path.to_string_lossy().to_string() }
    }

    // Callback to handle tool result metadata from the agent hook
    pub fn write_to_file(&self, params: HashMap<String, String>) {
        let mut tool_logs: Vec<ToolResultLog> = if Path::new(&self.path).exists() {
            match fs::read_to_string(&self.path) {
                Ok(contents) if !contents.trim().is_empty() => {
                    serde_json::from_str(&contents).unwrap_or_else(|e| {
                        eprintln!("Failed to deserialize tool result logs: {}", e);
                        Vec::new()
                    })
                }
                Ok(_) => Vec::new(),
                Err(e) => {
                    eprintln!("Failed to read file {}: {}", &self.path, e);
                    Vec::new()
                }
            }
        } else {
            Vec::new()
        };

        // Build new ToolResultLog from tool_name, args and result
        if
            let (Some(tool_name), Some(args), Some(result)) = (
                params.get("tool_name"),
                params.get("args"),
                params.get("result"),
            )
        {
            let new_log = ToolResultLog::new(tool_name, args, result);

            // Update the vector with the new ToolResultLog
            tool_logs.push(new_log);

            // Save the file with the updated vector
            match serde_json::to_string_pretty(&tool_logs) {
                Ok(json) => {
                    if let Err(e) = fs::write(&self.path, json) {
                        eprintln!("Failed to write to file {}: {}", &self.path, e);
                    }
                }
                Err(e) => {
                    eprintln!("Failed to serialize tool result logs: {}", e);
                }
            }
        }
    }
}
