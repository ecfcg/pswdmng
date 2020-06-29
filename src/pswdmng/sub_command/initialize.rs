use crate::pswdmng::Error;
use crate::pswdmng::sql::table::{exists_tables, Account, Ddl, User};

use super::SubCommand;

use rusqlite::Connection;

pub(crate) struct Initializer {}

impl Initializer {
    pub(crate) fn new() -> Self {
        Initializer {}
    }
}

impl SubCommand for Initializer {
    fn run(self: Self, conn: &Connection) -> Result<(), Error> {
        if exists_tables(conn)? {
            return Err(Error::AlreadyInitialized);
        }
        User::create_table(conn)?;
        Account::create_table(conn)?;
        Ok(())
    }

    fn with_transaction(self: &Self) -> bool {
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run() {
        let mut conn = Connection::open_in_memory().unwrap();

        let initializer = Initializer::new();
        assert_eq!(initializer.run(&mut conn).unwrap(), ());

        let initializer = Initializer::new();
        assert_eq!(initializer.run(&mut conn), Err(Error::AlreadyInitialized));

        conn.close().unwrap();
    }
}
