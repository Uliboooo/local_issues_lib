//! ## Example
//!
//! - 追記機能を導入予定
//! - インタラクティブモードを導入予定
//!
// TODO: Id structの導入はやめ。jsonなら1s未満で数千件から数万件は処理できそうなのでそちらに頼る。1プロジェクトで数万件を超えるisuesは扱わないと想定するためÏ

mod db;

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
// use sled::open;
use std::{
    collections::HashMap,
    fmt::Debug,
    io,
    path::{Path, PathBuf},
    str,
};

#[derive(Debug)]
pub enum Error {
    DbNotFound,
    IoError(io::Error),
    SerdeError(serde_json::Error),
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, PartialOrd, Eq)]
pub enum Status {
    #[default]
    Open,
    Closed(Closed),
    /// count of delted this issue
    MarkedAsDeleted(i32),
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, PartialOrd, Eq)]
pub enum Closed {
    #[default]
    Resolved,
    NotResolved,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Issue {
    title: String,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
    due_date: Option<DateTime<Local>>,
    status: Status,
    body_path: PathBuf,
}

impl Issue {
    pub fn new<S: AsRef<str>, P: AsRef<Path>>(
        title: S,
        due_date: Option<DateTime<Local>>,
        status: Status,
        body_path: P,
    ) -> Self {
        let now = Local::now();
        Self {
            title: title.as_ref().to_string(),
            created_at: now,
            updated_at: now,
            due_date,
            status,
            body_path: body_path.as_ref().to_path_buf(),
        }
    }

    /// edit title by arg title: String
    fn edit_title(&mut self, title: String) {
        self.title = title
    }

    /// update `updated_at` by now time
    fn update_date(&mut self) {
        self.updated_at = Local::now();
    }

    /// edit `due_date` by arg
    fn edit_due_date(&mut self, new_due: DateTime<Local>) {
        self.due_date = Some(new_due);
    }

    /// edit status
    fn edit_status(&mut self, new_status: Status) {
        self.status = new_status;
    }

    // /// if status is open, change to closed.if it is `MarkedAsDeleted`, reset count(default: 10).
    // fn update_status(&mut self) {
    //     self.status = match self.status {
    //         Status::Open => Status::Closed(C),
    //         Status::Closed => Status::Archived,
    //         Status::Archived => Status::MarkedAsDeleted(10),
    //         Status::MarkedAsDeleted(_) => Status::MarkedAsDeleted(10),
    //     };
    // }

    fn will_be_deleted_after(&self) -> Option<i32> {
        match self.status {
            Status::MarkedAsDeleted(c) => Some(c),
            _ => None,
        }
    }

