pub(crate) mod initialize;

use crate::pswdmng::error::Error;
use rusqlite::Connection;

pub(crate) use initialize::Initializer;

pub(crate) trait SubCommand {
   fn run(self: &Self, conn: &Connection) -> Result<(), Error>;
}
