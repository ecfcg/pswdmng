use clap::{
    crate_authors, crate_description, crate_name, crate_version, App, SubCommand as CSubCommand,
};

use rusqlite::Connection;

use crate::pswdmng::error::Error;
use crate::pswdmng::sub_command::{Initializer, SubCommand as PSubCommand};

fn build() -> App<'static, 'static> {
    clap::app_from_crate!().subcommand(CSubCommand::with_name("init").about("Initialize manager."))
}

pub fn execute() -> Result<(), Error> {
    let app = build();
    let matches = app.get_matches();

    let runner = match matches.subcommand() {
        ("init", Some(_)) => Initializer::new(),
        _ => Initializer::new(), // provisional value
    };

    todo!(); // Provisional file name
    let mut conn = match Connection::open("test.db") {
        Ok(c) => c,
        Err(e) => return Err(Error::SQLITE(e)),
    };

    runner.run(&mut conn)
}
