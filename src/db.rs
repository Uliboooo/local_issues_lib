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

    let de: Project = serde_json::from_str(&content).map_err(Error::SerdeError)?;
    Ok(de)
}

fn load_file(path: &PathBuf) -> Result<File, Error> {
    if !path.exists() {
        if let Some(p) = path.parent() {
            fs::create_dir_all(p).map_err(Error::IoError)?;
        }
    }
    fs::OpenOptions::new()
        .read(true)
        .truncate(false)
        .write(true)
        .create(true)
        .open(path)
        .map_err(Error::IoError)
}
