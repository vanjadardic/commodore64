pub struct Cpu {
    pub pc: u16,
    sp: u8,
    p: u8,
    pub a: u8,
    pub x: u8,
    pub y: u8,
}

// enum Flag {
//     N,
//     V,
//     B,
//     D,
//     I,
//     Z,
//     C,
// }

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

    pub fn get_and_increment_pc(&mut self) -> u16 {
        let pc = self.pc;
        self.pc += 1;
        pc
    }

    pub fn set_pc(&mut self, low_address_byte: u8, high_address_byte: u8) {
        self.pc = ((low_address_byte as u16) & 0x00FF) | (((high_address_byte as u16) << 8) & 0xFF00);
    }

    // fn is_flag_set(&self, flag: Flag) -> bool {
    //     self.p & match flag {
    //         Flag::N => 0x80,
    //         Flag::V => 0x40,
    //         Flag::B => 0x10,
    //         Flag::D => 0x08,
    //         Flag::I => 0x04,
    //         Flag::Z => 0x02,
    //         Flag::C => 0x01
    //     } > 0
    // }
}