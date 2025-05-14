# Release note

## 0.14.0('25-May-14)

### changes

* rename `to_show_commit()` to `set_commit_as_visible()`
* rename `to_hide_commit()` to `set_commit_as_hidden()`
* rename `search_commit()` to `search_commits()`
* rename `not_found()` to `is_not_found()`

## 0.13.0('25-May-13)

### added

* added `Option<current_user>` to config
* influenced by the change `Config`'s publication range, added `getter()` to `Config`(e.g. `self.current_user()`)

### changes

* all config fields change `pub` to `private`

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

### added

* added open_without_creating() to Project
* added some documentation comments

### changes

* change trait `DbProject` functions
* `Project::open()` now if file is empty, create new Project
