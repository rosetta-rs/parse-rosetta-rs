mod parser;

use std::{env, fs};

fn main() {
    let src = fs::read_to_string(env::args().nth(1).expect("Expected file argument"))
        .expect("Failed to read file");

    match parser::parser::json(&src) {
        Ok(json) => {
            println!("{:#?}", json);
        }
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };
}
