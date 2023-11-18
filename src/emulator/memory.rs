const BASIC: &[u8] = include_bytes!("basic.901226-01.bin");
const KERNAL: &[u8] = include_bytes!("kernal.901227-03.bin");
const CHARACTERS: &[u8] = include_bytes!("characters.901225-01.bin");

pub struct Memory {
    data: [u8; 0xFFFF],
}

impl Memory {
    pub fn new() -> Memory {
        let mut m = Memory {
            data: [0; 0xFFFF]
        };
        m.data[0x0000] = 0x2F;
        m.data[0x0001] = 0x37;
        m
    }

    fn get(&self, loc: usize) -> u8 {
        if loc >= 0xA000 && loc <= 0xBFFF && (self.data[0x0001] & 0x03) == 0x03 {
            return BASIC[loc - 0xA000];
        }
        if loc >= 0xE000 && loc <= 0xFFFF && (self.data[0x0001] & 0x02) == 0x02 {
            return KERNAL[loc - 0xE000];
        }
        if loc >= 0xD000 && loc <= 0xDFFF && (self.data[0x0001] & 0x04) == 0x00 && (self.data[0x0001] & 0x03) != 0x00 {
            return CHARACTERS[loc - 0xD000];
        }
        // todo IO area
        self.data[loc]
    }

    pub fn get_from_pc(&self, pc: u16) -> u8 {
        self.get(pc as usize)
    }

    pub fn get_from_low_high(&mut self, low: u8, high: u8) -> u8 {
        self.get((((low as u16) & 0x00FF) | (((high as u16) << 8) & 0xFF00)) as usize)
    }
}