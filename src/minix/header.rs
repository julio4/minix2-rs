use super::error::MinixError;
use crate::utils::HexdumpFormatter;
use std::fmt;

#[derive(PartialEq)]
pub struct Header {
    pub(crate) raw: [u8; 32],
    /* Short form: 32 bytes */
    pub magic: [u8; 2],
    pub flags: u8,
    pub cpu: u8,
    pub hdrlen: u8,
    pub unused: u8,
    pub version: u16,
    pub text: u32,
    pub data: u32,
    pub bss: u32,
    pub entry: u32,
    pub total: u32,
    pub syms: u32,
}

impl Header {
    pub fn parse(binary: &[u8]) -> Result<Header, MinixError> {
        // Slice of 32 bytes with Error if less than 32 bytes
        let b = match binary.get(0..32) {
            Some(b) => b,
            None => return Err(MinixError::InvalidSize),
        };

        // little endian
        let header = Header {
            raw: b.try_into().map_err(|_| MinixError::CorruptedData)?,
            magic: b[0..2].try_into().unwrap(),
            flags: b[2],
            cpu: b[3],
            hdrlen: b[4],
            unused: b[5],
            version: u16::from_le_bytes(b[6..8].try_into().map_err(|_| MinixError::CorruptedData)?),
            text: u32::from_le_bytes(b[8..12].try_into().map_err(|_| MinixError::CorruptedData)?),
            data: u32::from_le_bytes(
                b[12..16]
                    .try_into()
                    .map_err(|_| MinixError::CorruptedData)?,
            ),
            bss: u32::from_le_bytes(
                b[16..20]
                    .try_into()
                    .map_err(|_| MinixError::CorruptedData)?,
            ),
            entry: u32::from_le_bytes(
                b[20..24]
                    .try_into()
                    .map_err(|_| MinixError::CorruptedData)?,
            ),
            total: u32::from_le_bytes(
                b[24..28]
                    .try_into()
                    .map_err(|_| MinixError::CorruptedData)?,
            ),
            syms: u32::from_le_bytes(
                b[28..32]
                    .try_into()
                    .map_err(|_| MinixError::CorruptedData)?,
            ),
        };

        Ok(header)
    }
}

impl fmt::Debug for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", HexdumpFormatter(&self.raw))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_header() {
        let binary: Vec<u8> = vec![
            0x01, 0x03, 0x20, 0x04, 0x20, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x26, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00,
            0x70, 0x00, 0x00, 0x00,
        ];
        assert_eq!(binary.len(), 32);

        let header = Header::parse(&binary).unwrap();
        assert_eq!(header.magic, [0x01, 0x03]);
        assert_eq!(header.flags, 0x20);
        assert_eq!(header.cpu, 0x04);
        assert_eq!(header.hdrlen, 0x20);
        assert_eq!(header.unused, 0x00);
        assert_eq!(header.version, 0x0000);
        assert_eq!(header.text, 0x00000010);
        assert_eq!(header.data, 0x00000026);
        assert_eq!(header.bss, 0x00000000);
        assert_eq!(header.entry, 0x00000000);
        assert_eq!(header.total, 0x00010000);
        assert_eq!(header.syms, 0x00000070);
    }

    #[test]
    fn test_parse_invalid_header() {
        let binary: Vec<u8> = vec![0x01, 0x03, 0x20, 0x04, 0x20, 0x00, 0x00, 0x00, 0x10, 0x00];
        assert_eq!(binary.len(), 10);

        let header = Header::parse(&binary);
        assert_eq!(header, Err(MinixError::InvalidSize));
    }
}
