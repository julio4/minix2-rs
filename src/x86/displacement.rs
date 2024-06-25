#[derive(Debug, PartialEq)]
pub enum Displacement {
    Short(i8),
    Long(i16),
}

impl Displacement {
    pub fn is_neg(&self) -> bool {
        match self {
            Displacement::Short(d) => d.is_negative(),
            Displacement::Long(d) => d.is_negative(),
        }
    }
}

impl Into<u16> for Displacement {
    fn into(self) -> u16 {
        match self {
            Displacement::Short(d) => d as u16,
            Displacement::Long(d) => d as u16,
        }
    }
}

impl std::fmt::Display for Displacement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Displacement::Short(d) => write!(f, "{:x}", d),
            Displacement::Long(d) => {
                if d.is_negative() {
                    write!(f, "-{:x}", d.abs())
                } else {
                    write!(f, "{:x}", d)
                }
            }
        }
    }
}
