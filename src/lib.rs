pub mod config;
pub mod display_options;
mod storage;
mod users;

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::Display,
    io,
    path::{Path, PathBuf},
};
// use users::ManageUsers;
pub use users::{User, Users};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const DB_NAME: &str = "db.json";

#[derive(Debug)]
pub enum Error {
    DbError(storage::Error),
    SomeError,
    NotFound,
    Io(io::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::DbError(e) => write!(f, "db error: {}", e),
            Error::SomeError => write!(f, "some error. please retry."),
            Error::NotFound => write!(f, "not found"),
            Error::Io(error) => write!(f, "io error: {}", error),
        }
    }
}

impl Error {
    pub fn is_file_is_zero(&self) -> bool {
        match self {
            Error::DbError(e) => matches!(e, storage::Error::FileIsZero),
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct Message {
    message: String,
    show: bool,
    created_at: DateTime<Local>,
    author: User,
}
impl Message {
    pub fn new<S: AsRef<str>, U: Into<User>>(message: S, show: bool, author: U) -> Self {
        Self {
            message: message.as_ref().to_string(),
            show,
            created_at: Local::now(),
            author: author.into(),
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
    fn hide_message_by_id(&mut self, id: u64);
    fn show_message_by_id(&mut self, id: u64);
    fn add_message_to(&mut self, new_message: Message);
    fn rm_message(&mut self, id: u64);
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Messages(Vec<Message>);

impl Messages {
    fn count_messages(&self) -> i32 {
        self.0.len() as i32
    }
}

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

pub trait StatusT {
    fn is_opened(&self) -> bool;
    fn to_emoji(&self) -> String;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
enum Status {
    Open,
    Closed(Closed),
}

impl StatusT for Status {
    fn is_opened(&self) -> bool {
        matches!(self, Status::Open)
    }

    fn to_emoji(&self) -> String {
        match self {
            Status::Open => "🟢",
            Status::Closed(closed) => match closed {
                Closed::Resolved => "✅🔴",
                Closed::UnResolved => "❌🔴",
            },
        }
        .to_string()
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

#[derive(Debug, Serialize, Deserialize, Clone)]
enum Closed {
    Resolved,
    UnResolved,
}

trait IssueTrait {
    fn update(&mut self);
    fn comment<S: AsRef<str>, U: Into<User>>(&mut self, msg_str: S, author: U);
    fn rename<S: AsRef<str>>(&mut self, new_title: S);
    fn edit_due(&mut self, new_due: DateTime<Local>);
    fn rm_comment(&mut self, id: u64);
    fn hide_message(&mut self, id: u64);
    fn show_message(&mut self, id: u64);
    fn search<S: AsRef<str>>(&self, target_title: S) -> Option<u64>;
    fn search_list<S: AsRef<str>>(&self, target_title: S) -> Option<Vec<u64>>;
    fn close(&mut self, is_resolved: bool);
    fn open(&mut self);
    fn is_opened(&self) -> bool;
    fn get_message(&self) -> &Messages;
    fn change_author<U: Into<User>>(&mut self, new_author: U);
    fn assign_user<U: Into<User>>(&mut self, user: U);
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Issue {
    name: String,
    messages: Messages,
    status: Status,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
    due_date: Option<DateTime<Local>>,
    author: User,
    assigned_member: Option<Users>,
}

impl Issue {
    pub fn new<S: AsRef<str>, U: Into<User>>(name: S, author: U) -> Self {
        Self {
            name: name.as_ref().to_string(),
            messages: Messages::new(),
            status: Status::Open,
            created_at: Local::now(),
            updated_at: Local::now(),
            due_date: None,
            author: author.into(),
            assigned_member: None,
        }
    }
    pub fn count_message(&self) -> i32 {
        self.messages.count_messages()
    }
}

impl IssueTrait for Issue {
    fn update(&mut self) {
        self.updated_at = Local::now();
    }

    fn comment<S: AsRef<str>, U: Into<User>>(&mut self, msg_str: S, author: U) {
        self.update();
        self.messages
            .add_message_to(Message::new(msg_str, true, author));
    }

    fn rename<S: AsRef<str>>(&mut self, new_title: S) {
        self.update();
        self.name = new_title.as_ref().to_string();
    }

    /// set due_date as new_due. if it is `None`, change to Some(DateTime<Local>)
    fn edit_due(&mut self, new_due: DateTime<Local>) {
        self.update();
        self.due_date = Some(new_due);
    }

    /// ⚠️ this fn remove message in Vec and rewrite index.
    /// recommend hide_message().
    fn rm_comment(&mut self, id: u64) {
        self.update();
        self.messages.rm_message(id);
    }

    fn hide_message(&mut self, id: u64) {
        self.update();
        self.messages.hide_message_by_id(id);
    }

    fn show_message(&mut self, id: u64) {
        self.update();
        self.messages.show_message_by_id(id);
    }

    /// return first id found.
    fn search<S: AsRef<str>>(&self, target_title: S) -> Option<u64> {
        self.messages
            .0
            .iter()
            .position(|f| f.message == *target_title.as_ref())
            .map(|f| f as u64)
    }

    fn search_list<S: AsRef<str>>(&self, target_title: S) -> Option<Vec<u64>> {
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

    fn close(&mut self, is_resolved: bool) {
        self.update();
        if is_resolved {
            self.status = Status::Closed(Closed::Resolved);
        } else {
            self.status = Status::Closed(Closed::UnResolved);
        }
    }

    fn open(&mut self) {
        self.update();
        self.status = Status::Open;
    }

    fn is_opened(&self) -> bool {
        self.status.is_opened()
    }

    fn get_message(&self) -> &Messages {
        &self.messages
    }

    fn change_author<U: Into<User>>(&mut self, new_author: U) {
        self.update();
        self.author = new_author.into();
    }

    fn assign_user<U: Into<User>>(&mut self, user: U) {
        self.update();
        match &mut self.assigned_member {
            Some(v) => v.add_user(user),
            None => {
                self.assigned_member = Some({
                    let mut n = Users::new();
                    n.add_user(user);
                    n
                })
            }
        }
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
    pub fn fmt_only_prop(&self) -> String {
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
        format!("{}\n\n", issue_info)
    }

    pub fn fmt_only_open_prop(&self) -> String {
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
            format!("{}\n\n", issue_info)
        } else {
            String::new()
        }
    }
}

pub trait ProjectManager {
    fn new<S: AsRef<str>, P: AsRef<Path>>(name: S, project_path: P) -> Self;
    fn open<P: AsRef<Path>>(project_path: P) -> Result<Self, Error>
    where
        Self: Sized;
    fn open_or_create<P: AsRef<Path>, S: AsRef<str>>(
        project_path: P,
        name: S,
        // author: User,
    ) -> Result<Self, Error>
    where
        Self: Sized;
    fn save(&self) -> Result<(), Error>;
    fn update(&mut self);
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Project {
    project_name: String,
    issues: HashMap<u64, Issue>,
    current_id: u64,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
    project_path: PathBuf,
    storage_path: PathBuf,
    db_path: PathBuf,
    users: Users,
}

impl ProjectManager for Project {
    /// * create `storage_path` or `db_path` based on `project_path`.
    /// * and times info generated based on the current time.
    fn new<S: AsRef<str>, P: AsRef<Path>>(name: S, project_path: P) -> Self {
        let storage_path = project_path.as_ref().to_path_buf().join(".local_issue");
        let db_path = storage_path.join(DB_NAME);

        Self {
            project_name: name.as_ref().to_string(),
            issues: HashMap::new(),
            current_id: 0,
            created_at: Local::now(),
            updated_at: Local::now(),
            project_path: project_path.as_ref().to_path_buf(),
            storage_path,
            db_path,
            users: Users::new(),
        }
    }

    /// return loaded `Project` if file(db) is empty, error
    fn open<P: AsRef<Path>>(project_path: P) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let storage_path = project_path.as_ref().to_path_buf().join(".local_issue");
        let db_path = storage_path.join(DB_NAME);

        storage::load::<Project, _>(db_path, false).map_err(Error::DbError)
    }

    /// if db.json not found, create new.
    fn open_or_create<P: AsRef<Path>, S: AsRef<str>>(
        project_path: P,
        name: S,
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let storage_path = project_path.as_ref().to_path_buf().join(".local_issue");
        let db_path = storage_path.join(DB_NAME);

        storage::load::<Project, _>(db_path, true)
            // if create new, file size is 0,
            .or_else(|e| {
                if e.is_file_is_zero() {
                    Ok(Project::new(name, project_path))
                } else {
                    Err(e)
                }
            })
            .map_err(Error::DbError)
    }

    /// save to db based on a path of Self
    fn save(&self) -> Result<(), Error> {
        storage::save(self, &self.db_path).map_err(Error::DbError)
    }

    fn update(&mut self) {
        self.updated_at = Local::now();
    }
}

/// manage users
impl Project {
    pub fn add_user<U: Into<User>>(&mut self, new_user: U) {
        self.update();
        self.users.add_user(new_user);
    }
    pub fn list_users(&self) -> Vec<String> {
        self.users.users_list()
    }

    pub fn change_author_of_issue<U: Into<User>>(&mut self, new_author: U, issue_id: u64) {
        if let Some(f) = self.issues.get_mut(&issue_id) {
            f.change_author(new_author);
            self.update();
        }
    }

    pub fn assign_new_user_to_issue<U: Into<User>>(&mut self, new_user: U, issue_id: u64) {
        if let Some(f) = self.issues.get_mut(&issue_id) {
            f.assign_user(new_user);
            self.update();
        }
    }
}

pub trait SearchIssue {
    fn search_issue<S: AsRef<str>>(&self, target_title: S) -> Option<u64>;
    fn search_issues<S: AsRef<str>>(&self, target_title: S) -> Option<Vec<u64>>;
}

impl SearchIssue for Project {
    fn search_issue<S: AsRef<str>>(&self, target_title: S) -> Option<u64> {
        self.issues
            .iter()
            .find(|f| f.1.name == *target_title.as_ref())
            .map(|f| *f.0)
    }

    fn search_issues<S: AsRef<str>>(&self, target_title: S) -> Option<Vec<u64>> {
        let a = self
            .issues
            .iter()
            .filter(|f| f.1.name == *target_title.as_ref())
            .map(|f| *f.0)
            .collect::<Vec<u64>>();
        if a.is_empty() { Some(a) } else { None }
    }
}

impl Project {
    pub fn count_issues(&self) -> i32 {
        self.issues.iter().len() as i32
    }
    pub fn count_comments(&self, issue_id: u64) -> Option<i32> {
        self.issues.get(&issue_id).map(|f| f.count_message())
    }
}

// ctrl Project
impl Project {
    pub fn rename<T: AsRef<str>>(&mut self, title: T) {
        self.update();
        self.project_name = title.as_ref().to_string();
    }
}

impl Project {
    pub fn add_issue<T: AsRef<str>, U: Into<User>>(&mut self, new_name: T, author: U) {
        self.update();
        self.current_id += 1;
        self.issues
            .insert(self.current_id, Issue::new(new_name, author));
    }

    pub fn rename_issue<T: AsRef<str>>(&mut self, issue_id: u64, new_name: T) {
        self.update();
        if let Some(f) = self.issues.get_mut(&issue_id) {
            f.rename(new_name);
        }
    }

    pub fn edit_issue_due(&mut self, id: u64, due: DateTime<Local>) {
        self.update();
        if let Some(f) = self.issues.get_mut(&id) {
            f.edit_due(due);
        }
    }

    pub fn to_open_issue(&mut self, issue_id: u64) {
        if let Some(f) = self.issues.get_mut(&issue_id) {
            f.open();
            self.update();
        }
    }

    pub fn to_close_issue(&mut self, issue_id: u64, is_resolved: bool) {
        if let Some(f) = self.issues.get_mut(&issue_id) {
            f.close(is_resolved);
            self.update();
        }
    }

    pub fn is_opened_issue(&self, issue_id: u64) -> Option<bool> {
        self.issues.get(&issue_id).map(|f| f.is_opened())
    }

    pub fn exist(&self, issue_id: u64) -> bool {
        // `is_opened_issue()` return true when it exist.
        self.is_opened_issue(issue_id).is_some()
    }

    pub fn remove_issue(&mut self, issue_id: u64) {
        self.update();
        self.issues.remove(&issue_id);
    }
}

pub trait SearchComment {
    fn search_comment_position<T: AsRef<str>>(&self, issue_id: u64, target_title: T)
    -> Option<u64>;
    fn search_comments<T: AsRef<str>>(&self, issue_id: u64, target_title: T) -> Option<&Messages>;
    fn search_comments_positions<T: AsRef<str>>(
        &self,
        issue_id: u64,
        target_title: T,
    ) -> Option<Vec<u64>>;
}

impl SearchComment for Project {
    fn search_comment_position<T: AsRef<str>>(
        &self,
        issue_id: u64,
        target_title: T,
    ) -> Option<u64> {
        self.issues
            .get(&issue_id)
            .and_then(|f| f.search(target_title))
    }

    fn search_comments<T: AsRef<str>>(&self, issue_id: u64, target_title: T) -> Option<&Messages> {
        self.search_comment_position(issue_id, target_title)
            .and_then(|f| self.issues.get(&f).map(|f| f.get_message()))
    }

    fn search_comments_positions<T: AsRef<str>>(
        &self,
        issue_id: u64,
        target_title: T,
    ) -> Option<Vec<u64>> {
        self.issues
            .get(&issue_id)
            .and_then(|f| f.search_list(target_title))
    }
}

/// edit comment msg
impl Project {
    pub fn add_comment<T: AsRef<str>, U: Into<User>>(
        &mut self,
        issue_id: u64,
        comment: T,
        author: U,
    ) {
        if let Some(f) = self.issues.get_mut(&issue_id) {
            f.comment(comment, author);
            self.update();
        }
    }

    /// ⚠️ this fn remove message in Vec and rewrite index.
    /// recommend hide_message().
    pub fn rm_comment(&mut self, issue_id: u64, comment_id: u64) {
        if let Some(f) = self.issues.get_mut(&issue_id) {
            f.rm_comment(comment_id);
            self.update();
        }
    }

    pub fn set_comment_as_visible(&mut self, comment_id: u64, issue_id: u64) {
        if let Some(f) = self.issues.get_mut(&issue_id) {
            f.show_message(comment_id);
            self.update();
        }
    }

    pub fn set_comment_as_hidden(&mut self, comment_id: u64, issue_id: u64) {
        if let Some(f) = self.issues.get_mut(&issue_id) {
            f.hide_message(comment_id);
            self.update();
        }
    }
}

impl Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sub_prop = format!(
            "  created at: {}\n  updated at: {}\n  Path: {}",
            self.created_at.to_rfc2822(),
            self.updated_at.to_rfc2822(),
            self.project_path.to_string_lossy(),
        );
        let iss = self
            .issues
            .iter()
            .map(|f| (f.0, f.1))
            .collect::<Vec<(&u64, &Issue)>>()
            .iter()
            .map(|f| format!("#{}: {}", f.0, f.1))
            .collect::<String>();

        let r = format!("Project: {}\n{}\n\n{}", self.project_name, sub_prop, iss);
        write!(f, "{}", r)
    }
}

impl Project {
    pub fn fmt_only_prop(&self) -> String {
        self.issues.iter().map(|f| f.1.fmt_only_prop()).collect()
    }
    pub fn fmt_only_open(&self) -> String {
        self.issues.iter().map(|f| f.1.fmt_only_open()).collect()
    }
    pub fn fmt_only_open_prop(&self) -> String {
        self.issues
            .iter()
            .map(|f| f.1.fmt_only_open_prop())
            .collect()
    }
}

pub fn db_path(work_path: PathBuf) -> PathBuf {
    work_path.join(".local_issue").join(DB_NAME)
}
