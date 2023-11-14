pub struct Cpu {
    pc: u16,
    sp: u8,
    p: u8,
    a: u8,
    x: u8,
    y: u8,
}

enum Flag {
    N,
    V,
    B,
    D,
    I,
    Z,
    C,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            pc: 0,
            sp: 0,
            p: 0,
            a: 0,
            x: 0,
            y: 0,
        }
    }

    fn is_flag_set(&self, flag: Flag) -> bool {
        self.p & match flag {
            Flag::N => 0x80,
            Flag::V => 0x40,
            Flag::B => 0x10,
            Flag::D => 0x08,
            Flag::I => 0x04,
            Flag::Z => 0x02,
            Flag::C => 0x01
        } > 0
    }
}