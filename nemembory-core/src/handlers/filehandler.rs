use crate::agent::agent::{ Message, MessageHandler };
use async_trait::async_trait;
use std::fs::OpenOptions;
use std::io::{ Read, Write };
pub struct FileHandler {
    file_path: String,
}

impl FileHandler {
    pub fn new(file_path: String) -> Self {
        Self { file_path }
    }
}

#[async_trait]
impl MessageHandler for FileHandler {
    async fn handle_message(&self, message: Message) -> Result<(), std::io::Error> {
        // Read file
        let mut history: Vec<Message> = if
            let Ok(mut file) = OpenOptions::new().read(true).open(&self.file_path)
        {
            let mut buffer = String::new();
            file.read_to_string(&mut buffer)?;
            if buffer.is_empty() {
                Vec::new()
            } else {
                serde_json::from_str(&buffer).unwrap_or_else(|_| Vec::new())
            }
        } else {
            Vec::new()
        };

        // add new message
        history.push(message.clone());

        // add upddate message
        let serialized = serde_json::to_string_pretty(&history).unwrap();
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&self.file_path)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }
}
