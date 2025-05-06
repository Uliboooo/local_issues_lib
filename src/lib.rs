pub mod config;
mod db;
// pub mod printer;
mod users;
// mod build;

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::Display,
    io,
    path::{Path, PathBuf},
};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug)]
pub enum Error {
    DbError(db::Error),
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
            Error::DbError(e) => matches!(e, db::Error::FileIsZero),
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
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
    fn hide_message_by_id(&mut self, id: u64);
    fn show_message_by_id(&mut self, id: u64);
    fn add_message_to(&mut self, new_message: Message);
    fn rm_message(&mut self, id: u64);
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Messages(Vec<Message>);

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
}

#[derive(Debug, Serialize, Deserialize)]
enum Status {
    Open,
    Closed(Closed),
}

impl StatusT for Status {
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

#[derive(Debug, Serialize, Deserialize)]
enum Closed {
    Resolved,
    UnResolved,
}

trait IssueTrait {
    fn update(&mut self);
    fn commit<S: AsRef<str>>(&mut self, msg_str: S);
    fn rename<S: AsRef<str>>(&mut self, new_title: S);
    fn edit_due(&mut self, new_due: DateTime<Local>);
    fn rm_commit(&mut self, id: u64);
    fn hide_message(&mut self, id: u64);
    fn show_message(&mut self, id: u64);
    fn search<S: AsRef<str>>(&self, target_title: S) -> Option<u64>;
    fn search_list<S: AsRef<str>>(&self, target_title: S) -> Option<Vec<u64>>;
    fn close(&mut self, is_resolved: bool);
    fn open(&mut self);
    fn is_opened(&self) -> bool;
    fn get_message(&self) -> &Messages;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Issue {
    name: String,
    messages: Messages,
    status: Status,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
    due_date: Option<DateTime<Local>>,
    // users: Users,
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
}

impl IssueTrait for Issue {
    fn update(&mut self) {
        self.updated_at = Local::now();
    }

    fn commit<S: AsRef<str>>(&mut self, msg_str: S) {
        self.update();
        self.messages.add_message_to(Message::new(msg_str, true));
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
    fn rm_commit(&mut self, id: u64) {
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

pub trait DbProject {
    fn new<S: AsRef<str>, P: AsRef<Path>>(name: S, project_path: P) -> Self;
    fn open<P: AsRef<Path>>(project_path: P) -> Result<Self, Error>
    where
        Self: Sized;
    fn data_load<P: AsRef<Path>>(path: P) -> Result<Self, Error>
    where
        Self: Sized;
    fn data_load_without_creating<P: AsRef<Path>>(path: P) -> Result<Self, Error>
    where
        Self: Sized;
    fn save(&self) -> Result<(), Error>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    project_name: String,
    issues: HashMap<u64, Issue>,
    current_id: u64,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
    project_path: PathBuf,
    storage_path: PathBuf,
    db_path: PathBuf,
}

impl DbProject for Project {
    fn new<S: AsRef<str>, P: AsRef<Path>>(name: S, project_path: P) -> Self {
        let storage_path = project_path.as_ref().to_path_buf().join(".local_issue");
        let db_path = storage_path.join("db.json");

        Self {
            project_name: name.as_ref().to_string(),
            issues: HashMap::new(),
            current_id: 0,
            created_at: Local::now(),
            updated_at: Local::now(),
            project_path: project_path.as_ref().to_path_buf(),
            storage_path,
            db_path,
        }
    }

    fn open<P: AsRef<Path>>(project_path: P) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let storage_path = project_path.as_ref().to_path_buf().join(".local_issue");
        let db_path = storage_path.join("db.json");


        // if !storage_path.exists() || !db_path.exists() {
        //     return Err(Error::NotFound);
        // }
        db::load::<Project, _>(db_path, true).map_err(Error::DbError)
    }

    /// ⚠️ when db.json is 0, create new json.
    /// At that time, use Project::new().
    ///
    /// ## args
    ///
    /// * path: db path
    fn data_load<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        db::load(path, true).map_err(Error::DbError)
    }

    /// ## args
    ///
    /// * path: db path
    fn data_load_without_creating<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        db::load(path, false).map_err(Error::DbError)
    }

    fn save(&self) -> Result<(), Error> {
        db::save(self, &self.db_path).map_err(Error::DbError)
    }
}

// ctrl Project
impl Project {
    pub fn rename<T: AsRef<str>>(&mut self, title: T) {
        self.project_name = title.as_ref().to_string();
    }

    // / change storage_path and db_path automatic.
    // / ⚠️ don't change already exist db.json path
    // fn change_project_path<P: AsRef<Path>>(&mut self, path: P) {
    //     let storage_path = path.as_ref().to_path_buf().join(".local_issue");
    //     let db_path = path.as_ref().join("db.json");

    //     (self.project_path, self.storage_path, self.db_path) =
    //         (path.as_ref().to_path_buf(), storage_path, db_path);
    // }

    pub fn search_issue<S: AsRef<str>>(&self, target_title: S) -> Option<u64> {
        self.issues
            .iter()
            .find(|f| f.1.name == *target_title.as_ref())
            .map(|f| *f.0)
    }

