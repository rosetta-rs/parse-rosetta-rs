use std::{env, fs};

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let src = fs::read_to_string(env::args().nth(1).expect("Expected file argument"))
        .expect("Failed to read file");

    println!("{:#?}", src);

    Ok(())
}
