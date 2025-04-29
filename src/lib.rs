use chrono::{DateTime, Local};
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub struct Message {
    message: String,
    show: bool,
    created_at: DateTime<Local>,
}
impl Message {
    pub fn new<S: AsRef<str>>(message: S, show: bool) -> Self {
        Self {
            message: message.as_ref().to_string(),
            show,
            created_at: Local::now(),
        }
    }
    fn hide(&mut self) {
        self.show = false;
    }
    fn show(&mut self) {
        self.show = true
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.show {
            writeln!(f, "{}\n\t{}\n", self.created_at.to_rfc2822(), self.message)
        } else {
            write!(f, "")
        }
    }
}

trait ManageMessage {
    fn new() -> Self
    where
        Self: Sized;
    // fn id_increment(&mut self);

    fn hide_message_by_id(&mut self, id: u64);
    fn show_message_by_id(&mut self, id: u64);
    fn add_message_to(&mut self, new_message: Message);
    fn rm_message(&mut self, id: u64);
}

#[derive(Debug, Default)]
struct Messages(Vec<Message>);

impl ManageMessage for Messages {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self(Vec::new())
    }

    fn hide_message_by_id(&mut self, id: u64) {
        if let Some(f) = self.0.get_mut(id as usize) {
            f.hide()
        }
    }

    fn show_message_by_id(&mut self, id: u64) {
        if let Some(f) = self.0.get_mut(id as usize) {
            f.show()
        }
    }

    fn add_message_to(&mut self, new_message: Message) {
        // self.id_increment();
        self.0.push(new_message);
    }

    /// ⚠️ this fn remove message in Vec and rewrite index.
    fn rm_message(&mut self, id: u64) {
        self.0.remove(id as usize);
    }
}

// Tue, 29 Apr 2025 18:12:31 +0900
// message
//
// Tue, 29 Apr 2025 18:12:31 +0900
// message2
impl Display for Messages {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|f| { format!("{}", f) })
                .collect::<String>()
        )
    }
}

enum Status {
    Open,
    Closed(Closed),
}

impl Status {
    fn is_opened(&self) -> bool {
        matches!(self, Status::Open)
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Open => write!(f, "Open"),
            Status::Closed(closed) => match closed {
                Closed::Resolved => write!(f, "Resolved Closed"),
                Closed::UnResolved => write!(f, "UnResolved Closed"),
            },
        }
    }
}

enum Closed {
    Resolved,
    UnResolved,
}

pub struct Issue {
    name: String,
    messages: Messages,
    status: Status,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
    due_date: Option<DateTime<Local>>,
}

impl Issue {
    pub fn new<S: AsRef<str>>(name: S) -> Self {
        Self {
            name: name.as_ref().to_string(),
            messages: Messages::new(),
            status: Status::Open,
            created_at: Local::now(),
            updated_at: Local::now(),
            due_date: None,
        }
    }

    fn update(&mut self) {
        self.updated_at = Local::now();
    }

    pub fn commit<S: AsRef<str>>(&mut self, msg_str: S) {
        self.update();
        self.messages.add_message_to(Message::new(msg_str, true));
    }

    pub fn rename<S: AsRef<str>>(&mut self, new_title: S) {
        self.name = new_title.as_ref().to_string();
    }

    /// set due_date as new_due. if it is `None`, change to Some(DateTime<Local>)
    pub fn edit_due(&mut self, new_due: DateTime<Local>) {
        self.due_date = Some(new_due);
    }

    /// ⚠️ this fn remove message in Vec and rewrite index.
    /// recommend hide_message().
    pub fn rm_commit(&mut self, id: u64) {
        self.messages.rm_message(id);
    }

    pub fn hide_message(&mut self, id: u64) {
        self.messages.hide_message_by_id(id);
    }

    pub fn show_message(&mut self, id: u64) {
        self.messages.show_message_by_id(id);
    }

    /// return first id found.
    pub fn search<S: AsRef<str>>(&self, target_title: S) -> Option<u64> {
        self.messages
            .0
            .iter()
            .position(|f| f.message == *target_title.as_ref())
            .map(|f| f as u64)
    }

    pub fn search_list<S: AsRef<str>>(&self, target_title: S) -> Option<Vec<u64>> {
        let a = self
            .messages
            .0
            .iter()
            .enumerate()
            .filter(|f| f.1.message == *target_title.as_ref())
            .map(|f| f.0 as u64)
            .collect::<Vec<u64>>();
        if a.is_empty() { Some(a) } else { None }
    }

    pub fn close(&mut self, is_resolved: bool) {
        if is_resolved {
            self.status = Status::Closed(Closed::Resolved);
        } else {
            self.status = Status::Closed(Closed::UnResolved);
        }
    }

    pub fn open(&mut self) {
        self.status = Status::Open;
    }

    pub fn is_opened(&self) -> bool {
        self.status.is_opened()
    }
}

impl Display for Issue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let issue_info = format!(
            "issue: {}\n  status:\t{}\n  created:\t{}\n  update_at:\t{}\n  due date:\t{}",
            self.name,
            self.status,
            self.created_at.to_rfc2822(),
            self.updated_at.to_rfc2822(),
            self.due_date
                .map(|f| f.to_rfc2822())
                .unwrap_or("None".to_string())
        );
        write!(f, "{}\n\n{}\n", issue_info, self.messages)
    }
}

impl Issue {
    pub fn fmt_only_open(&self) -> String {
        if self.is_opened() {
            let issue_info = format!(
                "issue:\t\t{}\nstatus:\t\t{}\ncreated:\t{}\nupdate_at:\t{}\ndue date:\t{}",
                self.name,
                self.status,
                self.created_at.to_rfc2822(),
                self.updated_at.to_rfc2822(),
                self.due_date
                    .map(|f| f.to_rfc2822())
                    .unwrap_or("None".to_string())
            );
            format!("{}\n\n{}\n", issue_info, self.messages)
        } else {
            String::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{thread, time};

    use crate::Issue;

    use super::{ManageMessage, Message, Messages};

    #[test]
    fn messages_print_test() {
        let mut msgs = Messages::new();

        let new_msg = Message::new("message", true);
        msgs.add_message_to(new_msg);
        let new_msg2 = Message::new("message2", true);
        msgs.add_message_to(new_msg2);
        let new_msg3 = Message::new("hide", false);
        msgs.add_message_to(new_msg3);
        let new_msg4 = Message::new("show", true);
        msgs.add_message_to(new_msg4);

        println!("{}", msgs);
    }

    #[test]
    fn issue_tests() {
        let mut test_issue = Issue::new("test");

        test_issue.commit("test1_show");
        thread::sleep(time::Duration::from_secs(3));
        test_issue.commit("test2_hide");
        let hide_id = test_issue.search("test2_hide").unwrap();
        test_issue.hide_message(hide_id);

        println!("{}", test_issue);

        test_issue.show_message(hide_id);
        println!("{}", test_issue);
    }

    #[test]
    fn test_print_issue() {
        let mut open_issue = Issue::new("show_issue");
        open_issue.commit("msg_str");
        open_issue.commit("2");
        let mut close_issue = Issue::new("closed_issue");
        close_issue.close(true);

        let l = vec![open_issue, close_issue];
        for i in &l {
            println!("{}", i.fmt_only_open());
        }

        for i in &l {
            println!("{}", i);
        }
    }
}
