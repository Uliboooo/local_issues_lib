#[derive(Debug, Default)]
pub struct Users {
    list: Vec<User>,
}

impl Users {
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }

    pub fn add_user(&mut self, new_user: User) {
        self.list.push(new_user.clone());
    }

    pub fn find<T: AsRef<str>>(&self, name: T) -> Option<&User> {
        self.list.iter().find(|f| f.name == name.as_ref())
    }

    pub fn get(&self, index: i32) -> Option<&User> {
        self.list.get(index as usize)
    }

    pub fn get_mut(&mut self, index: i32) -> Option<&mut User> {
        self.list.get_mut(index as usize)
    }

    pub fn exist<T: AsRef<str>>(&self, name: T) -> bool {
        self.find(name).is_some()
    }

    pub fn get_list(&self) -> &Vec<User> {
        &self.list
    }
}

#[derive(Debug, Default)]
pub struct User {
    name: String,
    email: String,
    deactivate: bool,
}

impl User {
    pub fn new<T: AsRef<str>, U: AsRef<str>>(name: T, email: U) -> Self {
        Self {
            name: name.as_ref().to_string(),
            email: email.as_ref().to_string(),
            deactivate: false,
        }
    }

    pub fn deactivate(&mut self) {
        self.deactivate = true;
    }
}

impl Clone for User {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            email: self.email.clone(),
            deactivate: self.deactivate,
        }
    }
}
