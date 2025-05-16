use crate::{IssueTrait, Project, StatusT};
use std::fmt::Display;

#[derive(Debug, Default)]
pub enum Range {
    #[default]
    All,
    Max(u32),
}

#[derive(Debug, Default)]
pub enum ShowLevel {
    Project,
    Issues,
    #[default]
    Comments,
}

#[derive(Debug, Default)]
pub struct DisplayOptions {
    show_level: ShowLevel,
    contain_closed_issues: bool,
    // contain_closed_comments: bool,
    up_to_by_created_date: Range,
    up_to_by_latest_update: Range,
    up_to_by_due: Range,
    content: Option<Project>,
}

impl DisplayOptions {
    pub fn show_level(&mut self, level: ShowLevel) -> &mut Self {
        self.show_level = level;
        self
    }
    pub fn contain_close_issues(&mut self, contain_close_issues: bool) -> &mut Self {
        self.contain_closed_issues = contain_close_issues;
        self
    }
    // pub fn contain_close_comments(&mut self, contain_close_comments: bool) -> &mut Self {
    //     self.contain_closed_comments = contain_close_comments;
    //     self
    // }
    pub fn created_at(&mut self, up_to: Range) -> &mut Self {
        self.up_to_by_created_date = up_to;
        self
    }
    pub fn updated_at(&mut self, up_to: Range) -> &mut Self {
        self.up_to_by_latest_update = up_to;
        self
    }
    pub fn due_at(&mut self, up_to: Range) -> &mut Self {
        self.up_to_by_due = up_to;
        self
    }

    pub fn content(&mut self, content: Project) -> &mut Self {
        self.content = Some(content);
        self
    }
}

impl DisplayOptions {
    pub fn new() -> Self {
        Self {
            // content,
            // contain_issues: false,
            show_level: ShowLevel::default(),
            contain_closed_issues: false,
            // contain_commit_messages: false,
            // contain_closed_comments: false,
            up_to_by_created_date: Range::All,
            up_to_by_latest_update: Range::All,
            up_to_by_due: Range::All,
            content: None,
        }
    }

    pub fn format_display(&self) -> Option<String> {
        self.content.as_ref()?;

        let content = match &self.content {
            Some(v) => v,
            None => return None,
        };

        let oneline_issues = content
            .issues
            .iter() // only opened
            .filter(|f| {
                if self.contain_closed_issues {
                    true // because users show contained closed issues
                } else { // only opened
                    f.1.is_opened()
                }
            })
            .map(|f| {
                format!(
                    // e.g. 🟢 bug1 32 comments
                    "* {} #{} {} {}comments\n",
                    f.1.status.to_emoji(),
                    f.0,
                    f.1.name,
                    f.1.count_message()
                )
            })
            .collect::<String>();

        let contain_comment_issues = content
            .issues
            .iter()
            .map(|f| {
                format!("{} #{} {}\n{}", f.1.status.to_emoji(), f.0, f.1.name, {
                    f.1.messages
                        .0
                        .iter()
                        .map(|f| format!("  {}\n", f.message))
                        .collect::<String>()
                })
            })
            .collect::<String>();

        Some(match self.show_level {
            ShowLevel::Project => {
                format!(
                    "{}\n  issues: {}\n  created at: {}\n  updated at: {}\n  path: {}",
                    content.project_name,
                    content.count_issues(),
                    content.created_at.to_rfc2822(),
                    content.updated_at.to_rfc2822(),
                    content.project_path.to_string_lossy(),
                    // oneline_issues
                )
            }
            ShowLevel::Issues => format!(
                "{}\n  issues: {}\n  created at: {}\n  updated at: {}\n  path: {}\n\n{}",
                content.project_name,
                content.count_issues(),
                content.created_at.to_rfc2822(),
                content.updated_at.to_rfc2822(),
                content.project_path.to_string_lossy(),
                oneline_issues
            ),
            ShowLevel::Comments => format!(
                "{}\n  issues: {}\n  created at: {}\n  updated at: {}\n  path: {}\n\n{}",
                content.project_name,
                content.count_issues(),
                content.created_at.to_rfc2822(),
                content.updated_at.to_rfc2822(),
                content.project_path.to_string_lossy(),
                // oneline_issues
                contain_comment_issues
            ),
        })
    }
}

impl Display for DisplayOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format_display().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Project, ProjectManager,
        display_options::{DisplayOptions, ShowLevel},
    };

    #[test]
    fn hoge() {
        let mut pr = Project::new("name", "project_path");
        pr.add_issue("new_name");
        pr.add_commit(1, "commit_msg");
        pr.add_issue("close?");
        pr.add_commit(2, "closed?");
        pr.to_close_issue(2, false);
        pr.add_issue("not resolve");
        pr.to_close_issue(3, true);

        println!(
            "{}",
            DisplayOptions::new()
                .show_level(ShowLevel::Issues)
                // .contain_close_issues(false)
                .content(pr)
        );
    }
}
