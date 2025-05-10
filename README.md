# local issues lib

## Local Issues Lib

this lib provides feature that manage issues like a ["github issue"](https://github.com/Uliboooo/local_issues_lib/issues).

## RoadMap

### v1 ~

- [ ] basic feats
- [ ] feat: Config
- [ ] Users and author

### v2 ~

- [ ] feat: save to md file. 

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
