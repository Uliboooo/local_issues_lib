//! This module provides functionality for managing users in the local issues system.

use serde::{Deserialize, Serialize};

/// A collection of users.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Users {
    list: Vec<User>,
}

impl Users {
    /// Creates a new empty collection of users.
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }

    /// Adds a new user to the collection.
    pub fn add_user(&mut self, new_user: User) {
        self.list.push(new_user.clone());
    }

    /// Finds a user by their name.
    ///
    /// Returns `Some(&User)` if found, otherwise `None`.
    pub fn find<T: AsRef<str>>(&self, name: T) -> Option<&User> {
        self.list.iter().find(|f| f.name == name.as_ref())
    }

    /// Gets a user by their index in the list.
    /// Deprecated: Use `find_by_id` instead for stable access.
    pub fn get(&self, index: i32) -> Option<&User> {
        self.list.get(index as usize)
    }

    /// Gets a mutable reference to a user by their index in the list.
    /// Deprecated: Use `find_by_id` or similar instead.
    pub fn get_mut(&mut self, index: i32) -> Option<&mut User> {
        self.list.get_mut(index as usize)
    }

    /// Checks if a user with the given name exists in the collection.
    pub fn exist<T: AsRef<str>>(&self, name: T) -> bool {
        self.find(name).is_some()
    }

    /// Returns a reference to the underlying list of users.
    pub fn get_list(&self) -> &Vec<User> {
        &self.list
    }
}

impl easy_storage::Storeable for Users {}

/// Represents a user in the system.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct User {
    name: String,
    email: String,
    deactivate: bool,
}

impl easy_storage::Storeable for User {}

impl User {
    /// Creates a new user with the given name and email.
    ///
    /// The user is active by default and assigned a random UUID.
    pub fn new<T: AsRef<str>, U: AsRef<str>>(name: T, email: U) -> Self {
        Self {
            name: name.as_ref().to_string(),
            email: email.as_ref().to_string(),
            deactivate: false,
        }
    }

    /// Deactivates the user.
    pub fn deactivate(&mut self) {
        self.deactivate = true;
    }

    /// Returns name
    pub fn name(&self) -> String {
        self.name.clone()
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
