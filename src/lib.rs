//! ## Example
//!
//! - 追記機能を導入予定
//! - インタラクティブモードを導入予定
//!
// TODO: Id structの導入はやめ。jsonなら1s未満で数千件から数万件は処理できそうなのでそちらに頼る。1プロジェクトで数万件を超えるisuesは扱わないと想定するためÏ

mod db;

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
// use sled::open;
use std::{
    collections::HashMap,
    fmt::Debug,
    io,
    path::{Path, PathBuf},
    str,
};

#[derive(Debug)]
pub enum Error {
    DbNotFound,
    IoError(io::Error),
    SerdeError(serde_json::Error),
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, PartialOrd, Eq)]
pub enum Status {
    #[default]
    Open,
    Closed(Closed),
    /// count of delted this issue
    MarkedAsDelete(i32),
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, PartialOrd, Eq)]
pub enum Closed {
    #[default]
    Resolved,
    NotResolved,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Issue {
    title: String,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
    due_date: Option<DateTime<Local>>,
    status: Status,
    tags: Option<Vec<String>>,
    body_path: PathBuf,
}

impl Issue {
    pub fn new<S: AsRef<str>, P: AsRef<Path>>(
        title: S,
        due_date: Option<DateTime<Local>>,
        status: Status,
        tags: Option<Vec<String>>,
        body_path: P,
    ) -> Self {
        let now = Local::now();
        Self {
            title: title.as_ref().to_string(),
            created_at: now,
            updated_at: now,
            due_date,
            status,
            tags,
            body_path: body_path.as_ref().to_path_buf(),
        }
    }

    /// edit title by arg title: String
    fn edit_title(&mut self, title: String) {
        self.title = title
    }

    /// update `updated_at` by now time
    fn update_date(&mut self) {
        self.updated_at = Local::now();
    }

    /// edit `due_date` by arg
    fn edit_due_date(&mut self, new_due: DateTime<Local>) {
        self.due_date = Some(new_due);
    }

    /// edit status
    fn edit_status(&mut self, new_status: Status) {
        self.status = new_status;
    }

    fn clear_tags(&mut self) {
        self.tags = None;
    }

    fn get_tags(&mut self) -> Option<Vec<String>> {
        self.tags.clone()
    }

    /// self.tagsがSomeの場合にのみ`new_tags`をappend
    fn add_tag(&mut self, mut new_tags: Vec<String>) {
        if let Some(v) = &mut self.tags {
            v.append(&mut new_tags);
        }
    }

    // /// if status is open, change to closed.if it is `MarkedAsdelete`, reset count(default: 10).
    // fn update_status(&mut self) {
    //     self.status = match self.status {
    //         Status::Open => Status::Closed(C),
    //         Status::Closed => Status::Archived,
    //         Status::Archived => Status::MarkedAsdelete(10),
    //         Status::MarkedAsdelete(_) => Status::MarkedAsdelete(10),
    //     };
    // }

    fn is_delete_marked(&self) -> bool {
        matches!(self.status, Status::MarkedAsDelete(_))
    }

    fn delete_flag_is_zero(&self) -> bool {
        match self.status {
            Status::MarkedAsDelete(i) => i == 0,
            _ => false,
        }
    }

    fn delete_flag_count(&self) -> Option<i32> {
        match self.status {
            Status::MarkedAsDelete(c) => Some(c),
            _ => None,
        }
    }

