use crate::emulator::gpu::Gpu;

const BASIC: &[u8] = include_bytes!("basic.901226-01.bin");
const KERNAL: &[u8] = include_bytes!("kernal.901227-03.bin");
const CHARACTERS: &[u8] = include_bytes!("characters.901225-01.bin");

pub struct Memory {
    data: [u8; 0xFFFF],
    pub gpu: Gpu,
}

impl Memory {
    pub fn new() -> Memory {
        let mut m = Memory {
            data: [0; 0xFFFF],
            gpu: Gpu::new(),
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
        if loc >= 0xD000 && loc <= 0xDFFF && (self.data[0x0001] & 0x03) != 0x00 {
            if (self.data[0x0001] & 0x04) == 0x00 {
                return CHARACTERS[loc - 0xD000];
            }
            if loc >= 0xD000 && loc <= 0xD3FF {
                return self.gpu.get(((loc - 0xD000) % 64) + 0xD000);
            }
        }
        self.data[loc]
    }

    fn set(&mut self, loc: usize, value: u8) {
        if loc >= 0xD000 && loc <= 0xDFFF && (self.data[0x0001] & 0x03) != 0x00 {
            if (self.data[0x0001] & 0x04) != 0x00 {
                if loc >= 0xD000 && loc <= 0xD3FF {
                    self.gpu.set(((loc - 0xD000) % 64) + 0xD000, value);
                    return;
                }
            }
        }
        // if loc == 0x0001 {
        //     let a = loc;
        //println!("set mem {:04X}={}", loc, value);
        // }
        self.data[loc] = value;
    }

    pub fn set_from_low(&mut self, low: u8, value: u8) {
        self.set(low as usize, value);
    }

    pub fn get_from_low(&self, low: u8) -> u8 {
        self.get(low as usize)
    }

    pub fn set_from_low_high(&mut self, low: u8, high: u8, value: u8) {
        self.set((((low as u16) & 0x00FF) | (((high as u16) << 8) & 0xFF00)) as usize, value);
    }

    pub fn get_from_word(&self, loc: u16) -> u8 {
        self.get(loc as usize)
    }

    pub fn get_from_low_high(&self, low: u8, high: u8) -> u8 {
        self.get((((low as u16) & 0x00FF) | (((high as u16) << 8) & 0xFF00)) as usize)
    }

    pub fn set_stack(&mut self, sp: u8, value: u8) {
        self.set_from_low_high(sp, 0x01, value);
    }

    pub fn get_stack(&self, sp: u8) -> u8 {
        self.get_from_low_high(sp, 0x01)
    }
}