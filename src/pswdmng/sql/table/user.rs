use rusqlite::Connection;

use super::Ddl;
use crate::pswdmng::error::Error;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct User {
    user_name: String,
    user_password: String,
    salt: String,
}

impl User {
    pub(crate) fn new(user_name: &String, user_password: &String, salt: &String) -> Self {
        User {
            user_name: user_name.clone(),
            user_password: user_password.clone(),
            salt: salt.clone(),
        }
    }

    pub(crate) fn exists_by_name(conn: &Connection, user_name: &String) -> Result<bool, Error> {
        let count: isize = match conn.query_row_named(
            "SELECT COUNT(*) FROM USER WHERE USER_NAME = :user_name",
            &[(":user_name", &user_name)],
            |row| row.get(0),
        ) {
            Ok(c) => c,
            Err(e) => return Err(Error::SQLITE(e)),
        };
        Ok(count != 0)
    }

    pub(crate) fn insert(self: &Self, conn: &Connection) -> Result<usize, Error> {
        match conn.execute_named(
            "INSERT INTO USER (USER_NAME, USER_PASSWORD, SALT) VALUES (:user_name, :password, :salt)",
            &[
                (":user_name", &self.user_name),
                (":password", &self.user_password),
                (":salt", &self.salt),
            ],
        ) {
            Ok(i) => Ok(i),
            Err(e) => Err(Error::SQLITE(e)),
        }
    }
}

impl Ddl for User {
    fn create_ddl() -> &'static str {
        r###"
        CREATE TABLE USER(
            USER_NAME TEXT PRIMARY KEY,
            USER_PASSWORD TEXT NOT NULL,
            SALT TEXT NOT NULL
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
    use rusqlite::types::ToSql;

    const EMPTY_PARAM: [&dyn ToSql; 0] = [];

    #[test]
    fn test_new() {
        let user = User::new(
            &String::from("name"),
            &String::from("password"),
            &String::from("salt"),
        );

        assert_eq!(user.user_name, "name");
        assert_eq!(user.user_password, "password");
        assert_eq!(user.salt, "salt");
    }

    #[test]
    fn test_exists_by_name() {
        let conn = Connection::open_in_memory().unwrap();
        User::create_table(&conn).unwrap();
        conn.execute(
            "INSERT INTO USER VALUES ('name', 'password', 'salt')",
            &EMPTY_PARAM,
        )
        .unwrap();
        assert_eq!(User::exists_by_name(&conn, &String::from("name")), Ok(true));
        assert_eq!(
            User::exists_by_name(&conn, &String::from("named")),
            Ok(false)
        );
        User::drop_table(&conn).unwrap();
        conn.close().unwrap();
    }

    #[test]
    fn test_insert() {
        let conn = Connection::open_in_memory().unwrap();
        User::create_table(&conn).unwrap();

        let count: isize = conn
            .query_row("SELECT COUNT(*) FROM USER", &EMPTY_PARAM, |row| row.get(0))
            .unwrap();
        assert_eq!(count, 0);

        let user_1 = User {
            user_name: String::from("name"),
            user_password: String::from("pass"),
            salt: String::from("salt"),
        };
        user_1.insert(&conn).unwrap();

        let user_2 = conn
            .query_row("SELECT * FROM USER", &EMPTY_PARAM, |row| {
                Ok(User {
                    user_name: row.get(0)?,
                    user_password: row.get(1)?,
                    salt: row.get(2)?,
                })
            })
            .unwrap();
        assert_eq!(user_1, user_2);

        User::drop_table(&conn).unwrap();
        conn.close().unwrap();
    }

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
