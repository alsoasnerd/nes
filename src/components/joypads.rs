use std::collections::HashMap;

use sdl2::keyboard::Keycode;

bitflags! {
    pub struct JoypadButton: u8 {
        const RIGHT             = 0b1000_0000;
        const LEFT              = 0b0100_0000;
        const DOWN              = 0b0010_0000;
        const UP                = 0b0001_0000;
        const START             = 0b0000_1000;
        const SELECT            = 0b0000_0100;
        const BUTTON_B          = 0b0000_0010;
        const BUTTON_A          = 0b0000_0001;
    }
}

pub struct Joypad {
    strobe_mode: bool,
    button_index: u8,
    button_status: JoypadButton,
    pub keymap: HashMap<Keycode, JoypadButton>
}



impl Joypad {
    pub fn new() -> Self {
        let mut keymap = HashMap::new();
        keymap.insert(Keycode::W, JoypadButton::UP);
        keymap.insert(Keycode::A, JoypadButton::LEFT);
        keymap.insert(Keycode::S, JoypadButton::DOWN);
        keymap.insert(Keycode::D, JoypadButton::RIGHT);
        keymap.insert(Keycode::Space, JoypadButton::BUTTON_A);
        keymap.insert(Keycode::E, JoypadButton::BUTTON_B);
        keymap.insert(Keycode::Return, JoypadButton::START);
        keymap.insert(Keycode::Tab, JoypadButton::SELECT);

        Joypad {
            strobe_mode: false,
            button_index: 0,
            button_status: JoypadButton::from_bits_truncate(0b0000_0000),
            keymap
        }
    }

    pub fn write(&mut self, value: u8) {
        self.strobe_mode = value & 1 == 1;

        if self.strobe_mode {
            self.button_index = 0;
        }
    }

    pub fn read(&mut self) -> u8 {
        if self.button_index > 7 {
            return 1;
        }

        let value = (self.button_status.bits & (1 << self.button_index)) >> self.button_index;

        if !self.strobe_mode && self.button_index <= 7 {
            self.button_index += 1;
        }

        value
    }
}