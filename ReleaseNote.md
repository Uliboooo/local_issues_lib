# Release note

## 0.16.6('25-May-22)

### add

* add `enum ConfigPath`. this enum provide a choice of home directory or a specific path.

### changes

* `config sava(), load(), load_or_create()` now, require a save path as arg.

### remove

* `get_config_path()`

## 0.16.5('25-May-22)

### changes

* `config save()` now, return Result

## 0.16.4('25-May-22)

### add

* add fn `storage_path(path: P)`: return added storage folder's path path

## 0.16.3('25-May-22)

### changes

* be public `storage mod`

## 0.16.1('25-May-21)

### add `Clone` to `DisplayOptions`

## 0.15.0('25-May-17)

### add

* add method `change_current_user` to `Config`
* add method `update()` to `Project`
* add felid `users` to Project a issue
    * add functions `add_user()`, `change_author_of_issue()`, `assign_new_user_to_issue()` to Project

### changes

* rename `get_mut_users` to `get_user_list`

### fix

* don't update `Project::update_date` problem

## 0.14.3('25-May-17)

### changes

* rename commit to comment in this Lib

## 0.14.2('25-May-17)

### enhance

* enhance examples
* enhance DisplayOptions(old: Printer)

### changes

* rename `to_show_commit()` to `set_commit_as_visible()`
* rename `to_hide_commit()` to `set_commit_as_hidden()`
* rename `search_commit()` to `search_comments()`
* rename `commit()` to `add_commit()`
* rename `not_found()` to `is_not_found()`

### remove

* remove felid `author: User` from `Project`

**i may not have written some of the changes..**

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
