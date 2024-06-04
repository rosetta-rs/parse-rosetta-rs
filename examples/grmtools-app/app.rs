use lrlex::lrlex_mod;
use lrpar::lrpar_mod;
use std::{env, fs};

lrlex_mod!("json.l");
lrpar_mod!("json.y");

mod json_val;

fn main() {
    let src = fs::read_to_string(env::args().nth(1).expect("Expected file argument"))
        .expect("Failed to read file");

    let lexerdef = json_l::lexerdef();
    let lexer = lexerdef.lexer(&src);
    let (res, errs) = json_y::parse(&lexer);
    for e in errs {
        println!("{}", e.pp(&lexer, &json_y::token_epp));
    }
    match res {
        Some(r) => {
            #[cfg(debug_assertions)]
            println!("{r:#?}");
            #[cfg(not(debug_assertions))]
            let _ = std::hint::black_box(r);
        }
        None => panic!(),
    }
}
