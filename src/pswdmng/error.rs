use std::fmt::Display;

use rusqlite::Error as SqlError;
use aes_gcm::Error as AesError;

#[derive(Debug, PartialEq)]
pub enum Error {
    SQLITE(SqlError),
    EncryptError(AesError),
    DecryptError(AesError),
    AlreadyInitialized,
    AlreadyExistsUser(String),
    NotValidUser(String, String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Error::SQLITE(e) => e.fmt(f),
            Error::EncryptError(e) | Error::DecryptError(e) => e.fmt(f),
            Error::AlreadyInitialized => String::from("Already initialized.").fmt(f),
            Error::AlreadyExistsUser (e) => format!("The user is already exists.:{}", e).fmt(f),
            Error::NotValidUser(_, _) => format!("Not a valid user name or password.").fmt(f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::SQLITE(e) => Some(e),
            Error::EncryptError(_) | Error::DecryptError(_) => None,
            Error::AlreadyInitialized
            | Error::AlreadyExistsUser(_)
            | Error::NotValidUser(_, _) => None,
        }
    }
}
