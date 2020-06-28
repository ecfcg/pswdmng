mod account;
mod user;

pub(crate) use self::account::Account;
pub(crate) use self::user::User;
use crate::pswdmng::error::Error;
use rusqlite::{params, Connection, Error as SqlError};

pub(crate) const TABLE_NAMES: [&'static str; 2] = ["ACCOUNT", "USER"];

pub(crate) trait Ddl {
    fn create_ddl() -> &'static str;
    fn drop_ddl() -> &'static str;

    fn execute_ddl(conn: &Connection, ddl_statement: &'static str) -> Result<usize, Error> {
        match conn.execute(ddl_statement, params![]) {
            Ok(u) => Ok(u),
            Err(e) => Err(Error::SQLITE(e)),
        }
    }

    fn create_table(con: &Connection) -> Result<usize, Error> {
        Self::execute_ddl(con, Self::create_ddl())
    }

    fn drop_table(con: &Connection) -> Result<usize, Error> {
        Self::execute_ddl(con, Self::drop_ddl())
    }
}

pub(crate) fn exists_tables(conn: &Connection) -> Result<bool, Error> {
    let exists_tables = match exist_table_impl(conn) {
        Ok(v) => v,
        Err(e) => return Err(Error::SQLITE(e)),
    };

    Ok(exists_tables.eq(&TABLE_NAMES))
}

fn exist_table_impl(conn: &Connection) -> Result<Vec<String>, SqlError> {
    let mut stmt = conn.prepare(
        r###"
    SELECT
        NAME
    FROM
        SQLITE_MASTER
    WHERE
        TYPE = 'table' 
        AND NAME NOT LIKE 'sqlite_%'
    ORDER BY NAME
    "###,
    )?;
    let mut rows = stmt.query(params![])?;
    let mut exists_tables: Vec<String> = Vec::new();
    loop {
        let row = rows.next()?;
        let table_name = match row {
            Some(s) => s.get(0)?,
            None => break,
        };
        exists_tables.push(table_name);
    }
    Ok(exists_tables)
}

#[cfg(test)]
mod test {
    use super::*;

    /// Test of exists_tables
    #[test]
    fn test_exists_tables() {
        let conn = Connection::open_in_memory().unwrap();
        assert_eq!(exists_tables(&conn), Ok(false));

        for t in &TABLE_NAMES {
            conn.execute(&format!("CREATE TABLE {} (x integer)", &t), params![])
                .unwrap();
            assert_eq!(exists_tables(&conn), Ok(false));
            conn.execute(&format!("DROP TABLE {}", &t), params![])
                .unwrap();
        }

        for t in &TABLE_NAMES {
            conn.execute(&format!("CREATE TABLE {} (x integer)", &t), params![])
                .unwrap();
        }
        assert_eq!(exists_tables(&conn), Ok(true));
        conn.close().unwrap();
    }
}
