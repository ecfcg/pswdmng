pub mod cli;
mod error;
pub(crate) mod hashcode;
pub(crate) mod sql;
pub(crate) mod sub_command;

pub use error::Error;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ArgAccount {
    pub(crate) name: String,
    pub(crate) raw_password: Option<String>,
    pub(crate) account_id: Option<String>,
    pub(crate) email: Option<String>,
    pub(crate) url: Option<String>,
    pub(crate) comment: Option<String>,
}

impl ArgAccount {
    pub(crate) fn new(
        name: String,
        raw_password: Option<String>,
        account_id: Option<String>,
        email: Option<String>,
        url: Option<String>,
        comment: Option<String>,
    ) -> Self {
        ArgAccount {
            name: name,
            raw_password: raw_password,
            account_id: account_id,
            email: email,
            url: url,
            comment: comment,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ArgUser {
    pub(crate) name: String,
    pub(crate) raw_password: String,
}

impl ArgUser {
    pub(crate) fn new(name: String, raw_password: String) -> Self {
        ArgUser {
            name: name,
            raw_password: raw_password,
        }
    }
}
