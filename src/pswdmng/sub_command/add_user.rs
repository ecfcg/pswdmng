use rusqlite::{Connection, Error as SqlError, Transaction};

use super::SubCommand;
use crate::pswdmng::error::Error;
use crate::pswdmng::hashcode;
use crate::pswdmng::sql::table::User;

pub(crate) struct AddUser {
    user_name: String,
    raw_user_password: String,
}

impl AddUser {
    pub(crate) fn new(user_name: String, raw_user_password: String) -> Self {
        AddUser {
            user_name: user_name,
            raw_user_password: raw_user_password,
        }
    }
}

impl SubCommand for AddUser {
    fn run(self: &Self, conn: &mut Connection) -> Result<(), Error> {
        self.run_with_transaction(conn)
    }

    fn run_transaction_inner(self: &Self, tx: &Transaction) -> Result<(), Error> {
        if User::exists_by_name(&tx, &self.user_name)? {
            return Err(Error::LOGIC(format!(
                "The user is already exists. :{}",
                &self.user_name
            )));
        }

        let salt = hashcode::create_ascii_str(16);
        let hashed = hashcode::sha3_512_hashcode(&self.raw_user_password, &salt);
        let new_user = User::new(&self.user_name, &hashed, &salt);
        new_user.insert(&tx)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let result = AddUser::new(String::from("name"), String::from("password"));
        assert_eq!(result.user_name, "name");
        assert_eq!(result.raw_user_password, "password");
    }
}
