extern crate lalrpop_util;

use std::env;
use std::fs;

#[rustfmt::skip]
#[allow(clippy::all)]
mod json;
mod json_val;

fn main() {
    let src = fs::read_to_string(env::args().nth(1).expect("Expected file argument"))
        .expect("Failed to read file");

    match json::ValueParser::new().parse(&src) {
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
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
}
