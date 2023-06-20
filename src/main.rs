

use log::{trace};
use sqlx_md_gen::{cmd::{generate::generate, sheet::sheet, new::new}};
fn main() {
    env_logger::init();

    let mut args = std::env::args();
    trace!("Args given : {args:?} ");

    // Command is the 2nd item on the Iterator
    match args.nth(1) {
        Some(s) => match &s as &str {
            "generate" => generate(args.nth(0)),
            "sheet" => sheet(args.nth(0)),
            "new" => new(args.nth(0)),
            _ => println!("Invalid Command.")
        },
        None => println!("No command provided.")
    }

}

