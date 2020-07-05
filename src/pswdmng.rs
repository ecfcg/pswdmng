use rand::seq::SliceRandom;
use rand::thread_rng;

pub mod cli;
mod encryption;
mod error;
pub(crate) mod hashcode;
pub(crate) mod sql;
pub(crate) mod sub_command;

pub use error::Error;

const ASCII_STR: &'static str = r#"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%~+=_-`\|^&*()[]{}:'"<>?,./; "#;

pub(crate) fn create_ascii_str(len: usize) -> String {
    let mut rng = thread_rng();
    String::from_utf8(
        ASCII_STR
            .as_bytes()
            .choose_multiple(&mut rng, len)
            .cloned()
            .collect(),
    )
    .unwrap()
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ArgAccount {
    pub(crate) name: String,
    pub(crate) raw_password: String,
    pub(crate) account_id: String,
    pub(crate) email: String,
    pub(crate) url: String,
    pub(crate) comment: String,
}

impl ArgAccount {
    pub(crate) fn new(
        name: String,
        raw_password: String,
        account_id: String,
        email: String,
        url: String,
        comment: String,
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_ascii_str() {
        let len = 16;
        let result = create_ascii_str(len);
        assert_eq!(result.len(), len);
        assert_ne!(result, create_ascii_str(len));
        assert_ne!(result, create_ascii_str(len));
    }
}