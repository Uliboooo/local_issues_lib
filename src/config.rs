use crate::users::{ManageUsers, User, Users};
use crate::{VERSION, db};
use home::home_dir;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, path::PathBuf};
use uuid::{self, Uuid};

#[derive(Debug)]
pub enum Error {
    DbError(db::Error),
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Config {
    lib_version: String,
    users: Users,
}

impl ManageUsers for Config {
    fn add_user<T: AsRef<str>>(&mut self, name: T) -> User {
        self.users.add_user(name)
    }

    fn rm_user(&mut self, id: Uuid) {
        self.users.rm_user(id);
    }
}

impl Config {
    pub fn new() -> Self {
        Self {
            users: Users::new(),
            lib_version: VERSION.to_string(),
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
    }
    pub fn save(&self) {
        db::save(self, get_config_path().unwrap()).unwrap();
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
    home_dir().map(|f| f.join(".local_issues_lib").join("config.json"))
}

#[cfg(test)]
mod tests {
    use crate::config::ManageUsers;

    use super::Config;

    #[test]
    fn test_config_load() {
        let config = Config::load();
        println!("{:?}", config);
    }

    #[test]
    fn test_config_save() {
        let mut new_config = Config::new();
        new_config.add_user("name");
        new_config.add_user("name1");
        new_config.save();

        let loaded = Config::load();
        println!("{}", loaded.unwrap());
    }
}
