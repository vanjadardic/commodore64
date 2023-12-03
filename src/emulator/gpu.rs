use log::debug;

pub struct Gpu {}

impl Gpu {
    pub fn new() -> Gpu {
        Gpu {}
    }

    pub fn get(&self, loc: usize) -> u8 {
        debug!("gpu get {:04X} ", loc);
        0
    }

    pub fn set(&self, loc: usize, value: u8) {
        debug!("gpu set {:04X} = {:02X}", loc,value);
    }
}