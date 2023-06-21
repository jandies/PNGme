mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use std::fs;

fn main() {

    // Obtain PNG as Vec<u8>
    let file_path = "image.png";
    let file_as_bytes1 = fs::read(file_path).expect("Failed to read file");

    // Encode PNG with test message
    let message = "This is our secrete message".to_owned();
    let chunk_t = "RuSt".to_owned();

    println!("PNG PRE: {:?}\n\n\n", &file_as_bytes1);
    let encoded_png = commands::encode(&file_as_bytes1, chunk_t, message).unwrap();
    fs::write(file_path, encoded_png.as_bytes()).expect("Failed to write");
    let file_as_bytes2 = fs::read(file_path).expect("Failed to read file");

    println!("PNG POST: {:?}\n\n\n", &file_as_bytes2);
    println!("{:?}", file_as_bytes1 == file_as_bytes2);
    // Decode PNG with test message
    let chunk_t2 = "RuSt".to_owned();
    match commands::decode(&file_as_bytes2, chunk_t2) {
        Ok(_) => println!("Decode successful\n\n\n"),
        Err(e) => eprintln!("{}", e)
    }
}
