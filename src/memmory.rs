#[derive(Debug)]
pub struct Memmory {
    pub array: [u8; 0xFFFF],
}
impl Default for Memmory {
    fn default() -> Self {
        Self::new()
    }
}
impl Memmory {
    pub fn new() -> Self {
        Memmory { array: [0; 0xFFFF] }
    }
    pub fn read(&self, address: u16) -> u8 {
        self.array[address as usize]
    }

    pub fn write(&mut self, address: u16, data: u8) {
        self.array[address as usize] = data;
    }

    pub fn read_u16(&self, address: u16) -> u16 {
        let low = self.read(address);
        let high = self.read(address + 1);

        u16::from_le_bytes([high, low])
    }

    pub fn write_u16(&mut self, address: u16, data: u16) {
        let low = (data >> 8) as u8;
        let high = (data & 0xFF) as u8;

        self.write(address, low);
        self.write(address + 1, high)
    }
}