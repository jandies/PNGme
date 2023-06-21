#![allow(unused_variables, unused_imports, dead_code)]
use std::convert::TryFrom;
use std::fmt::Display;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ChunkTypeError {
    #[error("Failure converting from bytes to ChunkType")]
    ByteConversionError,
    #[error("Failure converting from string to ChunkType")]
    StringConversionError,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChunkType {
    pub bytes: [u8; 4],
}

impl ChunkType {
    pub fn is_valid_byte(b: u8) -> bool {
        (65..=90).contains(&b) || (97..=122).contains(&b)
    }

    pub fn bytes(&self) -> [u8; 4] {
        self.bytes
    }

    pub fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
    }

    pub fn is_critical(&self) -> bool {
        (65..=90).contains(&self.bytes[0])
    }

    pub fn is_public(&self) -> bool {
        (65..=90).contains(&self.bytes[1])
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        (65..=90).contains(&self.bytes[2])
    }

    pub fn is_safe_to_copy(&self) -> bool {
        (97..=122).contains(&self.bytes[3])
    }

    pub fn as_string(&self) -> &str {
        std::str::from_utf8(&self.bytes).unwrap()
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = ChunkTypeError;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        for byte in value {
            if !Self::is_valid_byte(byte) {
                return Err(Self::Error::ByteConversionError);
            }
        }
        Ok(Self { bytes: value })
    }
}

impl FromStr for ChunkType {
    type Err = ChunkTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            return Err(Self::Err::StringConversionError);
        }

        let s_bytes = s.as_bytes();
        for byte in s_bytes {
            if !Self::is_valid_byte(*byte) {
                return Err(Self::Err::ByteConversionError);
            }
        }
        let s_byte_array = [s_bytes[0], s_bytes[1], s_bytes[2], s_bytes[3]];

        Ok(Self {
            bytes: s_byte_array,
        })
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = String::from_utf8(self.bytes.to_vec()).unwrap();
        write!(f, "{}", string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
