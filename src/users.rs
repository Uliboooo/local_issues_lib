use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display};
use uuid::Uuid;

// trait IsUser {}
// impl IsUser for User {}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    name: String,
    id: Uuid,
}

impl<T: AsRef<str>> From<T> for User {
    fn from(value: T) -> Self {
        User::new(value)
    }
}

impl User {
    pub fn new<T: AsRef<str>>(name: T) -> Self {
        Self {
            name: name.as_ref().to_string(),
            id: Uuid::new_v4(),
        }
    }
}

impl Display for User {
    /// end is line break
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "id: {}, name: {}", self.id, self.name)
    }
}

// pub trait ManageUsers {
//     fn add_user<T: Into<User>>(&mut self, name: T);
//     fn rm_user(&mut self, id: Uuid);
//     fn users_list(&self) -> Vec<String>;
// }

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
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

impl Users {
    pub fn add_user<T: Into<User>>(&mut self, name: T) {
        // let user = User::new(name);
        let user = name.into();
        self.list.insert(user.id, user.clone());
    }

    pub fn rm_user(&mut self, id: Uuid) {
        self.list.remove(&id);
    }

    pub fn users_list(&self) -> Vec<String> {
        let name_list = self
            .list
            .iter()
            //            u.0 is unnecessary because u.1 contains both id and name
            //           & line break is unnecessary because u.1 contains it.
            .map(|u| format!("{}", u.1))
            .collect::<Vec<String>>();
        name_list
    }
}

#[cfg(test)]
mod tests {
    use super::Users;

    #[test]
    fn test_manage_users() {
        let mut a = Users::new();
        a.add_user("name");

        println!("{:?}", a);
    }
}
