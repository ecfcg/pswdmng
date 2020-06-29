use rusqlite::Connection;

use crate::pswdmng::Error;

mod add_account;
mod add_user;
mod initialize;
mod update_user;

pub(crate) use add_user::AddUser;
pub(crate) use initialize::Initializer;
pub(crate) use update_user::UpdateUser;

pub(crate) fn run<S: SubCommand>(sub_command: S) -> Result<(), Error> {
   let mut conn = match Connection::open("test.db") {
      Ok(c) => c,
      Err(e) => return Err(Error::SQLITE(e)),
   };

   if sub_command.with_transaction() {
      run_with_transaction(sub_command, &mut conn)
   } else {
      run_without_transaction(sub_command, &mut conn)
   }
}

fn run_without_transaction<S: SubCommand>(
   sub_command: S,
   conn: &mut Connection,
) -> Result<(), Error> {
   sub_command.run(&conn)
}

fn run_with_transaction<S: SubCommand>(sub_command: S, conn: &mut Connection) -> Result<(), Error> {
   let tx = match conn.transaction() {
      Ok(t) => t,
      Err(e) => return Err(Error::SQLITE(e)),
   };

   match sub_command.run(&tx) {
      Ok(_) => (),
      Err(e) => {
         // 失敗した場合はロールバック
         match tx.rollback() {
            Ok(_) => (),
            // ロールバックにも失敗した場合
            Err(e2) => return Err(Error::SQLITE(e2)),
         }
         return Err(e);
      }
   };

   match tx.commit() {
      Ok(_) => Ok(()),
      Err(e) => Err(Error::SQLITE(e)),
   }
}

pub(crate) trait SubCommand {
   fn run(self: Self, conn: &Connection) -> Result<(), Error>;
   fn with_transaction(self: &Self) -> bool {
      true
   }
}
