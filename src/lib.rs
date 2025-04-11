use std::path::PathBuf;

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sled::Db;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Issue {
    title: String,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
    due_date: DateTime<Local>,
    status: Status,
    body_path: PathBuf,
}

impl Issue {
    fn new(title: String, due_date: DateTime<Local>, status: Status, body_path: PathBuf) -> Self {
        let now = Local::now();
        Self {
            title,
            created_at: now,
            updated_at: now,
            due_date,
            status,
            body_path,
        }
    }
    fn save(&self) -> Result<(), ()> {
        let path = PathBuf::from("value"); // TODO: あとで設定しろ
        let data = Db::open_tree(&self, name)
        Err(())
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
enum Status {
    #[default]
    Open,
    Closed,
}
