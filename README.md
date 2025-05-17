# local issues lib

**There is a high possibility that many breaking changes will occur before reaching version 1.0.**

## notes

tasks [here](https://github.com/Uliboooo/local_issues_lib/blob/main/Tasks.md)

## Local Issues Lib

this lib provides feature that manage issues like a ["github issue"](https://github.com/Uliboooo/local_issues_lib/issues).

## Config

save to `home_dir/.local_issues/config.json`.

details info is in [here](https://github.com/Uliboooo/local_issues_lib/blob/main/reference/Config.md)

## RoadMap

### v1 ~

* [ ] basic feats
    * [ ] manage project
        * [ ] open
        * [ ] save
    * [ ] edit project info
        * [ ] rename
    * [ ] search issues and comments
    * [ ] manage issues
        * [ ] add issue
        * [ ] rename issue
        * [ ] edit due
        * [ ] open issue
        * [ ] close issue
        * [ ] remove issue
    * [ ] manage comments
        * [ ] add comment
        * [ ] remove comment
        * [ ] show comment
        * [ ] hide comment
* [ ] `Users` and `author` for AI functions(v2~)
* [ ] feat: Config
* [ ] Display by Options

### v2 ~

* [ ] feat: export to md file. 
* [ ] feat: AI team member as `User`.
* [ ] feat: Tags
* [ ] feat: `display_options mod` (Previously `Printer` struct).

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