    pub fn search_issues<S: AsRef<str>>(&self, target_title: S) -> Option<Vec<u64>> {
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
    pub fn add_issue<T: AsRef<str>>(&mut self, new_name: T) {
        self.current_id += 1;
        self.issues.insert(self.current_id, Issue::new(new_name));
    }

    pub fn rename_issue<T: AsRef<str>>(&mut self, issue_id: u64, new_name: T) {
        if let Some(f) = self.issues.get_mut(&issue_id) {
            f.rename(new_name);
        }
    }

    pub fn edit_issue_due(&mut self, id: u64, due: DateTime<Local>) {
        if let Some(f) = self.issues.get_mut(&id) {
            f.edit_due(due);
        }
    }

    pub fn to_open_issue(&mut self, issue_id: u64) {
        if let Some(f) = self.issues.get_mut(&issue_id) {
            f.open()
        }
    }

    pub fn to_close_issue(&mut self, issue_id: u64, is_resolved: bool) {
        if let Some(f) = self.issues.get_mut(&issue_id) {
            f.close(is_resolved);
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
        self.issues.remove(&issue_id);
    }
}

/// edit commit msg
impl Project {
    pub fn commit<T: AsRef<str>>(&mut self, issue_id: u64, commit_msg: T) {
        if let Some(f) = self.issues.get_mut(&issue_id) {
            f.commit(commit_msg)
        }
    }

    /// ⚠️ this fn remove message in Vec and rewrite index.
    /// recommend hide_message().
    pub fn rm_commit(&mut self, issue_id: u64, commit_id: u64) {
        if let Some(f) = self.issues.get_mut(&issue_id) {
            f.rm_commit(commit_id);
        }
    }

    pub fn to_show_commit(&mut self, commit_id: u64, issue_id: u64) {
        if let Some(f) = self.issues.get_mut(&issue_id) {
            f.show_message(commit_id);
        }
    }

    pub fn to_hide_commit(&mut self, commit_id: u64, issue_id: u64) {
        if let Some(f) = self.issues.get_mut(&issue_id) {
            f.hide_message(commit_id);
        }
    }

    /// return index
    pub fn search_commit_position<T: AsRef<str>>(
        &self,
        issue_id: u64,
        target_title: T,
    ) -> Option<u64> {
        self.issues
            .get(&issue_id)
            .and_then(|f| f.search(target_title))
    }

    /// return ref of value
    pub fn search_commit<T: AsRef<str>>(
        &self,
        issue_id: u64,
        target_title: T,
    ) -> Option<&Messages> {
        self.search_commit_position(issue_id, target_title)
            .and_then(|f| self.issues.get(&f).map(|f| f.get_message()))
    }

    /// return indexes
    pub fn search_commits_positions<T: AsRef<str>>(
        &self,
        issue_id: u64,
        target_title: T,
    ) -> Option<Vec<u64>> {
        self.issues
            .get(&issue_id)
            .and_then(|f| f.search_list(target_title))
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
    work_path.join(".local_issue").join("db.json")
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::{env, thread, time};

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

    #[test]
    fn project_show_test() {
        let mut db = Project::new("test", "project_path");
        db.add_issue("1");
        db.add_issue("2");
        println!("{}", db);
    }

    #[test]
    fn db_db_test() {
        let cd = env::current_dir().unwrap();
        let work_path = env::current_dir().unwrap().join("test/test");
        let db_path = work_path.join("db.json");

        let loaded_db_without_create = Project::data_load_without_creating(cd.join("hoge"));
        assert!(loaded_db_without_create.is_err());

        let loaded_db = Project::data_load(db_path);
        let mut loaded_project = match loaded_db {
            Ok(v) => v,
            Err(e) => {
                if e.is_file_is_zero() {
                    Project::new("name", work_path)
                } else {
                    panic!();
                }
            }
        };

        println!("{}", loaded_project);

        // let new_issue = Issue::new("name");

        loaded_project.add_issue("new1");
        println!("{}", loaded_project);

        println!("--------------------------");

        loaded_project.commit(1, "commit_msg");
        loaded_project.commit(1, "commit_msg2");
        thread::sleep(time::Duration::from_secs(1));
        loaded_project.commit(1, "commit_msg3");

        loaded_project.add_issue("2");
        // loaded_project.hi

        println!("{}", loaded_project.fmt_only_open_prop());
    }

    #[test]
    fn hoge() {
        println!(
            "{:?}",
            env::current_dir().unwrap().join("test/test").exists()
        );
    }

    #[test]
    fn cf_version() {
        println!("{}", VERSION);
    }

    #[test]
    fn hogee() -> Result<(), Error> {
        let work_path = env::current_dir()
            .unwrap()
            .join("test_resource")
            .join("tests");
        std::fs::create_dir_all(&work_path).map_err(Error::Io)?;
        if work_path.exists() {
            println!("ex");
        }

        let mut db = Project::open(work_path).unwrap();

        println!("res: {:?}", db);

        db.add_issue("new_name");
        Ok(())
    }
}
