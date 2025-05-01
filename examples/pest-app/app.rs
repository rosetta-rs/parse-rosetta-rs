// pest. The Elegant Parser
// Copyright (c) 2018 Dragoș Tiselice
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

mod parser;

use std::{env, fs};

fn main() {
    let src = fs::read_to_string(env::args().nth(1).expect("Expected file argument"))
        .expect("Failed to read file");

    match parser::Json::parse(&src) {
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
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };
}
