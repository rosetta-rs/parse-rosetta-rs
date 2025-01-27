mod parser;

use std::{env, fs};

use nom::Err;
use nom_language::error::convert_error;
use nom_language::error::VerboseError;

fn main() {
    let src = fs::read_to_string(env::args().nth(1).expect("Expected file argument"))
        .expect("Failed to read file");

    match parser::root::<VerboseError<&str>>(src.as_str()) {
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
        Err(Err::Error(err)) | Err(Err::Failure(err)) => {
            let err = convert_error(src.as_str(), err);
            eprintln!("{}", err);
            std::process::exit(1);
        }
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
}
