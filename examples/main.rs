//! An example demonstrating how to use the `local_issues_lib` crate.

use local_issues_lib::{Comment, Issue, Issues, user::User};

fn main() {
    // 1. Create a new Issues collection
    let mut issues = Issues::new();
    println!("Initial issues count: {}", issues.get_list().len());

    // 2. Create a User
    let user = User::new("Alice", "alice@example.com");

    // 3. Create a new Issue
    let mut new_issue = Issue::new("Add better documentation", user.clone(), vec!["Enhance"]);

    // 4. Add a Comment to the issue
    let comment = Comment::new("I think we should add more examples.", user.clone());
    new_issue.comment(comment);

    // 5. Add the issue to the collection
    let issue_index = issues.add_new_issue(new_issue);
    println!("Added new issue at index: {}", issue_index);

    // 6. Print all issues
    println!("\nAll Issues:");
    for (i, issue) in issues.get_list().iter().enumerate() {
        println!("Issue #{}: {:?}", i, issue);
    }

    // 7. Find an issue by title
    if let Some(found_issues) = issues.find_from_title("documentation") {
        println!(
            "\nFound {} issues matching 'documentation'",
            found_issues.len()
        );
    }

    // 8. Fork an issue
    if let Some(forked_index) = issues.fork(1) {
        println!("\nForked issue #1 to new issue #{}", forked_index);
        let forked_issue = issues.get(forked_index).unwrap();
        println!("Forked issue details: {:?}", forked_issue);

        let original_issue = issues.get(1).unwrap();
        println!("Original issue status after fork: {:?}", original_issue);
    }
}
