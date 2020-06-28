use crate::pswdmng::error::Error;
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
    fn run(self: &Self, conn: &mut Connection) -> Result<(), Error> {
        if exists_tables(conn)? {
            return Err(Error::LOGIC(String::from("Already initialized.")));
        }
        User::create_table(conn)?;
        Account::create_table(conn)?;
        Ok(())
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
        assert_eq!(
            initializer.run(&mut conn),
            Err(Error::LOGIC(String::from("Already initialized.")))
        );
        conn.close().unwrap();
    }
}
