use std::{
    fs::{self, File},
    io::{Read, Write},
    path::{Path, PathBuf},
};

use crate::{Error, Project};

/// db_pathはbodyから勝手に読み取る
pub fn save(body: Project) -> Result<(), Error> {
    let se = serde_json::to_string(&body).map_err(Error::SerdeError)?;
    let mut f = load_file(&body.db_path)?;
    f.write_all(se.as_bytes()).map_err(Error::IoError)?;
    Ok(())
}

/// if path is'n exist, create a db file at path.
pub fn load_db<P: AsRef<Path>>(path: &P) -> Result<Project, Error> {
    let mut f = load_file(&path.as_ref().to_path_buf())?;
    let mut content = String::new();
    f.read_to_string(&mut content).map_err(Error::IoError)?;

    let se: Project = serde_json::from_str(&content).map_err(Error::SerdeError)?;
    Ok(se)
}

fn load_file(path: &PathBuf) -> Result<File, Error> {
    fs::OpenOptions::new()
        .read(true)
        .truncate(false)
        .write(true)
        .create(true)
        .open(path)
        .map_err(Error::IoError)
}
