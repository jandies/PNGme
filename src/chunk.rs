#![allow(unused_variables, unused_imports, dead_code)]
use super::chunk_type::ChunkType;
use std::{fmt::Display, string::FromUtf8Error};
use thiserror::Error;

#[derive(Debug)]
pub struct Chunk {
    length: u32,
    pub chunk_type: ChunkType,
    pub data: Vec<u8>,
    crc: u32,
}

#[derive(Debug, Error)]
pub enum ChunkError {
    #[error("Failure generating Chunk")]
    ChunkError,
    #[error("Failure generating chunk UTF-8 data")]
    UTF8ConversionError(#[from] std::string::FromUtf8Error),
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let chunk_array: &[u8] = &chunk_type.bytes;
        let data_array = &data[..];
        let combined_array = [chunk_array, data_array].concat();
        let crc = crc32fast::hash(&combined_array);

        Chunk {
            length: data.len() as u32,
            chunk_type,
            data,
            crc,
        }
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// The CRC of this chunk
    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn data_as_string(&self) -> Result<String, FromUtf8Error> {
        let data = self.data.clone();
        let string_data = String::from_utf8(data);
        match string_data {
            Ok(string) => Ok(string),
            Err(e) => Err(e),
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::new();
        let length_bytes = self.length.to_be_bytes().to_vec();
        let chunk_type = self.chunk_type.bytes.to_vec();
        let data = self.data.clone();
        let crc = self.crc.to_be_bytes().to_vec();

        vec.extend_from_slice(&length_bytes);
        vec.extend_from_slice(&chunk_type);
        vec.extend_from_slice(&data);
        vec.extend_from_slice(&crc);
        vec
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = ChunkError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() < 12 {
            return Err(ChunkError::ChunkError);
        };
        // length bytes
        let length = &value[0..4];
        let chunk_type = &value[4..8];
        let data = &value[8..(value.len() - 4)];
        let crc = &value[value.len() - 4..];

        let calc_crc_array = [chunk_type, data].concat();
        let calc_crc = crc32fast::hash(&calc_crc_array);
        if u32::from_be_bytes([crc[0], crc[1], crc[2], crc[3]]) != calc_crc {
            return Err(ChunkError::ChunkError);
        }

        Ok(Self {
            length: u32::from_be_bytes([length[0], length[1], length[2], length[3]]),
            chunk_type: ChunkType {
                bytes: [chunk_type[0], chunk_type[1], chunk_type[2], chunk_type[3]],
            },
            data: data.to_vec(),
            crc: u32::from_be_bytes([crc[0], crc[1], crc[2], crc[3]]),
        })
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Chunk {{",)?;
        writeln!(f, "  Length: {}", self.length())?;
        writeln!(f, "  Type: {}", self.chunk_type())?;
        writeln!(f, "  Data: {} bytes", self.data().len())?;
        writeln!(f, "  Crc: {}", self.crc())?;
        writeln!(f, "}}",)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
