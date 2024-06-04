use cfgrammar::yacc::YaccKind;
use lrlex::CTLexerBuilder;
use std::{env, path::PathBuf};

fn main() {
    CTLexerBuilder::new()
        .lrpar_config(|ctp| {
            ctp.yacckind(YaccKind::Grmtools)
                .grammar_path("json.y")
                .output_path(
                    [env::var("OUT_DIR").unwrap().as_str(), "json.y.rs"]
                        .iter()
                        .collect::<PathBuf>(),
                )
                .mod_name("json_y")
        })
        .lexer_path("json.l")
        .output_path(
            [env::var("OUT_DIR").unwrap().as_str(), "json.l.rs"]
                .iter()
                .collect::<PathBuf>(),
        )
        .mod_name("json_l")
        .build()
        .unwrap();
}
