use std::env;
use std::process;

use neis_calculator::{parse_arg, run};

fn main() {
    let filename = parse_arg(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(filename) {
        eprintln!("Application Error: {}", e);
        process::exit(1);
    }
}