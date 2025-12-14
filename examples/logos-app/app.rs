mod parser;

use std::{env, fs};

use logos::Logos as _;

use crate::parser::JsonLexer;

fn main() {
    let filename = env::args().nth(1).expect("Expected file argument");
    let src = fs::read_to_string(&filename).expect("Failed to read file");

    let mut lexer = JsonLexer(parser::Token::lexer(src.as_str()));
    match parser::parse_value(&mut lexer) {
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
        Err((msg, span)) => {
            eprintln!("{filename}:{span:?}: {msg}");
        }
    }
}
