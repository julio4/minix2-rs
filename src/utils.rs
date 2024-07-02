use std::fmt;

/// Hexdump style output formatter
pub struct HexdumpFormatter<'a>(pub &'a [u8]);

impl<'a> fmt::Debug for HexdumpFormatter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for (i, byte) in self.0.iter().enumerate() {
            if i % 16 == 0 {
                s.push_str(&format!("{:08x}  ", i));
            }
            // Add one spaces when reaching 8 bytes
            if i % 8 == 0 {
                s.push_str(" ");
            }
            s.push_str(&format!("{:02x} ", byte));
            if i % 16 == 15 {
                s.push_str("\n");
            }
        }
        write!(f, "{}", s)
    }
}

pub fn min<T: PartialOrd>(a: T, b: T) -> T {
    if a < b {
        a
    } else {
        b
    }
}
