use super::Ddl;

#[derive(Clone, Debug)]
pub(crate) struct User {
    id: String,
    name: String,
    password: String,
}

impl Ddl for User {
    fn create_ddl() -> &'static str {
        r###"
        CREATE TABLE USER(
            ID TEXT PRIMARY KEY,
            NAME TEXT UNIQUE NOT NULL,
            PASSWORD TEXT NOT NULL
        )
        "###
    }

    fn drop_ddl() -> &'static str {
        "DROP TABLE USER"
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rusqlite::Connection;

    ///
    /// Test of trait Ddl for User.
    ///
    #[test]
    fn test_create_ddl() {
        let conn = Connection::open_in_memory().unwrap();
        assert_eq!(User::create_table(&conn), Ok(0));
        assert_eq!(User::drop_table(&conn), Ok(0));
        conn.close().unwrap();
    }
}
