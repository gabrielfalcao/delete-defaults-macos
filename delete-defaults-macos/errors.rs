use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Error {
    IOError(String),
    JsonError(String),
    PlistError(String),
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}: {}",
            self.variant(),
            match self {
                Self::IOError(s) => format!("{}", s),
                Self::JsonError(s) => format!("{}", s),
                Self::PlistError(s) => format!("{}", s),
            }
        )
    }
}

impl Error {
    pub fn variant(&self) -> String {
        match self {
            Error::IOError(_) => "IOError",
            Error::JsonError(_) => "JsonError",
            Error::PlistError(_) => "PlistError",
        }
        .to_string()
    }
}

impl std::error::Error for Error {}
impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IOError(format!("{}", e))
    }
}
impl From<iocore::Error> for Error {
    fn from(e: iocore::Error) -> Self {
        Error::IOError(format!("{}", e))
    }
}
impl From<plist::Error> for Error {
    fn from(e: plist::Error) -> Self {
        Error::PlistError(format!("{}", e))
    }
}
impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::JsonError(format!("{}", e))
    }
}
pub type Result<T> = std::result::Result<T, Error>;
