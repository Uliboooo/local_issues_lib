# Release note

## 0.12.0 ('25-May-11)

### fix

* fix problem: `Project::open()` and `Project::open_or_create()` features ware reversed

### Big changes

* mod name change `db.rs` to `storage.rs`

### changes

* `Users.aad_user()` now return `()`.
* `open_without_creating()` to `open_or_create()`
* change trait name `DbProject` to `ProjectManager`

## 0.11.1 ('25-May-11)

### add

* added open_without_creating() to Project
* added some documentation comments

### changes

* change trait `DbProject` functions
* `Project::open()` now if file is empty, create new Project
