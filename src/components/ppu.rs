pub struct AddressRegister {
    low: u8,
    high: u8,
    high_pointer: bool
}

impl AddressRegister {
    pub fn new() -> Self {
        Self {
            low: 0,
            high: 0,
            high_pointer: true
        }
    }

    pub fn get(&self) -> u16 {
        let high_u16 = self.high as u16;
        let low_u16 = self.low as u16;

        (high_u16 << 8) | low_u16
    }

    pub fn set(&mut self, value: u16) {
        self.high = (value >> 8) as u8;
        self.low = (value & 0xff) as u8;
    }

    pub fn update(&mut self, value: u8) {
        if self.high_pointer {
            self.high = value;
        } else {
            self.low = value;
        }

        if self.get() > 0x3fff {
            self.set(self.get() & 0b11111111111111);
        }

        self.high_pointer = self.high_pointer;
    }

    pub fn increment(&mut self, value: u8) {
        let old_low = self.low;
        self.low = self.low.wrapping_add(value);

        if old_low > self.low {
            self.high = self.high.wrapping_add(1);
        }

        if self.get() > 0x3fff {
            self.set(self.get() & 0b11111111111111);
        }
    }

    pub fn reset_latch(&mut self) {
        self.high_pointer = true;
    }
}