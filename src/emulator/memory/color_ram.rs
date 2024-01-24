pub struct ColorRAM {
    data: [u8; 0x0400],
}

impl ColorRAM {
    pub fn new() -> ColorRAM {
        ColorRAM {
            data: [0; 0x0400]
        }
    }

    pub fn get(&self, loc: usize) -> u8 {
        self.data[loc]
    }

    pub fn set(&mut self, loc: usize, value: u8) {
        self.data[loc] = value;
    }
}