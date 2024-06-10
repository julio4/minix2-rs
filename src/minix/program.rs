use super::error::MinixError;
use super::header::Header;
use super::segment::{Data, Segment, Text};

use std::fs::File;
use std::io::Read;

pub struct Program {
    pub header: Header,
    pub text_segment: Segment<Text>,
    pub data_segment: Segment<Data>,
}

impl Program {
    fn new(header: Header, text_segment: Segment<Text>, data_segment: Segment<Data>) -> Self {
        Program {
            header,
            text_segment,
            data_segment,
        }
    }

    pub fn from_file(file: File) -> Result<Self, MinixError> {
        let binary = file
            .bytes()
            .map(|b| b.map_err(|_| MinixError::InvalidFile))
            .collect::<Result<Vec<u8>, MinixError>>()?;

        let header = Header::parse(&binary)?;
        let text_segment = Text::parse(&binary, header.text)?;
        let data_segment = Data::parse(&binary, header.text, header.data)?;

        Ok(Program::new(header, text_segment, data_segment))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program_from_file() {
        let file = File::open("./tests_data/asem/1.s.out").unwrap();
        let program = Program::from_file(file).unwrap();

        assert_eq!(
            program.header.raw.to_vec(),
            vec![
                0x01, 0x03, 0x20, 0x04, 0x20, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x26, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00,
                0x70, 0x00, 0x00, 0x00
            ]
        );

        assert_eq!(
            *program.text_segment,
            vec![
                0xbb, 0x00, 0x00, 0xcd, 0x20, 0xbb, 0x10, 0x00, 0xcd, 0x20, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00
            ]
        );

        assert_eq!(
            *program.data_segment,
            vec![
                0x01, 0x00, 0x04, 0x00, 0x01, 0x00, 0x06, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x0a
            ]
        );
    }
}
