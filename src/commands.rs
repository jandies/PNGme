#![allow(unused_variables, unused_imports, dead_code, clippy::enum_variant_names)]
use std::str::FromStr;

use crate::chunk::ChunkError;
use crate::chunk_type::ChunkTypeError;
use crate::png::PngError;

use super::png::Png;
use super::chunk::Chunk;
use super::chunk_type::ChunkType;
use clap::Command;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CommandErrors {
    #[error("Error initialising command")]
    CommandError,
    #[error("Unable to decode PNG - invalid chunktype")]
    DecodeError,
    #[error("Error initialising PNG")]
    PngError(#[from] PngError),
    #[error("Error initialising ChunkType")]
    ChunkTypeError(#[from] ChunkTypeError),
    #[error("Error initialising Chunk")]
    ChunkError(#[from] ChunkError),

}

pub fn encode(file_as_bytes: &Vec<u8>, chunk_t: String, message: String ) -> Result<Png, CommandErrors> {
    let mut png = Png::try_from(&file_as_bytes[..])?;
    let chunk_type_converted = ChunkType::from_str(&chunk_t)?;
    let chunk = Chunk::new(chunk_type_converted, message.as_bytes().to_vec());
    png.append_chunk(chunk);
    Ok(png)
}

pub fn decode(file_as_bytes: &Vec<u8>, chunk_t: String) -> Result<(), CommandErrors> {
    let png = Png::try_from(&file_as_bytes[..])?;
    match png.chunk_by_type(&chunk_t) {
        Some(chunk) => { 
            let decoded_data = chunk.data_as_string();
            match decoded_data {
                Ok(data) => { println!("Decoded message: {}", data); Ok(()) },
                Err(e) => Err(CommandErrors::DecodeError)
            }
        },
        None => Err(CommandErrors::DecodeError)
    }
}