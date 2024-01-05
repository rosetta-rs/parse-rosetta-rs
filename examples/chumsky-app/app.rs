//! This is a parser for JSON.
//! Run it with the following command:
//! cargo run --example json -- examples/sample.json

mod parser;

use std::{env, fs};

use ariadne::{Color, Fmt, Label, Report, ReportKind, Source};
use chumsky::Parser;

fn main() {
    let src = fs::read_to_string(env::args().nth(1).expect("Expected file argument"))
        .expect("Failed to read file");

    let (json, errs) = parser::parser()
        .parse(src.trim().as_bytes())
        .into_output_errors();
    println!("{:#?}", json);
    errs.into_iter().for_each(|e| {
        Report::build(ReportKind::Error, (), e.span().start)
            .with_message(e.to_string())
            .with_label(
                Label::new(e.span().into_range())
                    .with_message("Unexpected token")
                    .with_color(Color::Red),
            )
            .finish()
            .print(Source::from(&src))
            .unwrap()
    });
}
