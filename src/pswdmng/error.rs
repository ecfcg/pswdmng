use std::fmt::Display;

use rusqlite::Error as SqlError;

#[derive(Debug, PartialEq)]
pub enum Error {
    SQLITE(SqlError),
    LOGIC(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Error::SQLITE(e) => e.fmt(f),
            Error::LOGIC(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::SQLITE(e) => Some(e),
            Error::LOGIC(_) => None,
        }
    }
}
