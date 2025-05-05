# local issues lib

## Local Issues Lib

this lib provides feature that manage issues like a ["github issue"](https://github.com/Uliboooo/local_issues_lib/issues).

## Example



## changes between 0.2.0 -> 0.3.0

- Issueの`body`を`commit_messages: message::CommitMessages,`に

## changes between v0.1.1 ~ v0.2.0

- Issueの`body`を`Vec<String>`に


## RoadMap

## Project Structure

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
