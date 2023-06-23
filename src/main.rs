mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use crate::args::Cli;
use clap::Parser;
use std::fs;

fn main() {
    let args = Cli::parse();
    let file_as_bytes1 = fs::read(&args.png_path).expect("Failed to read file");
    println!("PNG PRE: {:?}\n\n\n", &file_as_bytes1);
    let encoded_png = commands::encode(&file_as_bytes1, args.chunk_type, args.message).unwrap();
    fs::write(&args.png_path, encoded_png.as_bytes()).expect("Failed to write");
    let file_as_bytes2 = fs::read(&args.png_path).expect("Failed to read file");

    println!("PNG POST: {:?}\n\n\n", &file_as_bytes2);
    println!("{:?}", file_as_bytes1 == file_as_bytes2);
    // Decode PNG with test message
    let chunk_t2: String = "RuSt".to_owned();
    match commands::decode(&file_as_bytes2, chunk_t2) {
        Ok(_) => println!("Decode successful\n\n\n"),
        Err(e) => eprintln!("{}", e),
    }
}
