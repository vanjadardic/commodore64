pub struct Memory {
    data: [u8; 0xFFFF],
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            data: [0; 0xFFFF]
        }
    }

    pub fn get_from_pc(&self, pc: u16) -> u8 {
        self.data[pc as usize]
    }

    pub fn get_from_low_high(&mut self, low: u8, high: u8) -> u8 {
        self.data[(((low as u16) & 0x00FF) | (((high as u16) << 8) & 0xFF00)) as usize]
    }

    pub fn set(&mut self, pc: u16, data: u8) {
        self.data[pc as usize] = data;
    }
}