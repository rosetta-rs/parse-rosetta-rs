#[macro_use]
extern crate combine;

mod parser;

use std::{env, fs};

use combine::Parser;

fn main() {
    let src = fs::read_to_string(env::args().nth(1).expect("Expected file argument"))
        .expect("Failed to read file");

    let mut parser = parser::json_value();
    match parser.easy_parse(src.as_bytes()) {
        Ok(json) => {
            #[cfg(debug_assertions)]
            {
                println!("{:#?}", json);
            }
            #[cfg(not(debug_assertions))]
            {
                std::hint::black_box(json);
            }
        }
        Err(err) => {
            eprintln!("{:#?}", err);
            std::process::exit(1);
        }
    };
}