    fn set_delete_count(&mut self, new_count: i32) -> Option<i32> {
        match self.status {
            Status::MarkedAsDeleted(_) => {
                self.status = Status::MarkedAsDeleted(new_count);
                if let Status::MarkedAsDeleted(c) = self.status {
                    Some(c)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn reset_delete_count(&mut self) -> Option<i32> {
        self.set_delete_count(10)
    }

    /// edit body path
    fn edit_body_path(&mut self, new_path: PathBuf) {
        self.body_path = new_path;
    }

    /// decrement delete flag count
    fn decrement_delete_count(&mut self) {
        if let Status::MarkedAsDeleted(c) = self.status {
            self.status = Status::MarkedAsDeleted(c - 1)
        }
    }
}

pub enum MatchType {
    Exact,
    Partial,
}

pub trait ProjectTrait {
    // manage paroject
    fn open<S: AsRef<str>, P: AsRef<Path>>(title: S, path: P) -> Result<Project, Error>;
    fn save(&self) -> Result<(), Error>;
    // manage issues
    /// when title exact match, return `Some(id: u64)`.
    fn get_id_from_title<S: AsRef<str>>(&self, title: S) -> Option<u64>;
    /// return ids(Vec<u64>) when matched status.
    fn get_ids_from_status<S: AsRef<str>>(&self, status: Status) -> Option<Vec<u64>>;

    /// add issue
    fn add_issue(&self, new_issue: Issue) -> Result<(), Error>;
    fn remove_issue_from_id(&mut self, id: u64);
    fn pop_issue(&mut self, id: u64) -> Option<Issue>;

    fn remove_issue<S: AsRef<str>>(&mut self, title: S) {
        let id = self.get_id_from_title(title);
        if let Some(f) = id {
            self.remove_issue_from_id(f);
        }
    }

    fn get_from_title<S: AsRef<str>>(&self, target_title: S) -> Result<Option<FetchedList>, Error>;
    // manage tags
    fn add_tags(&mut self, new_tags: &mut Vec<String>);
    fn remove_tag(&mut self, tag_names: Vec<String>);
    fn get_tags(&mut self) -> Vec<String>;
    fn check_delete_flag(&mut self) -> i32;
}

pub type BodyData = HashMap<u64, Issue>;
pub type FetchedList = (MatchType, BodyData);

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub project_name: String,
    pub work_path: PathBuf,
    pub db_path: PathBuf,
    pub body: BodyData,
    current_id: u64,
    pub tags: Vec<String>,
}

impl Project {
    pub fn open<S: AsRef<str>, P: AsRef<Path>>(title: S, path: P) -> Result<Self, Error> {
        let db_path = path.as_ref().join("db").with_extension("toml");
        let db = {
            if db_path.exists() {
                return db::load_db(&path); // TODO: これで失敗している場合の処理も後で
            } else {
                // like a new()
                // when file isn't exist, init Project as id = 0
                let void_body = Project {
                    project_name: title.as_ref().to_string(),
                    work_path: path.as_ref().to_path_buf(),
                    db_path,
                    body: HashMap::new(),
                    current_id: 0,
                    tags: Vec::new(),
                };
                db::save(void_body)?;
            }
            db::load_db(&path)
        }?;
        Ok(db)
    }
    pub fn add_issue(&self, new_issue: Issue) -> Result<(), Error> {
        let mut loaded_db = db::load_db(&self.db_path)?;
        loaded_db.insert(new_issue);
        db::save(loaded_db)?;
        Ok(())
    }

    fn insert(&mut self, new_issue: Issue) {
        self.body.insert(self.current_id + 1, new_issue);
    }

    /// Return the issue struct(and match type(`Exact` or `Partial`)) that matches the argument title
    pub fn get_from_title<S: AsRef<str>>(
        &self,
        target_title: S,
    ) -> Result<Option<FetchedList>, Error> {
        let loaded_db = db::load_db(&self.db_path)?;

        let mut match_type = MatchType::Exact;

        Ok({
            let mut tmp = loaded_db
                .body
                .iter()
                .filter_map(|f| {
                    if f.1.title == target_title.as_ref() {
                        Some((*f.0, f.1.clone()))
                    } else {
                        None
                    }
                })
                .collect::<HashMap<_, _>>();

            // 完全一致がない場合
            if tmp.is_empty() {
                tmp = tmp
                    .iter()
                    .filter_map(|f| {
                        if f.1.title.contains(target_title.as_ref()) {
                            Some((*f.0, f.1.clone()))
                        } else {
                            None
                        }
                    })
                    .collect();
                match_type = MatchType::Partial;
            }

            if tmp.is_empty() {
                None
            } else {
                Some(FetchedList::from((match_type, tmp)))
            }
        })
    }

    /// add tags to self.tags from arg: `Vec<String>`
    pub fn add_tags(&mut self, new_tags: &mut Vec<String>) {
        self.tags.append(new_tags);
    }

    /// remove tags from self.tags, by tag_names: `Vec<String>`
    pub fn remove_tag(&mut self, tag_names: Vec<String>) {
        // fがtag_namesに含まれている場合は削除される。
        self.tags.retain(|f| tag_names.iter().any(|t| t != f));
    }

    /// return cloned current tags: `Vec<String>`
    pub fn get_tags(&mut self) -> Vec<String> {
        self.tags.clone()
    }

    /// incomplete
    fn check_delete_flag(&mut self) -> i32 {
        for i in &self.body {
            if let Status::MarkedAsDeleted(c) = i.1.status {
                if c == 0 {
                    return 32;
                }
            }
        }
        todo!()
    }
}

#[cfg(test)]
mod tests {}
