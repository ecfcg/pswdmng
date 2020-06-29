use rusqlite::Connection;

use super::SubCommand;
use crate::pswdmng::Error;
use crate::pswdmng::sql::table::User;
use crate::pswdmng::ArgUser;

pub(crate) struct AddUser {
    new_user: ArgUser,
}

impl AddUser {
    pub(crate) fn new(new_user: ArgUser) -> Self {
        AddUser {
            new_user: new_user
        }
    }
}

impl SubCommand for AddUser {
    fn run(self: Self, conn: &Connection) -> Result<(), Error> {
        if User::exists_by_name(&conn, &self.new_user.name)? {
            return Err(Error::AlreadyExistsUser(self.new_user.name));
        }

        let new_user =
            User::from_raw_password(self.new_user.name, self.new_user.raw_password);
        new_user.insert(&conn)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::pswdmng::sub_command::Initializer;

    #[test]
    fn test_new() {
        let user = ArgUser::new(String::from("name"), String::from("password"));
        let result = AddUser::new(user);
        assert_eq!(result.new_user.name, "name");
        assert_eq!(result.new_user.raw_password, "password");
    }

    #[test]
    fn test_run() {
        let mut conn = Connection::open_in_memory().unwrap();

        let initializer = Initializer::new();
        initializer.run(&mut conn).unwrap();

        let user = ArgUser::new(String::from("name"), String::from("pass"));
        let add_user = AddUser::new(user);
        assert_eq!(add_user.run(&mut conn), Ok(()));

        let user = ArgUser::new(String::from("name"), String::from("pass"));
        let add_user = AddUser::new(user);
        assert_eq!(
            add_user.run(&mut conn),
            Err(Error::AlreadyExistsUser(String::from("name")))
        );

        conn.close().unwrap();
    }
}
