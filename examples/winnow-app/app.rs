mod json;
mod parser;

use std::{env, fs};

use winnow::error::ContextError;
use winnow::prelude::*;

fn main() {
    let src = fs::read_to_string(env::args().nth(1).expect("Expected file argument"))
        .expect("Failed to read file");

    match parser::json::<ContextError>
        .parse(src.as_str())
        .map_err(|e| e.to_string())
    {
        Ok(json) => {
            println!("{:#?}", json);
        }
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
}
