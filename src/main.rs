#[macro_use]
extern crate clap;
extern crate dirs;
#[macro_use]
extern crate simple_error;
#[macro_use]
extern crate prettytable;

mod cli;

fn main() {
    match cli::run() {
        Err(error) => {
            println!("Failed to execute: {}", error)
        }
        _ => {}
    }
}
