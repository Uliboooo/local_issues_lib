use std::{collections::HashMap, fmt::Display};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
struct Message {
    body: String,
    at: DateTime<Local>,
}
impl Message {
    fn new<S: AsRef<str>>(message: S) -> Self {
        Message {
            body: message.as_ref().to_string(),
            at: Local::now(),
        }
    }
}
impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Date: {}\n    {}", self.at.to_rfc2822(), self.body)
    }
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct CommitMessages {
    messages: HashMap<u64, Message>,
    current_id: u64,
}
impl CommitMessages {
    pub fn new() -> Self {
        Self {
            messages: HashMap::new(),
            current_id: 0,
        }
    }

    pub fn push<S: AsRef<str>>(&mut self, message: S) {
        self.current_id += 1;
        self.messages.insert(self.current_id, Message::new(message));
    }

    pub fn remove(&mut self, id: u64) {
        self.messages.remove(&id);
    }
}
impl Display for CommitMessages {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let a = self
            .messages
            .iter()
            .map(|f| format!("{}\n\n", f.1))
            .collect::<String>();
        write!(f, "{}", a)
    }
}

#[cfg(test)]
mod tests {
    use super::CommitMessages;

    #[test]
    fn test_commit_messages_fmt() {
        let mut new_messages = CommitMessages::new();
        new_messages.push("message1");
        new_messages.push("message2");
        println!("{}", new_messages);
    }
}