# Remaining Tasks & Missing Features

This document outlines the features that are currently missing or need improvement in `local_issues_lib`.

## High Priority

- [x] **Persistence (Serialization/Deserialization)**
  - **Status**: Structs now derive `Serialize` and `Deserialize`. `easy_storage::Storeable` trait is implemented.
  - **Next Step**: Implement a manager or utility to actually save/load these to the filesystem (e.g., using `easy_storage` functionality).

- [ ] **Stable Identifiers (UUID)**
  - **Description**: Currently, issues and users are accessed via their index in a `Vec`. This is unstable if items are reordered or removed.
  - **Task**: Integrate the `uuid` crate (already in dependencies) to assign unique IDs to `Issue` and `User` entities. Replace index-based lookups with ID-based lookups.

- [ ] **Issue Metadata**
  - ~~**Description**: Issues are currently limited to a name/title and comments.~~
  - **Task**: Add the following fields to the `Issue` struct:
    - ~~`description`: String (Markdown support)~~
    - `updated_at`: `DateTime<Local>` (Note: `created_at` could be derived from ID or added explicitly)

## Medium Priority

- [ ] **Labels / Tags System**
  - **Description**: Ability to categorize issues (e.g., "bug", "feature", "wontfix").
  - **Task**: Create a `Label` struct/enum and allow attaching multiple labels to an `Issue`.

- [ ] **Assignees**
  - **Description**: Ability to assign specific users to an issue.
  - **Task**: Add `assignees: Vec<UserId>` to `Issue`.

- [ ] **Improved Error Handling**
  - **Description**: Many methods currently return `Option` or might panic on out-of-bounds access.
  - **Task**: Define a custom `Error` enum and switch return types to `Result<T, Error>`.

- [ ] **Advanced Filtering & Search**
  - **Description**: Implement filters for:
    - Status (Open vs Closed)
    - Assignee
    - Author
    - Labels

## Low Priority

- [ ] **Milestones**
  - **Description**: Grouping issues into milestones/versions.

- [x] **Comments System**
  - **Status**: `Log` has been renamed to `Comment` and is integrated into `Issue`.
  - **Next Step**: Consider if a separate audit log is needed alongside user comments.

- [ ] **Input Validation**
  - **Description**: Validate user inputs (e.g., email format, empty titles).
