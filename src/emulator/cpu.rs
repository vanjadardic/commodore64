pub struct Cpu {
    pub pc: u16,
    pub sp: u8,
    pub p: u8,
    pub a: u8,
    pub x: u8,
    pub y: u8,
}

pub enum Flag {
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
            p: 0x20,
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

    pub fn set_pc(&mut self, pcl: u8, pch: u8) {
        self.pc = ((pcl as u16) & 0x00FF) | (((pch as u16) << 8) & 0xFF00);
    }

    pub fn set_negative_and_zero_flags(&mut self, value: u8) {
        self.p = (self.p & !0x80) | (value & 0x80);
        if value == 0 { self.p |= 0x02; } else { self.p &= !0x02; }
    }

    pub fn set_interrupt_flag(&mut self, value: bool) {
        if value { self.p |= 0x04; } else { self.p &= !0x04; }
    }

    pub fn set_decimal_mode_flag(&mut self, value: bool) {
        if value { self.p |= 0x08; } else { self.p &= !0x08; }
    }
    pub fn set_carry_flag(&mut self, value: bool) {
        if value { self.p |= 0x01; } else { self.p &= !0x01; }
    }

    pub fn get_zero_flag(&self) -> bool {
        (self.p & 0x02) > 0
    }

    pub fn get_negative_flag(&self) -> bool {
        (self.p & 0x80) > 0
    }

    // pub fn set_flag(&mut self, flag: Flag) {
    //     self.p |= match flag {
    //         Flag::N => 0x80,
    //         Flag::V => 0x40,
    //         Flag::B => 0x10,
    //         Flag::D => 0x08,
    //         Flag::I => 0x04,
    //         Flag::Z => 0x02,
    //         Flag::C => 0x01
    //     };
    // }
    //
    // pub fn clear_flag(&mut self, flag: Flag) {
    //     self.p &= !match flag {
    //         Flag::N => 0x80,
    //         Flag::V => 0x40,
    //         Flag::B => 0x10,
    //         Flag::D => 0x08,
    //         Flag::I => 0x04,
    //         Flag::Z => 0x02,
    //         Flag::C => 0x01
    //     };
    // }
    //
    // pub fn get_flag(&self, flag: Flag) -> bool {
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

    pub fn get_pch(&self) -> u8 {
        (self.pc >> 8) as u8
    }

    pub fn get_pcl(&self) -> u8 {
        self.pc as u8
    }
}