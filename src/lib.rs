use crate::user::User;

pub mod user;

#[derive(Debug, Default)]
pub struct Issues {
    list: Vec<Issue>,
}

impl Issues {
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }

    pub fn add_new_issue(&mut self, i: Issue) {
        self.list.push(i);
    }

    pub fn get_list(&self) -> &Vec<Issue> {
        &self.list
    }

    pub fn find_from_title<T: AsRef<str>>(&mut self, s: T) -> Option<Vec<&mut Issue>> {
        let res = self
            .list
            .iter_mut()
            .filter(|f| f.name.contains(s.as_ref()))
            .collect::<Vec<_>>();
        if res.is_empty() { None } else { Some(res) }
    }

    pub fn get(&mut self, index: i32) -> Option<&mut Issue> {
        self.list.get_mut(index as usize)
    }
}

#[derive(Debug)]
pub struct Issue {
    name: String,
    status: Status,
    log: Vec<Log>,
    created_by: User,
}

impl Issue {
    pub fn new<T: AsRef<str>>(name: T, user: User) -> Self {
        Self {
            name: name.as_ref().to_string(),
            status: Status::default(),
            log: Vec::new(),
            created_by: user,
        }
    }

    pub fn close_as_cmp(&mut self) {
        self.status = Status::CloseAsCmp;
    }

    pub fn close_as_not_planed(&mut self) {
        self.status = Status::CloseAsNotPlaned;
    }

    pub fn close_as_forked(&mut self) {
        self.status = Status::CloseAsForked;
    }

    pub fn log(&mut self, new_log: Log) {
        self.log.push(new_log);
    }
}

#[derive(Debug, Default)]
pub enum Status {
    #[default]
    Open,
    CloseAsCmp,
    CloseAsNotPlaned,
    CloseAsForked,
}

#[derive(Debug)]
pub struct Log {
    content: String,
    date: chrono::DateTime<chrono::Local>,
    user: user::User,
}

impl Log {
    pub fn new<T: AsRef<str>>(content: T, user: user::User) -> Self {
        Self {
            content: content.as_ref().to_string(),
            date: chrono::Local::now(),
            user,
        }
    }
}

pub fn add(x: i32, y: i32) -> i32 {
    x + y
}
