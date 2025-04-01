//! This is a parser for JSON.
//! Run it with the following command:
//! cargo run --example json -- examples/sample.json

mod parser;

use std::{env, fs};

use chumsky::Parser;

fn main() {
    let src = fs::read_to_string(env::args().nth(1).expect("Expected file argument"))
        .expect("Failed to read file");

    let (json, errs) = parser::parser().parse(&src).into_output_errors();
    #[cfg(debug_assertions)]
    {
        println!("{:#?}", json);
    }
    #[cfg(not(debug_assertions))]
    {
        std::hint::black_box(json);
    }
    for err in errs {
        eprintln!("{err}");
    }
}
