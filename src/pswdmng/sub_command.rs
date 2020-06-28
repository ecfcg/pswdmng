use rusqlite::{Connection, Transaction};

use crate::pswdmng::error::Error;
use crate::pswdmng::hashcode;
use crate::pswdmng::sql::table::User;

mod add_user;
mod initialize;
mod update_user;

pub(crate) use add_user::AddUser;
pub(crate) use initialize::Initializer;
pub(crate) use update_user::UpdateUser;

pub(crate) trait SubCommand {
   fn run(self: Self, conn: &mut Connection) -> Result<(), Error>;

   fn run_transaction_inner(self: &Self, _tx: &Transaction) -> Result<(), Error> {
      Ok(())
   }

   fn run_with_transaction(self: &Self, conn: &mut Connection) -> Result<(), Error> {
      let tx = match conn.transaction() {
         Ok(t) => t,
         Err(e) => return Err(Error::SQLITE(e)),
      };

      match self.run_transaction_inner(&tx) {
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
}
