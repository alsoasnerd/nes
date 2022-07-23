pub struct RAM {
    content: [u8; 2048],
}

impl RAM {
    pub fn new() -> Self {
        Self { content: [0; 2048] }
    }

    pub fn read(&self, address: u16) -> u8 {
        self.content[address as usize]
    }

    pub fn write(&mut self, address: u16, value: u8) {
        self.content[address as usize] = value;
    }
}
