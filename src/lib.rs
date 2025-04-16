//! Example

mod db;

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
// use sled::open;
use std::{
    io,
    path::{Path, PathBuf},
    str,
};

#[derive(Debug)]
pub enum Error {
    DbError(sled::Error),
    DbNotFound,
    SomeError,
    BinError(bincode::Error),
    IoError(io::Error),
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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub project_name: String,
    pub work_path: PathBuf,
    pub db_path: PathBuf,
    pub body: Vec<Issue>,
}

impl Project {
    pub fn open<T: AsRef<str>, P: AsRef<Path>>(title: T, path: P) -> Result<Self, Error> {
        let db_path = path.as_ref().join("db").with_extension("toml");
        let db = {
            if db_path.exists() {
                return db::load_db(&path); // TODO: これで失敗している場合の処理も後で
            } else {
                let a = Project {
                    project_name: title.as_ref().to_string(),
                    work_path: path.as_ref().to_path_buf(),
                    db_path,
                    body: Vec::new(),
                };
                db::save(a)?;
            }
            db::load_db(&path)
        }?;
        Ok(db)
    }
    pub fn add(&self, new_issue: Issue) -> Result<(), Error> {
        let mut loaded_db = db::load_db(&self.db_path)?;
        loaded_db.insert(new_issue);
        db::save(loaded_db)?;
        Ok(())
    }
    fn insert(&mut self, new_issue: Issue) {
        self.body.push(new_issue);
    }

    pub fn get<S: AsRef<str>>(
        &self,
        target_title: S,
    ) -> Result<Option<(MatchType, Vec<Issue>)>, Error> {
        let loaded_db = db::load_db(&self.db_path)?;

        let mut match_type = MatchType::Exact;
        // TODO: mutを消せるように
        // ブラケットで囲んでifで分岐すれば、おそらく束縛は1回で済む
        let mut found_issues: Vec<Issue> = loaded_db
            .body
            .iter()
            .filter(|f| f.title == *target_title.as_ref())
            .cloned()
            .collect();
        // 完全一致がない場合は、部分一致で埋める
        if found_issues.is_empty() {
            found_issues = loaded_db
                .body
                .iter()
                .filter(|f| f.title.contains(target_title.as_ref()))
                .cloned()
                .collect();
            if !found_issues.is_empty() {
                match_type = MatchType::Partial;
            }
        };
        Ok(if found_issues.is_empty() {
            None
        } else {
            Some((match_type, found_issues))
        })
    }
}

pub enum MatchType {
    Exact,
    Partial,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub enum Status {
    #[default]
    Open,
    Closed,
    Archived,
    Deleted,
}

#[cfg(test)]
mod tests {
}
