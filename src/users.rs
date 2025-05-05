use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    name: String,
    id: Uuid,
}

impl User {
    fn new<T: AsRef<str>>(name: T) -> Self {
        Self {
            name: name.as_ref().to_string(),
            id: Uuid::new_v4(),
        }
    }
}

pub trait ManageUsers {
    fn add_user<T: AsRef<str>>(&mut self, name: T) -> User;
    fn rm_user(&mut self, id: Uuid);
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Users {
    list: HashMap<Uuid, User>,
}

impl Users {
    pub fn new() -> Self {
        Self {
            list: HashMap::new(),
        }
    }
}

impl ManageUsers for Users {
    fn add_user<T: AsRef<str>>(&mut self, name: T) -> User {
        let user = User::new(name);
        self.list.insert(user.id, user.clone());
        user
    }

    fn rm_user(&mut self, id: Uuid) {
        self.list.remove(&id);
    }
}
