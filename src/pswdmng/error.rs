use std::fmt::Display;

use rusqlite::Error as SqlError;

#[derive(Debug, PartialEq)]
pub enum Error {
    SQLITE(SqlError),
    AlreadyInitialized,
    AlreadyExistsUser(String),
    NotValidUser(String, String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Error::SQLITE(e) => e.fmt(f),
            AlreadyInitialized => String::from("Already initialized.").fmt(f),
            Error::AlreadyExistsUser (e) => format!("The user is already exists.:{}", e).fmt(f),
            Error::NotValidUser(_, _) => format!("Not a valid user name or password.").fmt(f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::SQLITE(e) => Some(e),
            Error::AlreadyInitialized
            | Error::AlreadyExistsUser(_)
            | Error::NotValidUser(_, _) => None,
        }
    }
}
