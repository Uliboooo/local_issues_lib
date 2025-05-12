use crate::users::Users;
use crate::{VERSION, storage};
use home::home_dir;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, path::PathBuf};

#[derive(Debug)]
pub enum Error {
    DbError(storage::Error),
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Config {
    pub version: String,
    pub users: Users,
}

impl Config {
    pub fn new() -> Self {
        Self {
            users: Users::new(),
            version: VERSION.to_string(),
        }
    }

    pub fn load() -> Result<Self, Error> {
        match storage::load(get_config_path().unwrap(), true) {
            Ok(v) => Ok(v),
            Err(e) => {
                if e.is_file_is_zero() {
                    Ok(Self::default())
                } else {
                    Err(Error::DbError(e))
                }
            }
        }
    }
    pub fn save(&self) {
        storage::save(self, get_config_path().unwrap()).unwrap();
    }
}

impl Config {
    /// this function don't save() automatically.please use `.save()`
    pub fn edit_users(&mut self, new_users: Users) {
        self.users = new_users;
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fm = serde_json::to_string(self).unwrap().to_string();
        write!(f, "{}", fm)
    }
}

fn get_config_path() -> Option<PathBuf> {
    if cfg!(test) {
        return Some(PathBuf::from("test/config_test/").join("config.json"));
    }
    home_dir().map(|f| f.join(".local_issues").join("config.json"))
}
