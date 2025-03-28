mod grammar;
mod grammar_trait;
mod parser;

use std::{env, fs};

use parol_runtime::Report;

struct JSONErrorReporter;
impl Report for JSONErrorReporter {}

fn main() {
    let path = env::args().nth(1).expect("Expected file argument");
    let src = fs::read_to_string(&path).expect("Failed to read file");

    let mut json_grammar = grammar::Grammar::new();
    match parser::parse(&src, &path, &mut json_grammar) {
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
            let _ = JSONErrorReporter::report_error(&err, &path);
            std::process::exit(1);
        }
    };
}
