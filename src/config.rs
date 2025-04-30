use crate::db;
use home::home_dir;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug)]
pub enum Error {
    DbError(db::Error),
}

#[derive(Debug, Deserialize, Serialize)]
struct User {
    name: String,
    // id: u64,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Users {
    list: HashMap<String, User>,
    // current_id: u64,
}

impl Users {
    fn new() -> Self {
        Self {
            list: HashMap::new(),
            // current_id: 0,
        }
    }

    fn add_user<S: AsRef<str>>(&mut self, user_name: S) {
        // self.current_id += 1;
        self.list.insert(
            user_name.as_ref().to_string(),
            User {
                name: user_name.as_ref().to_string(),
                // id: self.current_id,
            },
        );
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Config {
    users: Users,
}

impl Config {
    pub fn add_user<T: AsRef<str>>(&mut self, name: T) {
        self.users.add_user(name);
    }
    pub fn new() -> Self {
        Self {
            users: Users::new(),
        }
    }
    pub fn load() -> Result<Self, Error> {
        match db::load(get_config_path().unwrap(), true) {
            Ok(v) => Ok(v),
            Err(e) => {
                if e.is_file_is_zero() {
                    Ok(Self::default())
                } else {
                    Err(Error::DbError(e))
                }
            }
        }
        // a
    }
    pub fn save(&self) {
        db::save(self, get_config_path().unwrap()).unwrap();
    }
}

fn get_config_path() -> Option<PathBuf> {
    if cfg!(test) {
        return Some(PathBuf::from("test/config_test/").join("config.json"));
    }
    home_dir().map(|f| f.join(".local_issues_lib").join("config.json"))
}

#[cfg(test)]
mod tests {
    use super::Config;

    #[test]
    fn test_config_load() {
        let config = Config::load();
        println!("{:?}", config);
    }

    #[test]
    fn test_config_save() {
        let mut new_config = Config::new();
        new_config.add_user("hoge");
        new_config.add_user("hogee");
        new_config.save();

        let loaded = Config::load();
        println!("{:?}", loaded);
    }
}
