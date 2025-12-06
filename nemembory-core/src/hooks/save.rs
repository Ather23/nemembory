use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

pub struct WriteToolLogToFile {
    pub path: String,
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
        println!("Writing agent response to file... {:?}", params);
        println!("File path: {}", &self.path);

        let file_result = OpenOptions::new().append(true).create(true).open(&self.path);

        let mut file = match file_result {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Failed to open file {}: {}", &self.path, e);
                return;
            }
        };

        // Handle tool call parameters (from on_tool_call)
        if let (Some(tool_name), Some(args)) = (params.get("tool_name"), params.get("args")) {
            if let Err(e) = writeln!(file, "Tool: {}\nArgs: {}\n", tool_name, args) {
                eprintln!("Failed to write to file: {}", e);
            }
        } else if
            // Handle completion response parameters (from on_completion_response)
            let (Some(content), Some(role)) = (params.get("content"), params.get("role"))
        {
            if let Err(e) = writeln!(file, "Role: {}\nContent: {}\n", role, content) {
                eprintln!("Failed to write to file: {}", e);
            }
        }
    }
}
