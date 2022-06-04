use std::{fmt::Display, str::FromStr};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ChunkType([u8; 4]);

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = &'static str;

    fn try_from(chunktype: [u8; 4]) -> Result<Self, Self::Error> {
        Ok(ChunkType(chunktype))
    }
}

impl FromStr for ChunkType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4{
            return Err("chunktype should have length = 4");
        }
        let mut arr: [u8; 4] = [0, 0, 0, 0];
        for (i, c) in s.bytes().enumerate() {
            if !((c >= 65 && c <= 90) || (c >= 97 && c <= 122)) {
                return Err("Not an ASCII Alphabet");
            }
            arr[i] = c;
        }
        Ok(ChunkType(arr))
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();
        for c in self.0.into_iter() {
            string.push(c as char);
        }
        write!(f, "{}", string)
    }
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.0
    }

    pub fn _is_valid(&self) -> bool {
        for (i, c) in self.bytes().into_iter().enumerate() {
            println!("{} {}", i, c);
            if i == 2 {
                if !(c >= 65 && c <= 90) {
                    return false;
                }
            }
            if !((c >= 65 && c <= 90) || (c >= 97 && c <= 122)) {
                return false;
            }
        }
        true
    }

    pub fn _is_critical(&self) -> bool {
        let byte = self.bytes()[0];
        let bit = byte >> 5 & 1;
        if bit == 0 {
            true
        } else {
            false
        }
    }

    pub fn _is_public(&self) -> bool {
        let byte = self.bytes()[1];
        let bit = byte >> 5 & 1;
        if bit == 0 {
            true
        } else {
            false
        }
    }

    pub fn _is_reserved_bit_valid(&self) -> bool {
        let byte = self.bytes()[2];
        let bit = byte >> 5 & 1;
        if bit == 0 {
            true
        } else {
            false
        }
    }

    pub fn _is_safe_to_copy(&self) -> bool {
        let byte = self.bytes()[3];
        let bit = byte >> 5 & 1;
        if bit == 1 {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

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
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        println!("{:?}", chunk);
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
