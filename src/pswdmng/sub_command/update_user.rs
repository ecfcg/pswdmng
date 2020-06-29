use rusqlite::Connection;

use super::SubCommand;
use crate::pswdmng::Error;
use crate::pswdmng::sql::table::User;
use crate::pswdmng::ArgUser;

pub(crate) struct UpdateUser {
    old_user: ArgUser,
    new_user: ArgUser,
}

impl UpdateUser {
    pub(crate) fn new(old_user: ArgUser, new_user: ArgUser) -> Self {
        UpdateUser {
            old_user: old_user,
            new_user: new_user,
        }
    }
}

impl SubCommand for UpdateUser {
    fn run(self: Self, conn: &Connection) -> Result<(), Error> {
        if !User::exists_by_name(&conn, &self.old_user.name)? {
            return Err(Error::NotValidUser(
                self.old_user.name,
                self.old_user.raw_password,
            ));
        }

        let user_old = User::select_by_name(&conn, &self.old_user.name)?;
        user_old.validate_password(&self.old_user.raw_password)?;

        if self.old_user.name != self.new_user.name
            && User::exists_by_name(&conn, &self.new_user.name)?
        {
            return Err(Error::AlreadyExistsUser(self.new_user.name));
        }

        let user_new = User::from_raw_password(self.new_user.name, self.new_user.raw_password);

        user_new.update(&conn, &self.old_user.name)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::pswdmng::sub_command::{AddUser, Initializer};

    #[test]
    fn test_new() {
        let old_user = ArgUser::new(String::from("old_name"), String::from("old_pass"));
        let new_user = ArgUser::new(String::from("new_name"), String::from("new_pass"));
        let update_user = UpdateUser::new(old_user, new_user);

        assert_eq!(update_user.old_user.name, String::from("old_name"));
        assert_eq!(update_user.old_user.raw_password, String::from("old_pass"));
        assert_eq!(update_user.new_user.name, String::from("new_name"));
        assert_eq!(update_user.new_user.raw_password, String::from("new_pass"));
    }

    #[test]
    fn test_run_same_name() {
        let mut conn = Connection::open_in_memory().unwrap();
        let initializer = Initializer::new();
        initializer.run(&mut conn).unwrap();

        let user = ArgUser::new(String::from("name"), String::from("pass"));
        let add_user = AddUser::new(user);
        add_user.run(&mut conn).unwrap();
        let created = User::select_by_name(&conn, &String::from("name"));

        let old_user = ArgUser::new(String::from("name"), String::from("pass"));
        let new_user = ArgUser::new(String::from("name"), String::from("new_pass"));
        let update_user = UpdateUser::new(old_user, new_user);
        assert_eq!(update_user.run(&mut conn), Ok(()));

        let updated = User::select_by_name(&conn, &String::from("name"));
        assert_ne!(updated, created);

        conn.close().unwrap();
    }

    #[test]
    fn test_run_diff_name() {
        let mut conn = Connection::open_in_memory().unwrap();
        let initializer = Initializer::new();
        initializer.run(&mut conn).unwrap();

        let user = ArgUser::new(String::from("name"), String::from("pass"));
        let add_user = AddUser::new(user);
        add_user.run(&mut conn).unwrap();
        let created = User::select_by_name(&conn, &String::from("name"));

        let old_user = ArgUser::new(String::from("name"), String::from("pass"));
        let new_user = ArgUser::new(String::from("new_name"), String::from("new_pass"));
        let update_user = UpdateUser::new(old_user, new_user);
        assert_eq!(update_user.run(&mut conn), Ok(()));

        let updated = User::select_by_name(&conn, &String::from("new_name"));
        assert_ne!(updated, created);

        conn.close().unwrap();
    }

    #[test]
    fn test_run_err() {
        let mut conn = Connection::open_in_memory().unwrap();
        let initializer = Initializer::new();
        initializer.run(&mut conn).unwrap();

        let user = ArgUser::new(String::from("name"), String::from("pass"));
        let add_user = AddUser::new(user);
        add_user.run(&mut conn).unwrap();

        let user = ArgUser::new(String::from("new_name"), String::from("pass"));
        let add_user = AddUser::new(user);
        add_user.run(&mut conn).unwrap();

        let created = User::select_by_name(&conn, &String::from("name"));

        let old_user = ArgUser::new(String::from("_name"), String::from("pass"));
        let new_user = ArgUser::new(String::from("new_name"), String::from("new_pass"));
        let update_user = UpdateUser::new(old_user, new_user);
        assert_eq!(
            update_user.run(&mut conn),
            Err(Error::NotValidUser(
                String::from("_name"),
                String::from("pass"),
            ))
        );
        let rollbacked = User::select_by_name(&conn, &String::from("name"));
        assert_eq!(rollbacked, created);

        let old_user = ArgUser::new(String::from("name"), String::from("_pass"));
        let new_user = ArgUser::new(String::from("new_name"), String::from("new_pass"));
        let update_user = UpdateUser::new(old_user, new_user);
        assert_eq!(
            update_user.run(&mut conn),
            Err(Error::NotValidUser(
                String::from("name"),
                String::from("_pass"),
            ))
        );
        let rollbacked = User::select_by_name(&conn, &String::from("name"));
        assert_eq!(rollbacked, created);

        let old_user = ArgUser::new(String::from("name"), String::from("pass"));
        let new_user = ArgUser::new(String::from("new_name"), String::from("new_pass"));
        let update_user = UpdateUser::new(old_user, new_user);
        assert_eq!(
            update_user.run(&mut conn),
            Err(Error::AlreadyExistsUser(String::from("new_name"),))
        );
        let rollbacked = User::select_by_name(&conn, &String::from("name"));
        assert_eq!(rollbacked, created);

        conn.close().unwrap();
    }
}
