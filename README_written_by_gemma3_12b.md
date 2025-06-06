# local issues lib

**There is a high possibility that many breaking changes will occur before reaching version 1.0.**

## Overview

`local issues lib` is a Rust library that provides local issue management similar to GitHub's issue tracking. This allows you to manage tasks and bug reports without relying on external services.

## Features

- **Project Management**:  Manage projects with their own issues.
- **Issue Creation**: Create issues with titles, descriptions, due dates, and assignees.
- **Message History**: Maintain a history of comments and messages for each issue.
- **Status Tracking**: Track the status of issues (Open, Closed - Resolved/Unresolved).
- **Search Functionality**: Search for issues by title.
- **JSON Persistence**: Data is stored in local JSON files.
- **Users and Authors**: Support for managing users and assigning authorship to issues.
- **Configuration**: Define project-specific settings.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
local_issues_lib = "0.5.0"
```

## Example

```rust
use std::env;

use local_issues_lib::{Project, ProjectManager, config::Config, display_options::DisplayOptions};

fn main() -> Result<(), local_issues_lib::Error> {
    let project_path = env::current_dir().unwrap().join("examples").join("project");
    let mut config = Config::new(); // to actually use it, save it to a file with the load function and then use the
    config.change_current_user("test_user");

    let current_user = config.get_current_user().unwrap();

    let mut p = Project::open_or_create(&project_path, "example_project")?;

    // 👇's id is 1
    p.add_issue("issue1", current_user.clone());
    p.add_comment(1, "first comment", current_user.clone());
    p.save()?;

    let mut p = Project::open(&project_path)?;
    p.add_comment(1, "second comment", current_user.clone());

    p.add_issue("will close by resolve", "test_author");
    p.add_issue("will close by unresolved", "test_author");

    p.to_close_issue(2, true);
    p.to_close_issue(3, false);

    println!(
        "{}",
        DisplayOptions::new().contain_close_issues(true).content(&p)
    );
    p.save()?;
    Ok(())
}
```

## Configuration

- **Version:** `String` (Maintains version consistency)
- **Users:** `[User]` (List of registered users)

## Roadmap

- [x] Basic feature implementation
  - [x] Project management
    - [x] Open project
    - [x] Save project
    - [x] Rename project
  - [x] Edit project info
    - [x] Rename
  - [x] Search issues and comments
  - [x] Manage issues
    - [x] Add issue
    - [x] Rename issue
    - [x] Edit due date
    - [x] Open issue
    - [x] Close issue
    - [x] Remove issue
  - [x] Manage comments
    - [x] Add comment
    - [x] Remove comment
    - [x] Show comment
    - [x] Hide comment
- [ ] Users and Authors (for AI functions)
- [ ] Implement tag and priority
- [ ] Implement display options
- [ ] Stabilize API

## Project Structure

- Project
    - `project_name`
    - `current_id`
    - `created_at`
    - `updated_at`
    - `project_path`
    - `storage_path`
    - `db_path`
    - `users`
        - `list`: `HashMap\<Uuid, User\>`
            - `User`: `{name, id: Uuid}`
    - `issues`: `HashMap\<u64, Issue\>`
        - `u64`: `id`
        - `Issue`
            - `name`
            - `created_at`
            - `updated_at`
            - `due_date`: `Option`
            - `author`: `User`
            - `assigned_member`: `Users`
            - `status`
                - `Open`
                - `Closed`
                    - `Resolved`
                    - `Unresolved`
            - `messages`
                - `list`: `Vec\<Message\>`
                    - `Message`
                        - `message`: `String`
                        - `show`: `bool`
                        - `created_at`
                        - `author`: `User`

## Contributing

Contributions are welcome! Please report bugs, suggest features, and submit pull requests.

## License

This project is licensed under the MIT or Apache 2.0 License. See the `LICENSE` file for details.

I have saved the README content as a markdown string.  Do you want me to:

1. Save this content to a file named `README.md`?
2. Provide any other modifications to the content?
3. Something else?
