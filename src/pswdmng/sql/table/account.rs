use super::Ddl;

#[derive(Clone, Debug)]
pub(crate) struct Account {
    id: String,
    name: String,
    password: String,
    email: String,
    url: String,
}

impl Ddl for Account {
    fn create_ddl() -> &'static str {
        r###"
        CREATE TABLE ACCOUNT(
        ID TEXT,
        NAME TEXT,
        PASSWORD TEXT NOT NULL,
        EMAIL TEXT,
        URL TEXT,
        PRIMARY KEY(ID, NAME),
        FOREIGN KEY(ID) REFERENCES USER(ID)
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
