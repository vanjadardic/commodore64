use crate::emulator::memory::cia1::Cia1;
use crate::emulator::memory::cia2::Cia2;
use crate::emulator::memory::color_ram::ColorRAM;
use crate::emulator::memory::gpu::Gpu;

mod color_ram;
mod cia2;
mod gpu;
pub mod cia1;

const BASIC: &[u8] = include_bytes!("basic.901226-01.bin");
const KERNAL: &[u8] = include_bytes!("kernal.901227-03.bin");
const CHARACTERS: &[u8] = include_bytes!("characters.901225-01.bin");

pub struct Memory {
    data: [u8; 0xFFFF],
    color_ram: ColorRAM,
    cia1: Cia1,
    cia2: Cia2,
    gpu: Gpu,
}

impl Memory {
    pub fn new() -> Memory {
        let mut data = [0; 0xFFFF];
        data[0x0000] = 0x2F;
        data[0x0001] = 0x37;
        Memory {
            data,
            color_ram: ColorRAM::new(),
            cia1: Cia1::new(),
            cia2: Cia2::new(),
            gpu: Gpu::new(),
        }
    }

    fn get(&mut self, loc: usize) -> u8 {
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
            if loc >= 0xD400 && loc <= 0xD7FF {
                //debug!("SID get {:04X}", loc);
            } else if loc >= 0xD800 && loc <= 0xDBFF {
                return self.color_ram.get(loc - 0xD800);
            } else if loc >= 0xDC00 && loc <= 0xDCFF {
                return self.cia1.get(((loc - 0xDC00) % 16) + 0xDC00);
            } else if loc >= 0xDD00 && loc <= 0xDDFF {
                return self.cia2.get(((loc - 0xDD00) % 16) + 0xDD00);
            } else if loc >= 0xDE00 && loc <= 0xDEFF {
                //debug!("I/O Area #1 get {:04X}", loc);
            } else if loc >= 0xDF00 && loc <= 0xDFFF {
                //debug!("I/O Area #2 get {:04X}", loc);
            }
        }
        self.data[loc]
    }

    pub fn get_from_gpu(&self, loc: usize) -> u8 {
        if (loc >= 0x1000 && loc < 0x2000) || (loc >= 0x9000 && loc < 0xA000) {
            return CHARACTERS[(loc % 0x8000) - 0x1000];
        }
        self.data[loc]
    }

    fn set(&mut self, loc: usize, value: u8) {
        if loc >= 0xD000 && loc <= 0xDFFF && (self.data[0x0001] & 0x03) != 0x00 {
            if (self.data[0x0001] & 0x04) == 0x00 {
                //charrom noop
            }
            if (self.data[0x0001] & 0x04) != 0x00 {
                if loc >= 0xD000 && loc <= 0xD3FF {
                    self.gpu.set(((loc - 0xD000) % 64) + 0xD000, value);
                } else if loc >= 0xD400 && loc <= 0xD7FF {
                    //debug!("SID get {:04X} = {:02X}", loc, value);
                } else if loc >= 0xD800 && loc <= 0xDBFF {
                    self.color_ram.set(loc - 0xD800, value);
                } else if loc >= 0xDC00 && loc <= 0xDCFF {
                    return self.cia1.set(((loc - 0xDC00) % 16) + 0xDC00, value);
                } else if loc >= 0xDD00 && loc <= 0xDDFF {
                    return self.cia2.set(((loc - 0xDD00) % 16) + 0xDD00, value);
                } else if loc >= 0xDE00 && loc <= 0xDEFF {
                    //debug!("I/O Area #1 set {:04X} = {:02X}", loc, value);
                } else if loc >= 0xDF00 && loc <= 0xDFFF {
                    //debug!("I/O Area #2 set {:04X} = {:02X}", loc, value);
                }
                return;
            }
        }
        // if loc == 0x0001 {
        //     let a = loc;
        //println!("set mem {:04X}={}", loc, value);
        // }
        if loc == (0xc400 + 46) {
            //debug!("memset {:04X} = {:02X}", loc, value);
        }
        self.data[loc] = value;
    }

    pub fn set_from_low(&mut self, low: u8, value: u8) {
        self.set(low as usize, value);
    }

    pub fn get_from_low(&mut self, low: u8) -> u8 {
        self.get(low as usize)
    }

    pub fn set_from_low_high(&mut self, low: u8, high: u8, value: u8) {
        self.set((((low as u16) & 0x00FF) | (((high as u16) << 8) & 0xFF00)) as usize, value);
    }

    pub fn get_from_word(&mut self, loc: u16) -> u8 {
        self.get(loc as usize)
    }

    pub fn get_from_low_high(&mut self, low: u8, high: u8) -> u8 {
        self.get((((low as u16) & 0x00FF) | (((high as u16) << 8) & 0xFF00)) as usize)
    }

    pub fn set_stack(&mut self, sp: u8, value: u8) {
        self.data[sp as usize | 0x0100] = value;
    }

    pub fn get_stack(&self, sp: u8) -> u8 {
        self.data[sp as usize | 0x0100]
    }

    pub fn cia1(&mut self) -> &mut Cia1 {
        &mut self.cia1
    }

    pub fn cia2(&self) -> &Cia2 {
        &self.cia2
    }

    pub fn gpu(&self) -> &Gpu {
        &self.gpu
    }

    pub fn color_ram(&self) -> &ColorRAM {
        &self.color_ram
    }
}