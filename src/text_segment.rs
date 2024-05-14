use std::fmt;

use crate::disassembler::error::ParseError;
use crate::utils::HexdumpFormatter;

/// Raw binary data of the text segment.
/// For the high-level representation of the text segment, see `Program`.
#[derive(PartialEq)]
pub struct TextSegment {
    pub text: Vec<u8>,
}

impl TextSegment {
    pub fn parse(binary: &[u8], size: u32) -> Result<TextSegment, ParseError> {
        // get from 33th byte to 33th + size byte
        let b = match binary.get(32..32 + size as usize) {
            Some(b) => b,
            None => return Err(ParseError::InvalidSize),
        };

        Ok(TextSegment { text: b.to_vec() })
    }
}

impl fmt::Debug for TextSegment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", HexdumpFormatter(&self.text))
    }
}

impl fmt::Display for TextSegment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", HexdumpFormatter(&self.text))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::header::Header;

    #[test]
    fn test_parse_text_segment() {
        let binary: Vec<u8> = vec![
            // Header part
            0x01, 0x03, 0x20, 0x04, 0x20, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x26, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00,
            0x70, 0x00, 0x00, 0x00, // Text part (16 bytes)
            0xbb, 0x00, 0x00, 0xcd, 0x20, 0xbb, 0x10, 0x00, 0xcd, 0x20, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ];
        assert_eq!(binary.len(), 32 + 16);

        let header = Header::parse(&binary).unwrap();
        let text_segment = TextSegment::parse(&binary, header.text).unwrap();

        assert_eq!(
            text_segment.text,
            vec![
                0xbb, 0x00, 0x00, 0xcd, 0x20, 0xbb, 0x10, 0x00, 0xcd, 0x20, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00
            ]
        );
    }

    #[test]
    fn test_parse_invalid_text_segment() {
        let binary: Vec<u8> = vec![
            // Header part
            0x01, 0x03, 0x20, 0x04, 0x20, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x26, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00,
            0x70, 0x00, 0x00, 0x00, // Text part (should be 16 bytes, but only 5 bytes)
            0xbb, 0x00, 0x00, 0xcd, 0x20,
        ];

        let header = Header::parse(&binary).unwrap();
        let text_segment = TextSegment::parse(&binary, header.text);

        assert_eq!(text_segment, Err(ParseError::InvalidSize));
    }
}
