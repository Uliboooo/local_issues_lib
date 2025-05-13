use crate::users::{User, Users};
use crate::{VERSION, storage};
use derive_getters::Getters;
use home::home_dir;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, path::PathBuf};

#[derive(Debug)]
pub enum Error {
    DbError(storage::Error),
}

#[derive(Debug, Deserialize, Serialize, Getters, Default)]
pub struct Config {
    version: String,
    users: Option<Users>,
    current_user: Option<User>,
}

impl Config {
    /// Specify only required options as arguments, and initialize all optional values ​​to `None`.
    pub fn new() -> Self {
        Self {
            version: VERSION.to_string(),
            users: None,
            current_user: None,
        }
    }

    /// when config not found, return Error without creating.
    pub fn load() -> Result<Self, Error> {
        storage::load(get_config_path().unwrap(), true).map_err(Error::DbError)
    }

    pub fn load_or_create() -> Result<Self, Error> {
        storage::load(get_config_path().unwrap(), true)
            .or_else(|e| {
                if e.is_file_is_zero() {
                    Ok(Config::new())
                } else {
                    Err(e)
                }
            })
            .map_err(Error::DbError)
    }
    pub fn save(&self) {
        storage::save(self, get_config_path().unwrap()).unwrap();
    }
}

impl Config {
    /// this function don't save() automatically.please use `.save()`  
    ///
    /// ⚠️this function only replace users list.
    /// more detailed operations (add, rm, and etc) are performed by getting the `Users` and using its methods.
    pub fn over_write_user_list(&mut self, new_users: Option<Users>) {
        self.users = new_users;
    }

    pub fn get_mut_users(&mut self) -> Option<Users> {
        self.users.clone()
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

#[cfg(test)]
mod tests {
    use super::Config;
    use crate::Users;

    #[test]
    fn getter_test() {
        let config = Config::new();
        let a = config.version();
        println!("{}", a);
    }

    #[test]
    fn change_user_list() {
        let mut config = Config::new();
        let mut user_list = Users::new();
        user_list.add_user("name");
        config.over_write_user_list(Some(user_list));

        let mut get_list_from_config = config.get_mut_users();
        if let Some(f) = get_list_from_config.as_mut() {
            f.add_user("name2")
        }

        config.over_write_user_list(get_list_from_config);
    }
}
