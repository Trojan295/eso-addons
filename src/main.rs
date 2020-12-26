extern crate clap;
extern crate dirs;

mod cli;

fn main() {
    match cli::run() {
        Err(error) => {
            println!("Failed to execute: {}", error)
        }
        _ => {}
    }
}
