pub struct Gpu {
    memory_control_register: u8,
    border_color: u8,
    background_color: u8,
}

impl Gpu {
    pub fn new() -> Gpu {
        Gpu {
            memory_control_register: 0,
            border_color: 0,
            background_color: 0,
        }
    }

    pub fn get(&self, loc: usize) -> u8 {
        if loc == 0xD018 {
            return self.memory_control_register;
        } else if loc == 0xD020 {
            return self.border_color;
        } else if loc == 0xD021 {
            return self.background_color;
        }
        //debug!("gpu get {:04X} ", loc);
        0
    }

    pub fn set(&mut self, loc: usize, value: u8) {
        if loc == 0xD018 {
            self.memory_control_register = value;
        } else if loc == 0xD020 {
            self.border_color = value;
        } else if loc == 0xD021 {
            self.background_color = value;
        } else {
            //debug!("gpu set {:04X} = {:02X}", loc, value);
        }
    }

    pub fn get_video_matrix_address(&self) -> u16 {
        ((self.memory_control_register as u16) << 6) & 0x3C00
    }

    pub fn get_character_bitmap_address(&self) -> u16 {
        ((self.memory_control_register as u16) << 10) & 0x3C00
    }

    pub fn background_color(&self) -> u8 {
        self.background_color
    }
}