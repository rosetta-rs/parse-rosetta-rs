mod parser;

use std::{env, fs};

use codespan_reporting::diagnostic::Severity;
use codespan_reporting::files::SimpleFile;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use codespan_reporting::term::{self, Config};
use logos::Logos as _;

fn main() {
    let path = env::args().nth(1).expect("Expected file argument");
    let src = fs::read_to_string(&path).expect("Failed to read file");

    let mut diags = vec![];
    let (tokens, ranges) = parser::tokenize(parser::Token::lexer(&src), &mut diags);
    let cst = parser::Parser::parse(&src, tokens, ranges, &mut diags);

    #[cfg(debug_assertions)]
    {
        println!("{}", cst);
    }
    #[cfg(not(debug_assertions))]
    {
        std::hint::black_box(cst);
    }

    if !diags.is_empty() {
        let writer = StandardStream::stderr(ColorChoice::Auto);
        let config = Config::default();
        let file = SimpleFile::new(&path, &src);
        for diag in diags.iter() {
            term::emit(&mut writer.lock(), &config, &file, diag).unwrap();
        }
        if diags.iter().any(|d| d.severity == Severity::Error) {
            std::process::exit(1);
        }
    }
}
