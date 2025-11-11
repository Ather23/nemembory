use crate::chat::chat::Message;

impl From<Message> for rig::message::Message {
    fn from(message: Message) -> Self {
        match message.role {
            crate::chat::chat::MessageRole::User => rig::message::Message::user(message.message),
            crate::chat::chat::MessageRole::Assistant =>
                rig::message::Message::assistant(message.message),
        }
    }
}
