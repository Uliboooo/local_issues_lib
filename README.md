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

* [x] basic feats
    * [x] manage project
        * [x] open
        * [x] save
    * [x] edit project info
        * [x] rename
    * [x] search issues and comments
    * [x] manage issues
        * [x] add issue
        * [x] rename issue
        * [x] edit due
        * [x] open issue
        * [x] close issue
        * [x] remove issue
    * [x] manage comments
        * [x] add comment
        * [x] remove comment
        * [x] show comment
        * [x] hide comment
* [x] `Users` and `author` for AI functions(v2~)
* [x] feat: Config
* [ ] Display by Options
* [ ] stabilization

### v2 ~

* [ ] feat: export to md file. 
* [ ] feat: AI(LLM) team member as `User`. -> [details](https://github.com/Uliboooo/local_issues_lib/blob/main/articles/LLM_feature.md)
* [ ] feat: Tags and priority
* [ ] feat: `display_options mod` (Previously `Printer` struct).
* [ ] feat: integrated git ctrl

## Project Structure

- Project
    - project name
    - current_id
    - created_at
    - updated_at
    - project_path
    - storage_path
    - db_path
    - users
        - list: HashMap\<Uuid, User\>
            - User: {name, id: Uuid}
    - issues: HashMap\<u64, Issue\>
        - u64: id
        - Issue
            - name
            - created_at
            - updated_at
            - due_date: Option
            - author: User
            - assigned_member: Users
            - status
                - Open
                - Closed
                    - Resolved
                    - UnResolved
            - messages
                - list: Vec\<Message\>
                    - Message
                        - message: String
                        - show: bool
                        - created_at
                        - author: User 