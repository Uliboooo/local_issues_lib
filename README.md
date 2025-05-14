# local issues lib

**There is a high possibility that many breaking changes will occur before reaching version 1.0.**

## notes

* printer module is still incomplete.

more tasks [here](https://github.com/Uliboooo/local_issues_lib/blob/main/Tasks.md)

## Local Issues Lib

this lib provides feature that manage issues like a ["github issue"](https://github.com/Uliboooo/local_issues_lib/issues).

## Config

save to `home_dir/.local_issues/config.json`.

details info is in [here](https://github.com/Uliboooo/local_issues_lib/blob/main/reference/Config.md)

## RoadMap

### v1 ~

* [ ] basic feats -> 
* [ ] `Users` and `author` for AI functions to be implemented in the future
* [ ] feat: Config

### v2 ~

* [ ] feat: save to md file. 
* [ ] feat: Printer struct.
* [ ] feat: AI team member by `Users`.
* [ ] feat: Tags

## Project Structure

maybe, this info is old...

- Project
  - name: `String`
  - issues: `HashMap<u64, Issue>`
    - Issue
        - name: `String`
        - messages: `Vec<Message>`
            - Message
                - message: `String`
                - show: `bool`
                - created_at: `DateTime<Local>`
        - status
            - Open
            - Closed
                - Resolved
                - UnResolved
        - created_at: `DateTime<Local>`
        - updated_at: `DateTime<Local>`
        - due_date: `DateTime<Local>`
  - created_at: `DateTime<Local>`
  - updated_at: `DateTime<Local>`
  - project_path: `PathBuf`
  - storage_path: `PathBuf`
  - db_path: `PathBuf`
