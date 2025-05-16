use std::env;

use local_issues_lib::{Project, ProjectManager};

fn main() -> Result<(), local_issues_lib::Error> {
    let project_path = env::current_dir().unwrap().join("examples").join("project");

    let mut p = Project::open_or_create(&project_path, "name")?;

    p.add_issue("new_name");
    p.save()?;

    let mut p = Project::open(&project_path)?;
    p.add_commit(1, "commit_msg");
    for i in 0..30 {
        p.add_commit(1, i.to_string());
    }

    println!("{}", p);
    p.save()?;
    Ok(())
}
