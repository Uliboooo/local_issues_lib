use std::{
    fs::OpenOptions,
    io::{self, Read, Write},
    path::Path,
};

use serde::{Serialize, de::DeserializeOwned};
pub enum Error {
    IoError(io::Error),
    SerdeError(serde_json::Error),
}

/// Arg's `create` use as `OpenOptions.create()`
pub fn load<T: DeserializeOwned, P: AsRef<Path>>(path: P, create: bool) -> Result<T, Error> {
    let mut f = OpenOptions::new()
        .read(true)
        .create(create)
        .truncate(false)
        .write(true)
        .open(path)
        .map_err(Error::IoError)?;
    let mut con = String::new();
    f.read_to_string(&mut con).map_err(Error::IoError)?;

    serde_json::from_str(&con).map_err(Error::SerdeError)
}

pub fn save<T: Serialize, P: AsRef<Path>>(src: T, path: P) -> Result<(), Error> {
    let serialized_json = serde_json::to_string(&src).map_err(Error::SerdeError)?;
    let mut f = OpenOptions::new()
        .read(false)
        .write(true)
        .truncate(true)
        .open(path)
        .map_err(Error::IoError)?;

    f.write_all(serialized_json.as_bytes())
        .map_err(Error::IoError)
}
