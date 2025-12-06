use crate::agent::agent::Message;

impl From<Message> for rig::message::Message {
    fn from(message: Message) -> Self {
        match message.role {
            crate::agent::agent::MessageRole::User => rig::message::Message::user(message.message),
            crate::agent::agent::MessageRole::Assistant =>
                rig::message::Message::assistant(message.message),
        }
    }
}

impl From<rig::message::Message> for Message {
    fn from(message: rig::message::Message) -> Self {
        match message {
            rig::message::Message::User { content, .. } =>
                Self {
                    role: crate::agent::agent::MessageRole::User,
                    message: content
                        .iter()
                        .map(|f| {
                            match f {
                                rig::message::UserContent::Text(text) => text.text.clone(),
                                rig::message::UserContent::ToolResult(_) => String::new(),
                                rig::message::UserContent::Image(_) => String::new(),
                                rig::message::UserContent::Audio(_) => String::new(),
                                rig::message::UserContent::Video(_) => String::new(),
                                rig::message::UserContent::Document(_) => String::new(),
                            }
                        })
                        .collect::<String>(),
                    time_stamp: chrono::Utc::now(),
                },
            rig::message::Message::Assistant { content, .. } =>
                Self {
                    role: crate::agent::agent::MessageRole::Assistant,
                    message: content
                        .iter()
                        .map(|f| {
                            match f {
                                rig::message::AssistantContent::Text(text) => text.text.clone(),
                                rig::message::AssistantContent::ToolCall(_) => String::new(),
                                // Catch-all for Reasoning, Image, and any future variants
                                _ => String::new(),
                            }
                        })
                        .collect::<String>(),
                    time_stamp: chrono::Utc::now(),
                },
        }
    }
}
