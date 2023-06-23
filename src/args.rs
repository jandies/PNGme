use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version)]
pub struct Cli {
    // Message to encrypt
    pub message: String,

    /// Path to the PNG file to encode/decode
    #[arg(short, long, default_value = "image.png")]
    pub png_path: String,

    // 4 letter chunk type code
    #[arg(short, long, default_value = "RuSt")]
    pub chunk_type: String,
}
