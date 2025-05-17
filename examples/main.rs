use std::env;

use local_issues_lib::{Project, ProjectManager, display_options::DisplayOptions};

fn main() -> Result<(), local_issues_lib::Error> {
    let project_path = env::current_dir().unwrap().join("examples").join("project");

    let mut p = Project::open_or_create(&project_path, "example_project")?;

    // 👇's id is 1
    p.add_issue("issue1");
    p.add_comment(1, "first comment");
    p.save()?;

    let mut p = Project::open(&project_path)?;
    p.add_comment(1, "second comment");

    p.add_issue("will close by resolve");
    p.add_issue("will close by unresolved");

    p.to_close_issue(2, true);
    p.to_close_issue(3, false);

    println!(
        "{}",
        DisplayOptions::new().contain_close_issues(true).content(&p)
    );
    p.save()?;
    Ok(())
}
