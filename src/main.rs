#[macro_use]
extern crate clap;
extern crate dirs;
#[macro_use]
extern crate prettytable;

mod cli;

fn main() {
    match cli::run() {
        Err(err) => println!("{}", err),
        Ok(()) => {}
    }
}
