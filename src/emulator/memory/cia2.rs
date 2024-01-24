use log::debug;

pub struct Cia2 {
    port_a: u8,
}

impl Cia2 {
    pub fn new() -> Cia2 {
        Cia2 {
            port_a: 0,
        }
    }

    pub fn get(&self, loc: usize) -> u8 {
        debug!("cia2 get {:04X} ", loc);
        if loc == 0xDD00 {
            return self.port_a;
        }
        0
    }

    pub fn set(&mut self, loc: usize, value: u8) {
        if loc == 0xDD00 {
            self.port_a = value;
        }
        debug!("cia2 set {:04X} = {:02X}", loc, value);
    }

    pub fn get_vic_bank(&self) -> u16 {
        (!((self.port_a as u16) << 14)) & 0xC000
    }
}