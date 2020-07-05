use rusqlite::Connection;

use crate::pswdmng::{ArgAccount, ArgUser, Error};
use crate::pswdmng::sql::table::{Account, User};
use super::SubCommand;

struct AddAccount{
    user:ArgUser,
    new_account:ArgAccount, 
}

impl SubCommand for AddAccount {
    fn run(self: Self, conn: &Connection) -> Result<(), Error> {
        let user = User::from_arg_user(&conn, self.user);
        

        Ok(())
    }
}
