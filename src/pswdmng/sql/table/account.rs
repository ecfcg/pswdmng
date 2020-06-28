use super::Ddl;

#[derive(Clone, Debug)]
pub(crate) struct Account {
    user_name: String,
    account_name: String,
    account_id: String,
    password: String,
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
        ACCOUNT_ID TEXT,
        ACCOUNT_PASSWORD TEXT NOT NULL,
        EMAIL TEXT,
        URL TEXT,
        COMMENT TEXT,
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
