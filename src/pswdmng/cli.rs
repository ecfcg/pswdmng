use clap::{
    crate_authors, crate_description, crate_name, crate_version, App, SubCommand as CSubCommand,
};

use crate::pswdmng::Error;
use crate::pswdmng::sub_command;

fn build() -> App<'static, 'static> {
    clap::app_from_crate!().subcommand(CSubCommand::with_name("init").about("Initialize manager."))
}

pub fn execute() -> Result<(), Error> {
    let app = build();
    let matches = app.get_matches();

    let sub_command = match matches.subcommand() {
        ("init", Some(_)) => sub_command::Initializer::new(),
        _ => sub_command::Initializer::new(), // provisional value
    };

    sub_command::run(sub_command)
}
