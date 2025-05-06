use std::{
    fmt::Display,
    fs::{self, OpenOptions},
    io::{self, Read, Write},
    path::Path,
};

use serde::{Serialize, de::DeserializeOwned};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Serde(serde_json::Error),
    /// emty file
    FileIsZero,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(e) => write!(f, "io error: {}", e),
            Error::Serde(e) => write!(f, "serde error: {}", e),
            Error::FileIsZero => write!(f, "file is zero"),
        }
    }
}

impl Error {
    pub fn is_file_is_zero(&self) -> bool {
        matches!(self, Error::FileIsZero)
    }

    pub fn not_found(&self) -> bool {
        match self {
            Error::Io(error) => error.kind() == io::ErrorKind::NotFound,
            _ => false,
        }
    }
}

/// Arg's `create` use as `OpenOptions.create()`
pub fn load<T: DeserializeOwned, P: AsRef<Path>>(path: P, create: bool) -> Result<T, Error> {
    if path.as_ref().parent().unwrap().exists() || create {
        fs::create_dir_all(path.as_ref().parent().unwrap()).map_err(Error::Io)?;
        println!("created.");
    }

    let mut f = OpenOptions::new()
        .read(true)
        .create(create)
        .truncate(false)
        .write(true)
        .open(path)
        .map_err(Error::Io)?;
    let mut con = String::new();
    f.read_to_string(&mut con).map_err(Error::Io)?;

    if con.is_empty() {
        Err(Error::FileIsZero)
    } else {
        serde_json::from_str(&con).map_err(Error::Serde)
    }
}

pub fn save<T: Serialize, P: AsRef<Path>>(src: T, path: P) -> Result<(), Error> {
    let serialized_json = serde_json::to_string(&src).map_err(Error::Serde)?;
    let mut f = OpenOptions::new()
        .read(false)
        .write(true)
        .truncate(true)
        .open(path)
        .map_err(Error::Io)?;

    f.write_all(serialized_json.as_bytes()).map_err(Error::Io)
}
