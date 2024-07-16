pub struct Memory {
    pub data: Vec<u8>,
}

#[allow(dead_code)]
impl Memory {
    pub fn new(size: usize) -> Self {
        Memory {
            data: vec![0; size],
        }
    }

    pub fn from(data: Vec<u8>) -> Self {
        Memory { data }
    }

    pub fn read(&self, address: u16) -> u8 {
        self.data[address as usize]
    }

    pub fn read_word(&self, address: u16) -> u16 {
        u16::from_le_bytes([
            self.data[address as usize],
            self.data[(address + 1) as usize],
        ])
    }

    pub fn read_bytes(&self, address: u16, size: usize) -> &[u8] {
        &self.data[address as usize..(address as usize + size)]
    }

    pub fn write(&mut self, address: u16, value: u8) {
        self.data[address as usize] = value;
    }

    pub fn write_word(&mut self, address: u16, value: u16) {
        let bytes = value.to_le_bytes();
        self.data[address as usize] = bytes[0];
        self.data[(address + 1) as usize] = bytes[1];
    }

    pub fn write_bytes(&mut self, address: u16, data: &[u8]) {
        self.data[address as usize..(address as usize + data.len())].copy_from_slice(data);
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}
