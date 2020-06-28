use rusqlite::{Connection, Transaction};

use super::SubCommand;
use crate::pswdmng::error::Error;
use crate::pswdmng::sql::table::User;

pub(crate) struct UpdateUser {
    user_name_old: String,
    user_name_new: String,
    raw_password_old: String,
    raw_password_new: String,
}

impl UpdateUser {
    pub(crate) fn new(
        user_name_old: String,
        user_name_new: String,
        raw_password_old: String,
        raw_password_new: String,
    ) -> Self {
        UpdateUser {
            user_name_old: user_name_old,
            user_name_new: user_name_new,
            raw_password_old: raw_password_old,
            raw_password_new: raw_password_new,
        }
    }
}

impl SubCommand for UpdateUser {
    fn run(self: Self, conn: &mut Connection) -> Result<(), Error> {
        self.run_with_transaction(conn)
    }

    fn run_transaction_inner(self: &Self, tx: &Transaction) -> Result<(), Error> {
        if !User::exists_by_name(&tx, &self.user_name_old)? {
            return Err(Error::NotValidUser(
                self.user_name_old.clone(),
                self.raw_password_old.clone(),
            ));
        }

        let user_old = User::select_by_name(&tx, &self.user_name_old)?;
        User::validate_password(&user_old, &self.raw_password_old)?;

        if self.user_name_old != self.user_name_new
            && User::exists_by_name(&tx, &self.user_name_new)?
        {
            return Err(Error::AlreadyExistsUser(self.user_name_new.clone()));
        }

        let user_new =
            User::from_raw_password(self.user_name_new.clone(), self.raw_password_new.clone());

        user_new.update(&tx, &self.user_name_old)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::pswdmng::sub_command::{AddUser, Initializer};

    #[test]
    fn test_new() {
        let update_user = UpdateUser::new(
            String::from("old_name"),
            String::from("new_name"),
            String::from("old_pass"),
            String::from("new_pass"),
        );

        assert_eq!(update_user.user_name_old, String::from("old_name"));
        assert_eq!(update_user.user_name_new, String::from("new_name"));
        assert_eq!(update_user.raw_password_old, String::from("old_pass"));
        assert_eq!(update_user.raw_password_new, String::from("new_pass"));
    }

    #[test]
    fn test_run_same_name() {
        let mut conn = Connection::open_in_memory().unwrap();
        let initializer = Initializer::new();
        initializer.run(&mut conn).unwrap();

        let add_user = AddUser::new(String::from("name"), String::from("pass"));
        add_user.run(&mut conn).unwrap();
        let created = User::select_by_name(&conn, &String::from("name"));

        let update_user = UpdateUser::new(
            String::from("name"),
            String::from("name"),
            String::from("pass"),
            String::from("new_pass"),
        );
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

        let add_user = AddUser::new(String::from("name"), String::from("pass"));
        add_user.run(&mut conn).unwrap();
        let created = User::select_by_name(&conn, &String::from("name"));

        let update_user = UpdateUser::new(
            String::from("name"),
            String::from("new_name"),
            String::from("pass"),
            String::from("new_pass"),
        );
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

        let add_user = AddUser::new(String::from("name"), String::from("pass"));
        add_user.run(&mut conn).unwrap();
        let add_user = AddUser::new(String::from("new_name"), String::from("pass"));
        add_user.run(&mut conn).unwrap();
        let created = User::select_by_name(&conn, &String::from("name"));

        let update_user = UpdateUser::new(
            String::from("_name"),
            String::from("new_name"),
            String::from("pass"),
            String::from("new_pass"),
        );
        assert_eq!(
            update_user.run(&mut conn),
            Err(Error::NotValidUser(
                String::from("_name"),
                String::from("pass"),
            ))
        );
        let rollbacked = User::select_by_name(&conn, &String::from("name"));
        assert_eq!(rollbacked, created);

        let update_user = UpdateUser::new(
            String::from("name"),
            String::from("new_name"),
            String::from("_pass"),
            String::from("new_pass"),
        );
        assert_eq!(
            update_user.run(&mut conn),
            Err(Error::NotValidUser(
                String::from("name"),
                String::from("_pass"),
            ))
        );
        let rollbacked = User::select_by_name(&conn, &String::from("name"));
        assert_eq!(rollbacked, created);

        let update_user = UpdateUser::new(
            String::from("name"),
            String::from("new_name"),
            String::from("pass"),
            String::from("new_pass"),
        );
        assert_eq!(
            update_user.run(&mut conn),
            Err(Error::AlreadyExistsUser(String::from("new_name"),))
        );
        let rollbacked = User::select_by_name(&conn, &String::from("name"));
        assert_eq!(rollbacked, created);

        conn.close().unwrap();
    }
}
