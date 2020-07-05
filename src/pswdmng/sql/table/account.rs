use super::Ddl;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Account {
    user_name: String,
    account_name: String,
    password: Vec::<u8>,
    nonce: Vec::<u8>,
    account_id: String,
    email: String,
    url: String,
    comment: String,
}

impl Ddl for Account {
    fn create_ddl() -> &'static str {
        r###"
        CREATE TABLE ACCOUNT(
        USER_NAME TEXT,
        ACCOUNT_NAME TEXT,
        ACCOUNT_PASSWORD BLOB NOT NULL,
        NONCE BLOB NOT NULL,
        ACCOUNT_ID TEXT NOT NULL,
        EMAIL TEXT NOT NULL,
        URL TEXT NOT NULL,
        COMMENT TEXT NOT NULL,
        PRIMARY KEY(USER_NAME, ACCOUNT_NAME)
        )"###
    }

    fn drop_ddl() -> &'static str {
        "DROP TABLE ACCOUNT"
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use rusqlite::Connection;

    ///
    /// Test of trait Ddl for Account.
    ///
    #[test]
    fn test_create_ddl() {
        let conn = Connection::open_in_memory().unwrap();
        assert_eq!(Account::create_table(&conn), Ok(0 as usize));
        assert_eq!(Account::drop_table(&conn), Ok(0));
        conn.close().unwrap();
    }
}
