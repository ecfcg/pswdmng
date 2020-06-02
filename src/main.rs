use pswdmng::cli;
use std::process::exit;

fn main() {
    exit(match cli::execute() {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("{}", e);
            1
        }
    });
}
