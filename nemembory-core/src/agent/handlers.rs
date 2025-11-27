use crate::agent::agent::{ Message, MessageHandler };
use async_trait::async_trait;

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
        use std::fs::OpenOptions;
        use std::io::Write;

        let file = OpenOptions::new().create(true).append(true).open(&self.file_path);
        writeln!(file?, "[{}] {:?}: {}", message.time_stamp, message.role, message.message)?;
        Ok(())
    }
}
