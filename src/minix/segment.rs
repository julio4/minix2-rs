use super::error::MinixError;
use crate::utils::HexdumpFormatter;
use std::{
    fmt,
    ops::{Deref, DerefMut},
};

#[derive(PartialEq)]
pub struct Segment<T> {
    pub data: Vec<u8>,
    _marker: std::marker::PhantomData<T>,
}

impl<T> Segment<T> {
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            data,
            _marker: std::marker::PhantomData,
        }
    }
}

// Implement Deref to allow Segment<T> to be treated as a Vec<u8>
impl<T> Deref for Segment<T> {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

// Implement DerefMut to allow mutable access to Segment<T> as a Vec<u8>
impl<T> DerefMut for Segment<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T> fmt::Debug for Segment<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", HexdumpFormatter(&self.data))
    }
}

impl<T> fmt::Display for Segment<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", HexdumpFormatter(&self.data))
    }
}

/// Raw binary data of the text segment.
/// For the high-level representation of the text segment, see `x86::Program`.
#[derive(PartialEq)]
pub struct Text;
impl Text {
    pub fn parse(binary: &[u8], size: u32) -> Result<Segment<Self>, MinixError>
    where
        Self: Sized,
    {
        // get from 33th byte to 33th + size byte
        let b = match binary.get(32..32 + size as usize) {
            Some(b) => b,
            None => return Err(MinixError::InvalidSize),
        };

        Ok(Segment::new(b.to_vec()))
    }
}

/// Raw binary data of the data segment.
#[derive(PartialEq)]
pub struct Data;
impl Data {
    pub fn parse(binary: &[u8], offset: u32, size: u32) -> Result<Segment<Self>, MinixError> {
        // get from offset byte to offset + size byte
        let b = match binary.get((32 + offset as usize)..(32 + offset + size) as usize) {
            Some(b) => b,
            None => return Err(MinixError::InvalidSize),
        };

        Ok(Segment::new(b.to_vec()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::minix::header::Header;

    fn asem_binary() -> Vec<u8> {
        vec![
            // Header part
            0x01, 0x03, 0x20, 0x04, 0x20, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x26, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00,
            0x70, 0x00, 0x00, 0x00, // Text part (0x10 bytes)
            0xbb, 0x00, 0x00, 0xcd, 0x20, 0xbb, 0x10, 0x00, 0xcd, 0x20, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, // Data part (0x26 bytes)
            0x01, 0x00, 0x04, 0x00, 0x01, 0x00, 0x06, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x0a, // Additional data
            0x00, 0x00, 0x00, 0x00,
        ]
    }

    #[test]
    fn test_parse_text_segment() {
        let header = Header::parse(&asem_binary()).unwrap();
        let text_segment = Text::parse(&asem_binary(), header.text).unwrap();

        assert_eq!(
            *text_segment,
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
        let text_segment = Text::parse(&binary, header.text);

        assert_eq!(text_segment, Err(MinixError::InvalidSize));
    }

    #[test]
    fn test_parse_data_segment() {
        let header = Header::parse(&asem_binary()).unwrap();
        let data_segment = Data::parse(&asem_binary(), header.text, header.data).unwrap();

        assert_eq!(
            *data_segment,
            vec![
                0x01, 0x00, 0x04, 0x00, 0x01, 0x00, 0x06, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x0a
            ]
        );
    }
}
