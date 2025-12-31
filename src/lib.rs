use serde::{Deserialize, Serialize};

use crate::user::User;

pub mod user;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
/// A collection of issues.
pub struct Issues {
    list: Vec<Issue>,
}

impl easy_storage::Storeable for Issues {}

impl Issues {
    /// Creates a new `Issues` collection.
    ///
    /// This initializes the collection with a "root" issue.
    pub fn new() -> Self {
        let now = chrono::Local::now();
        let root_issue = Issue {
            name: "root".to_string(),
            status: Status::CloseAsCmp,
            comment: Vec::new(),
            created_by: User::new("root", "root"),
            from: usize::MAX,
            created_at: now,
            updated_at: now,
        };
        Self {
            list: vec![root_issue],
        }
    }

    /// Adds a new issue to the collection.
    ///
    /// Returns the index of the newly added issue.
    pub fn add_new_issue(&mut self, i: Issue) -> usize {
        let l = self.list.len();
        self.list.push(i);
        l
    }

    /// Returns a reference to the list of issues.
    pub fn get_list(&self) -> &Vec<Issue> {
        &self.list
    }

    /// Finds issues containing the given title string.
    ///
    /// Returns `Some(Vec<&mut Issue>)` if matches are found, otherwise `None`.
    pub fn find_from_title<T: AsRef<str>>(&mut self, s: T) -> Option<Vec<&mut Issue>> {
        let res = self
            .list
            .iter_mut()
            .filter(|f| f.name.contains(s.as_ref()))
            .collect::<Vec<_>>();
        if res.is_empty() { None } else { Some(res) }
    }

    pub fn find_from_updated_time(
        &mut self,
        st: chrono::DateTime<chrono::Local>,
        ed: chrono::DateTime<chrono::Local>,
    ) -> Option<Vec<&mut Issue>> {
        let res = self
            .list
            .iter_mut()
            .filter(|f| (st..=ed).contains(&f.updated_at))
            .collect::<Vec<_>>();
        if res.is_empty() { None } else { Some(res) }
    }

    pub fn find_from_created_time(
        &mut self,
        st: chrono::DateTime<chrono::Local>,
        ed: chrono::DateTime<chrono::Local>,
    ) -> Option<Vec<&mut Issue>> {
        let res = self
            .list
            .iter_mut()
            .filter(|f| (st..=ed).contains(&f.created_at))
            .collect::<Vec<_>>();
        if res.is_empty() { None } else { Some(res) }
    }

    pub fn find_from_comments<T: AsRef<str>>(&mut self, s: T) -> Option<Vec<&mut Issue>> {
        let res = self
            .list
            .iter_mut()
            .filter(|f| f.contains_comment(s.as_ref()))
            .collect::<Vec<_>>();
        if res.is_empty() { None } else { Some(res) }
    }

    /// Gets an issue by its index.
    pub fn get(&self, index: usize) -> Option<&Issue> {
        self.list.get(index)
    }

    /// Gets a mutable reference to an issue by its index.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Issue> {
        self.list.get_mut(index)
    }

    /// Forks an issue from a given index.
    ///
    /// The original issue is marked as `CloseAsForked`, and a new copy is created
    /// with `from` set to the original index.
    ///
    /// Returns `Some(usize)` which is the index of the new forked issue, or `None` if the original issue doesn't exist.
    pub fn fork(&mut self, from: usize) -> Option<usize> {
        let mut forked = self.get(from)?.clone();
        forked.from = from;
        self.get_mut(from).unwrap().status = Status::CloseAsForked;
        Some(self.add_new_issue(forked))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a single issue.
pub struct Issue {
    name: String,
    status: Status,
    comment: Vec<Comment>,
    created_by: User,
    created_at: chrono::DateTime<chrono::Local>,
    updated_at: chrono::DateTime<chrono::Local>,
    from: usize,
}

impl Issue {
    /// Creates a new issue with a name and the user who created it.
    pub fn new<T: AsRef<str>>(name: T, user: User) -> Self {
        let now = chrono::Local::now();
        Self {
            name: name.as_ref().to_string(),
            status: Status::default(),
            comment: Vec::new(),
            created_by: user,
            from: 0,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update(&mut self) {
        self.updated_at = chrono::Local::now();
    }

    /// Closes the issue as completed.
    pub fn close_as_cmp(&mut self) {
        self.status = Status::CloseAsCmp;
        self.update();
    }

    /// Closes the issue as not planned.
    pub fn close_as_not_planed(&mut self) {
        self.status = Status::CloseAsNotPlaned;
        self.update();
    }

    /// Closes the issue as forked.
    pub fn close_as_forked(&mut self) {
        self.status = Status::CloseAsForked;
        self.update();
    }

    /// Adds a comment entry to the issue.
    pub fn comment(&mut self, new_comment: Comment) {
        self.comment.push(new_comment);
        self.update();
    }

    pub fn contains_comment<T: AsRef<str>>(&self, s: T) -> bool {
        let res = self
            .comment
            .iter()
            .filter(|f| f.content.contains(s.as_ref()))
            .collect::<Vec<_>>();
        !res.is_empty()
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
/// Represents the status of an issue.
pub enum Status {
    #[default]
    /// The issue is open.
    Open,
    /// The issue is closed as completed.
    CloseAsCmp,
    /// The issue is closed as not planned.
    CloseAsNotPlaned,
    /// The issue is closed because it was forked.
    CloseAsForked,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a comment entry associated with an issue.
pub struct Comment {
    content: String,
    date: chrono::DateTime<chrono::Local>,
    user: user::User,
}

impl Comment {
    /// Creates a new comment entry with content and the user who created it.
    ///
    /// The date is set to the current local time.
    pub fn new<T: AsRef<str>>(content: T, user: user::User) -> Self {
        Self {
            content: content.as_ref().to_string(),
            date: chrono::Local::now(),
            user,
        }
    }
}
