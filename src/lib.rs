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
    // pub fn add(new_issue: Self) -> Result<(), Error> {
    //     let data = sled::open(db_path()?).map_err(Error::DbError)?;
    //     let serialized_data = bincode::serialize(&new_issue).map_err(Error::BinError)?;
    //     let _result = data
    //         .insert(&new_issue.title, serialized_data)
    //         .map_err(Error::DbError)?;
    //     Ok(())
    // }
    // // fn load(target: String) -> Result<(), Error> {
    // //     let data = sled::open(db_path()?).map_err(Error::DbError)?;
    // //     data.get(key)
    // // }
    // pub fn get_all() -> Result<(), Error> {
    //     let db = sled::open(db_path()?).map_err(Error::DbError)?;
    //     let _a = db.get("title").map_err(Error::DbError)?;
    //     Ok(())
    // }
}

/// for test, lib users don't use this.
// fn db_path() -> Result<PathBuf, Error> {
//     Ok(PathBuf::from("value"))
// }

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
        // ブラケットで囲んでifで分岐すればおそらく束縛は1回で済む
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
    use crate::{Issue, Project};

    const WORKPATCH: &str = "/Users/yuki/Develop/local_issues_lib/";

    #[test]
    fn project_init_test() {
        let result = Project::open("title", WORKPATCH);
        assert!(result.is_ok());
    }

    #[test]
    fn add_new_issue_test() {
        let pro = Project::open("title", WORKPATCH).unwrap();
        println!("{:?}", pro);
        let new_issue = Issue::new("title", None, crate::Status::Open, "body_path");
        let result = pro.add(new_issue);
        println!("{:?}", result);
    }

    #[test]
    fn load_test() {
        let pro = Project::open("title", WORKPATCH).unwrap();
        let loaded = pro.load();
        println!("{:?}", loaded);
    }
}
