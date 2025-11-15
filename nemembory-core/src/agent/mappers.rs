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
