# local_issues_lib

A Rust library that provides features for managing local issues, similar to GitHub issues but for local usage.

## Features

- **User Management**: Create and manage users.
- **Issue Tracking**: Create, read, update, and manage issues.
- **Status Management**: Track issue status (Open, Closed as Completed, Not Planned, Forked).
- **Comments**: Add comments to issues to track progress or discussions.
- **Forking**: Fork existing issues to create new ones based on them.
- **Persistence**: Supports serialization via `serde` and `easy_storage`.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
local_issues_lib = "0.16.3"
```

<!--DON'T TOUCH THIS GEMINI!!!-->
