use std::{
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use crate::{Error, Project};

pub fn save(body: Project) -> Result<(), Error> {
    let se = toml::to_string(&body).map_err(|_| Error::SomeError)?;
    let mut f = fs::OpenOptions::new()
        .read(true)
        .truncate(false)
        .write(true)
        .create(true)
        .open(body.db_path)
        .map_err(Error::IoError)?;
    f.write_all(se.as_bytes()).map_err(Error::IoError)?;
    Ok(())
}

/// if path is'n exist, create a db file at path.
pub fn load_db<P: AsRef<Path>>(path: &P) -> Result<Project, Error> {
    let mut f = fs::OpenOptions::new()
        .read(true)
        .truncate(false)
        .write(true)
        .create(true)
        .open(path.as_ref())
        .map_err(Error::IoError)?;
    let mut content = String::new();
    f.read_to_string(&mut content).map_err(Error::IoError)?;
    let se: Project = toml::from_str(&content).map_err(|_| Error::SomeError)?;
    Ok(se)
}

fn load_file(path: &PathBuf) -> Result<String, Error> {
    let mut f = fs::OpenOptions::new()
        .read(true)
        .truncate(false)
        .write(true)
        .create(true)
        .open(path)
        .map_err(Error::IoError)?;
    let mut content = String::new();
    f.read_to_string(&mut content).map_err(Error::IoError)?;
    Ok(content)
}
