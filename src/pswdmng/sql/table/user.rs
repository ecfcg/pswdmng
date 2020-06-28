use rusqlite::Connection;

use super::Ddl;
use crate::pswdmng::error::Error;
use crate::pswdmng::hashcode;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct User {
    user_name: String,
    user_password: String,
    salt: String,
}

impl User {
    pub(crate) fn new(user_name: String, user_password: String, salt: String) -> Self {
        User {
            user_name: user_name,
            user_password: user_password,
            salt: salt,
        }
    }

    pub(crate) fn from_raw_password(
        user_name: String,
        raw_user_password: String,
    ) -> Self {
        let salt = hashcode::create_ascii_str(16);
        let hashed = hashcode::sha3_512_hashcode(&raw_user_password, &salt);
        Self::new(user_name, hashed, salt)
    }

    pub(crate) fn select_by_name(conn: &Connection, user_name: &String) -> Result<Self, Error> {
        match conn.query_row_named(
            "SELECT USER_NAME, USER_PASSWORD, SALT FROM USER WHERE USER_NAME = :user_name",
            &[(":user_name", &user_name)],
            |row| Ok(Self::new(row.get(0)?, row.get(1)?, row.get(2)?)),
        ) {
            Ok(u) => Ok(u),
            Err(e) => Err(Error::SQLITE(e)),
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

    pub(crate) fn insert(self: Self, conn: &Connection) -> Result<usize, Error> {
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

    pub(crate) fn update(
        self: Self,
        conn: &Connection,
        user_name_old: &String,
    ) -> Result<usize, Error> {
        match conn.execute_named(
            "UPDATE USER SET (USER_NAME, USER_PASSWORD, SALT) = (:user_name, :password, :salt) WHERE USER_NAME = :user_name_old",
            &[
                (":user_name", &self.user_name),
                (":password", &self.user_password),
                (":salt", &self.salt),
                (":user_name_old", &user_name_old)
            ],
        ) {
            Ok(i) => Ok(i),
            Err(e) => Err(Error::SQLITE(e)),
        }
    }

    pub(crate) fn validate_password(user: &User, raw_password: &String) -> Result<(), Error> {
        let hashed = hashcode::sha3_512_hashcode(&raw_password, &user.salt);
        if user.user_password != hashed {
            return Err(Error::NotValidUser(
                user.user_name.clone(),
                raw_password.clone(),
            ));
        }
        Ok(())
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

    fn test_def_user() -> User {
        User::new(
            String::from("name"),
            String::from("password"),
            String::from("salt"),
        )
    }

    #[test]
    fn test_new() {
        let user = test_def_user();

        assert_eq!(user.user_name, "name");
        assert_eq!(user.user_password, "password");
        assert_eq!(user.salt, "salt");
    }

    #[test]
    fn test_select_by_name() {
        let user = test_def_user();

        let conn = Connection::open_in_memory().unwrap();
        User::create_table(&conn).unwrap();

        user.insert(&conn).unwrap();

        let user = test_def_user();
        assert_eq!(User::select_by_name(&conn, &user.user_name), Ok(user));

        User::drop_table(&conn).unwrap();
        conn.close().unwrap();
    }

    #[test]
    fn test_exists_by_name() {
        let user = test_def_user();

        let conn = Connection::open_in_memory().unwrap();
        User::create_table(&conn).unwrap();

        user.insert(&conn).unwrap();

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
        let user = test_def_user();

        let conn = Connection::open_in_memory().unwrap();
        User::create_table(&conn).unwrap();

        let count: isize = conn
            .query_row("SELECT COUNT(*) FROM USER", &EMPTY_PARAM, |row| row.get(0))
            .unwrap();
        assert_eq!(count, 0);

        user.insert(&conn).unwrap();

        let user = test_def_user();
        assert_eq!(User::select_by_name(&conn, &user.user_name), Ok(user));

        User::drop_table(&conn).unwrap();
        conn.close().unwrap();
    }

    #[test]
    fn test_update() {
        let user = test_def_user();

        let conn = Connection::open_in_memory().unwrap();
        User::create_table(&conn).unwrap();

        user.insert(&conn).unwrap();
        let user = test_def_user();
        assert_eq!(User::select_by_name(&conn, &user.user_name), Ok(user));

        let user_updated = User::new(
            String::from("name_u"),
            String::from("password_u"),
            String::from("salt_u"),
        );

        user_updated
            .update(&conn, &String::from(test_def_user().user_name))
            .unwrap();

        let user_updated = User::new(
            String::from("name_u"),
            String::from("password_u"),
            String::from("salt_u"),
        );
        assert_eq!(
            User::select_by_name(&conn, &user_updated.user_name),
            Ok(user_updated)
        );

        User::drop_table(&conn).unwrap();
        conn.close().unwrap();
    }

    #[test]
    fn test_validate_password() {
        let user = User::from_raw_password(
            String::from("name"),
            String::from("pass"),
        );
        assert_eq!(
            User::validate_password(&user, &String::from("pass")),
            Ok(())
        );

        assert_eq!(
            User::validate_password(&user, &String::from("password")),
            Err(Error::NotValidUser(
                user.user_name.clone(),
                String::from("password")
            )),
        );
    }

    #[test]
    fn test_ddl() {
        let conn = Connection::open_in_memory().unwrap();
        assert_eq!(User::create_table(&conn), Ok(0));
        assert_eq!(User::drop_table(&conn), Ok(0));
        conn.close().unwrap();
    }
}