    fn set_delete_count(&mut self, new_count: i32) -> Option<i32> {
        match self.status {
            Status::MarkedAsDelete(_) => {
                self.status = Status::MarkedAsDelete(new_count);
                if let Status::MarkedAsDelete(c) = self.status {
                    Some(c)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn reset_delete_count(&mut self) -> Option<i32> {
        self.set_delete_count(10)
    }

    /// edit body path
    fn edit_body_path(&mut self, new_path: PathBuf) {
        self.body_path = new_path;
    }

    /// decrement delete flag count
    fn decrement_delete_count(&mut self) {
        if let Status::MarkedAsDelete(c) = self.status {
            self.status = Status::MarkedAsDelete(c - 1)
        }
    }
}

// pub trait ManageIssue {
//     // manage paroject
//     fn open<S: AsRef<str>, P: AsRef<Path>>(title: S, path: P) -> Result<Project, Error>;
//     fn save(&self) -> Result<(), Error>;

//     // manage issues
//     /// when title exact match, return `Some(id: u64)`.
//     fn get_id_from_title<S: AsRef<str>>(&self, title: S) -> Option<u64>;
//     /// return ids(Vec<u64>) when matched status.
//     fn get_ids_from_status<S: AsRef<str>>(&self, status: Status) -> Option<Vec<u64>>;

//     fn search_ids_from_title<S: AsRef<str>>(&self, title: S) -> Option<>

//     /// add issue
//     fn remove_issue_from_id(&mut self, id: u64);
//     fn pop_issue(&mut self, id: u64) -> Option<Issue>;

//     /// add issue
//     //save()で失敗する可能性があるため、Result<>.
//     fn add_issue(&self, new_issue: Issue) -> Result<(), Error>;
//     fn remove_issue<S: AsRef<str>>(&mut self, title: S) {
//         let id = self.get_id_from_title(title);
//         if let Some(f) = id {
//             self.remove_issue_from_id(f);
//         }
//     }

//     fn get_from_title<S: AsRef<str>>(&self, target_title: S) -> Result<Option<FetchedList>, Error>;
// }

// pub trait PowerManageIssue: ManageIssue {
//     fn add_new_issue(&mut self, new_issue: Issue) -> Result<(), Error> {
//         self.add_issue(new_issue)
//     }
//     fn remove_issue<S: AsRef<str>>(&mut self, title: S)
//     {
//         let id = self.get_id_from_title(title);
//         if let Some(f) = id {
//             self.remove_issue_from_id(f);
//         }
//     }
// }

// pub trait ManageTag {
//     fn add_tags(&mut self, new_tags: &mut Vec<String>);
//     fn remove_tag(&mut self, tag_names: Vec<String>);
//     fn get_tags(&mut self) -> Vec<String>;
//     fn check_delete_flag(&mut self) -> i32;
// }

pub enum MatchType {
    Exact,
    Partial,
    Not,
}
impl MatchType {
    fn to_bool(&self) -> bool {
        !matches!(self, MatchType::Not)
    }
}

/// 部分一致の場合は、`Some(item or item2)`
/// 0 is item、1 is item2
fn compare_titles<S: AsRef<str> + Eq>(item: &S, item2: &S) -> (MatchType, Option<u8>) {
    if item.as_ref() == item2.as_ref() {
        (MatchType::Exact, None)
    } else if item.as_ref().contains(item2.as_ref()) {
        (MatchType::Partial, Some(0))
    } else if item2.as_ref().contains(item.as_ref()) {
        (MatchType::Partial, Some(1))
    } else {
        (MatchType::Not, None)
    }
}

pub type BodyData = HashMap<u64, Issue>;
pub type FetahedItem = (MatchType, BodyData);
pub type FetchedList = Vec<FetahedItem>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub project_name: String,
    pub work_path: PathBuf,
    pub db_path: PathBuf,
    pub body: BodyData,
    current_id: u64,
    pub tags: Vec<String>,
}

impl Project {
    pub fn open<S: AsRef<str>, P: AsRef<Path>>(title: S, path: P) -> Result<Self, Error> {
        let db_path = path.as_ref().join("db").with_extension("toml");
        let db = {
            if db_path.exists() {
                return db::load_db(&path);
            } else {
                // like a new()
                // when file isn't exist, init Project as id = 0
                let void_body = Project {
                    project_name: title.as_ref().to_string(),
                    work_path: path.as_ref().to_path_buf(),
                    db_path,
                    body: HashMap::new(),
                    current_id: 0,
                    tags: Vec::new(),
                };
                db::save(void_body)?;
            }
            db::load_db(&path)
        }?;
        Ok(db)
    }

    pub fn save(mut self) -> Result<(), Error> {
        self.purge_delete_mated_issues();
        self.ready_delete_flags();
        db::save(self)?;
        Ok(())
    }

    /// MarkedAsdeleteのチェックなしで保存
    /// また、デクリメント処理もしない
    /// **カウントが実際の操作とズレる可能性があるため注意**
    pub fn save_without_delete(self) -> Result<(), Error> {
        db::save(self)?;
        Ok(())
    }

    /// current_idをインクリメントして挿入
    fn insert(&mut self, new_issue: Issue) {
        let new_id = self.current_id + 1;
        self.body.insert(new_id, new_issue);
        self.current_id = new_id;
    }

    /// タイトルが完全一致したidを`Option<Vec<u64>>`で返す
    pub fn get_id_with_exact<S: AsRef<str>>(&self, title: &S) -> Option<Vec<u64>> {
        let res = self
            .body
            .iter()
            .filter(|f| f.1.title == title.as_ref())
            .map(|f| *f.0)
            .collect::<Vec<u64>>();
        if res.is_empty() { None } else { Some(res) }
    }

    /// タイトルが部分一致したidを`Option<Vec<u64>>`で返す
    pub fn get_id_with_partial<S: AsRef<str>>(&self, title: &S) -> Option<Vec<u64>> {
        let res = self
            .body
            .iter()
            .filter(|f| f.1.title.contains(title.as_ref()) || title.as_ref().contains(&f.1.title))
            .map(|f| *f.0)
            .collect::<Vec<u64>>();
        if res.is_empty() { None } else { Some(res) }
    }

    /// Project.bodyに`new_issue`を追加(idのインクリメントは自動)
    pub fn add_issue(&mut self, new_issue: Issue) {
        self.insert(new_issue);
    }

    // idを元にissueを削除
    pub fn remove_issue(&mut self, id: &u64) -> Option<Issue> {
        self.body.remove(id)
    }

    /// 完全一致が一つだった場合にのみ削除  
    /// Noneの場合は削除した項目なし
    pub fn remove_issue_from_title<S: AsRef<str>>(&mut self, title: S) -> Option<Issue> {
        match self.get_id_with_exact(&title) {
            Some(i) => {
                if i.len() == 1 {
                    match i.first() {
                        Some(i) => self.remove_issue(i),
                        None => None,
                    }
                } else {
                    None
                }
            }
            None => None,
        }
    }

    /// 完全一致した内容を全て削除  
    /// Noneの場合は削除した項目なし
    pub fn remove_all_issue_from_title<S: AsRef<str>>(&mut self, title: S) {
        let res = self.get_id_with_exact(&title);
        if let Some(c) = res {
            for i in c {
                self.remove_issue(&i);
            }
        }
    }

    /// idに対応するissueのタイトルを変更
    pub fn edit_issue_title<S: AsRef<str>>(&mut self, id: u64, new_title: S) -> Option<()> {
        self.body
            .get_mut(&id)
            .map(|issue| issue.edit_title(new_title.as_ref().to_string()))
    }

    /// `new_tag<S>`をidを元にIssueへ追記
    pub fn add_issue_tag<S: AsRef<str>>(&mut self, id: u64, new_tag: Vec<S>) -> Option<()> {
        self.body
            .get_mut(&id)
            .map(|issue| issue.add_tag(new_tag.iter().map(|f| f.as_ref().to_string()).collect()))
    }

    /// idを元にissueのtagをクリア
    pub fn clear_issue_tag(&mut self, id: u64) -> Option<()> {
        self.body.get_mut(&id).map(|is| is.clear_tags())
    }

    /// idを元にissueのstatusを変更
    pub fn edit_issue_status<S: AsRef<str>>(&mut self, id: u64, status: Status) -> Option<()> {
        self.body.get_mut(&id).map(|issue| issue.status = status)
    }

    /// idを元にissueの`due date`を変更
    pub fn edit_issue_due(&mut self, id: u64, due: DateTime<Local>) -> Option<()> {
        self.body.get_mut(&id).map(|f| f.edit_due_date(due))
    }

    /// add tags to self.tags from arg: `Vec<String>`
    pub fn add_tags(&mut self, new_tags: &mut Vec<String>) {
        self.tags.append(new_tags);
    }

    /// remove tags from self.tags, by tag_names: `Vec<String>`
    pub fn remove_tag(&mut self, tag_names: Vec<String>) {
        // fがtag_namesに含まれている場合は削除される。
        self.tags.retain(|f| tag_names.iter().any(|t| t != f));
    }

    /// return cloned current tags: `Vec<String>`
    pub fn get_tags(&mut self) -> Vec<String> {
        self.tags.clone()
    }

    /// デクリメント処理
    fn ready_delete_flags(&mut self) {
        for i in self.body.iter_mut() {
            i.1.decrement_delete_count();
        }
    }

    /// incomplete
    fn delete_flaged_ids(&self) -> Option<Vec<u64>> {
        let delete_marked_ids = self
            .body
            .iter()
            .filter(|f| f.1.is_delete_marked())
            .map(|f| *f.0)
            .collect::<Vec<u64>>();
        if delete_marked_ids.is_empty() {
            None
        } else {
            Some(delete_marked_ids)
        }
    }

    fn purge_delete_mated_issues(&mut self) -> Option<()> {
        let ids = self.delete_flaged_ids()?;
        for i in ids {
            self.remove_issue(&i);
        }
        Some(())
    }
}

#[cfg(test)]
mod tests {}
