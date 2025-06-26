use std::env;

use local_issues_lib::{config::Config, display_options::DisplayOptions, storage::Storage, Error, Project, ProjectManager};

fn main() -> Result<(), local_issues_lib::Error> {
    let project_dir_path = env::current_dir().unwrap().join("examples").join("project");
    let mut config = Config::new(); // to actually use it, save it to a file with the load function and then use the
    config.change_current_user("test_user");

    let current_user = config.get_current_user().unwrap();

    // let mut p = Project::open_or_create(&project_path, "example_project")?;
    let mut p = Project::open::<Project>(&project_dir_path).unwrap();

    // 👇's id is 1
    p.add_issue("issue1", current_user.clone());
    p.add_comment(1, "first comment", current_user.clone());
    p.save(project_dir_path, true);

    let mut p = Project::open::<Project>(&project_dir_path).unwrap();
    p.add_comment(1, "second comment", current_user.clone());

    p.add_issue("will close by resolve", "test_author");
    p.add_issue("will close by unresolved", "test_author");

    p.to_close_issue(2, true);
    p.to_close_issue(3, false);

    println!(
        "{}",
        DisplayOptions::new().contain_close_issues(true).content(&p)
    );
    p.save(project_dir_path.join("a"), true).unwrap();
    Ok(())
}
