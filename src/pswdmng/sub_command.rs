pub(crate) mod add_user;
pub(crate) mod update_user;
pub(crate) mod initialize;

use crate::pswdmng::error::Error;
use rusqlite::{Connection, Transaction};

pub(crate) use add_user::AddUser;
pub(crate) use initialize::Initializer;

pub(crate) trait SubCommand {
   fn run(self: &Self, conn: &mut Connection) -> Result<(), Error>;

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
