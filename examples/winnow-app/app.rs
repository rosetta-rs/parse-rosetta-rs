mod json;
mod parser;

use std::{env, fs};

use winnow::error::VerboseError;
use winnow::prelude::*;

fn main() {
    let src = fs::read_to_string(env::args().nth(1).expect("Expected file argument"))
        .expect("Failed to read file");

    match parser::json::<VerboseError<parser::Stream<'_>>>
        .parse(src.as_str())
        .map_err(VerboseError::into_owned)
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
