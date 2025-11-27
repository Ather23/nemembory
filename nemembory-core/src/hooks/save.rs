use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;

// Callback to handle tool call metadata from the agent hook
pub fn write_to_file(params: HashMap<String, String>) {
    println!("Writing agent response to file... {:?}", params);
    if let Some(content) = params.get("content") {
        if let Some(role) = params.get("role") {
            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open("agent_response.txt")
                .expect("Unable to open file");

            writeln!(file, "Role: {}\nContent: {}\n", role, content).expect(
                "Unable to write to file"
            );
        }
    }
}
