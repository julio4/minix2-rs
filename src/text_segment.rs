use std::fmt;

use crate::utils::HexdumpFormatter;

#[derive(PartialEq)]
pub struct TextSegment {
    pub text: Vec<u8>,
}

impl TextSegment {
    pub fn parse(binary: &[u8], size: u32) -> Result<TextSegment, &str> {
        // get from 33th byte to 33th + size byte
        let b = match binary.get(32..32 + size as usize) {
            Some(b) => b,
            None => return Err("Incorrect text segment size"),
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
        /* For now we hardcode:
        0000: bb0000   mov bx, 0000
        0003: cd20     int 20
        0005: b1000    mov bx, 0010
        0008: cd20     int 20
        000a: 0000     add [bx+si], al
        000c: 0000     add [bx+si], al
        000e: 0000     add [bx+si], al
        */
        // Next step will be to lex/parse text segment
        let str = "0000: bb0000   mov bx, 0000\n0003: cd20     int 20\n0005: b1000    mov bx, 0010\n0008: cd20     int 20\n000a: 0000     add [bx+si], al\n000c: 0000     add [bx+si], al\n000e: 0000     add [bx+si], al";
        write!(f, "{}", str)
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

        assert_eq!(text_segment, Err("Incorrect text segment size"));
    }
}
