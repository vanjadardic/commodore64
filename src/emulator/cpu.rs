pub struct Cpu {
    pub pc: u16,
    pub sp: u8,
    pub p: u8,
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub inst: &'static str,
}

// pub enum Flag {
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
            p: 0x20,
            a: 0,
            x: 0,
            y: 0,
            inst: "",
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

    pub fn set_zero_flag(&mut self, value: bool) {
        if value { self.p |= 0x02; } else { self.p &= !0x02; }
    }

    pub fn set_interrupt_flag(&mut self, value: bool) {
        if value { self.p |= 0x04; } else { self.p &= !0x04; }
    }

    pub fn get_decimal_mode_flag(&mut self) -> bool {
        (self.p & 0x08) > 0
    }

    pub fn set_decimal_mode_flag(&mut self, value: bool) {
        if value { self.p |= 0x08; } else { self.p &= !0x08; }
    }

    pub fn set_carry_flag(&mut self, value: bool) {
        if value { self.p |= 0x01; } else { self.p &= !0x01; }
    }

    pub fn set_overflow_flag(&mut self, value: bool) {
        if value { self.p |= 0x40; } else { self.p &= !0x40; }
    }

    pub fn get_zero_flag(&self) -> bool {
        (self.p & 0x02) > 0
    }

    pub fn get_carry_flag(&self) -> bool {
        (self.p & 0x01) > 0
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

    pub fn set_pch(&mut self, value: u8) {
        self.pc = (self.pc & 0x00FF) | (((value as u16) << 8) & 0xFF00);
    }

    pub fn set_pcl(&mut self, value: u8) {
        self.pc = ((value as u16) & 0x00FF) | (self.pc & 0xFF00);
    }

    pub fn inc(&mut self, value: u8) -> u8 {
        self.inst = "INC";
        let new_value = value.wrapping_add(1);
        self.set_negative_and_zero_flags(new_value);
        new_value
    }

    pub fn dec(&mut self, value: u8) -> u8 {
        self.inst = "DEC";
        let new_value = value.wrapping_sub(1);
        self.set_negative_and_zero_flags(new_value);
        new_value
    }

    pub fn lda(&mut self, value: u8) {
        self.inst = "LDA";
        self.a = value;
        self.set_negative_and_zero_flags(self.a);
    }

    pub fn ldx(&mut self, value: u8) {
        self.inst = "LDX";
        self.x = value;
        self.set_negative_and_zero_flags(self.x);
    }

    pub fn and(&mut self, value: u8) {
        self.inst = "AND";
        self.a &= value;
        self.set_negative_and_zero_flags(self.a);
    }

    pub fn bmi(&mut self) -> bool {
        self.inst = "BMI";
        self.get_negative_flag()
    }

    pub fn bpl(&mut self) -> bool {
        self.inst = "BPL";
        !self.get_negative_flag()
    }

    pub fn ora(&mut self, value: u8) {
        self.inst = "ORA";
        self.a |= value;
        self.set_negative_and_zero_flags(self.a);
    }

    pub fn bit(&mut self, value: u8) {
        self.inst = "BIT";
        self.p |= (self.p & !0xC0) | (value & 0xC0);
        self.set_zero_flag((self.a & value) == 0);
    }

    pub fn clc(&mut self) {
        self.inst = "CLC";
        self.set_carry_flag(false);
    }

    pub fn sec(&mut self) {
        self.inst = "SEC";
        self.set_carry_flag(true);
    }

    pub fn ldy(&mut self, value: u8) {
        self.inst = "LDY";
        self.y = value;
        self.set_negative_and_zero_flags(self.y);
    }

    pub fn eor(&mut self, value: u8) {
        self.inst = "EOR";
        self.a ^= value;
        self.set_negative_and_zero_flags(value);
    }

    pub fn adc(&mut self, value: u8) {
        self.inst = "ADC";
        self._adc(value);
    }

    fn _adc(&mut self, value: u8) {
        if self.get_decimal_mode_flag() {
            let a = (self.a & 0x0F) % 10 + (((self.a >> 4) & 0x0F) % 10) * 10;
            let value = (value & 0x0F) % 10 + (((value >> 4) & 0x0F) % 10) * 10;
            let newa = a + value + (self.get_carry_flag() as u8);
            self.set_carry_flag(newa > 99);
            let newa = newa % 100;
            self.a = (newa % 10) | ((newa / 10) << 4);
            self.set_negative_and_zero_flags(self.a);
            //todo overflow flag
        } else {
            let (newa, carry) = self.a.overflowing_add(value);
            let (newa, carry2) = newa.overflowing_add(self.get_carry_flag() as u8);
            self.set_overflow_flag(
                ((newa & 0x80) != (value & 0x80)) && ((newa & 0x80) != (self.a & 0x80))
            );
            self.a = newa;
            self.set_negative_and_zero_flags(self.a);
            self.set_carry_flag(carry || carry2);
        }
    }

    pub fn sbc(&mut self, value: u8) {
        self.inst = "SBC";
        self._adc(!value);
    }

    pub fn pha(&mut self) -> u8 {
        self.inst = "PHA";
        self.a
    }

    pub fn php(&mut self) -> u8 {
        self.inst = "PHP";
        self.p
    }

    pub fn pla(&mut self, value: u8) {
        self.inst = "PLA";
        self.a = value;
        self.set_negative_and_zero_flags(value);
    }

    pub fn plp(&mut self, value: u8) {
        self.inst = "PLP";
        self.p = value;
    }

    pub fn sei(&mut self) {
        self.inst = "SEI";
        self.set_interrupt_flag(true);
    }

    pub fn cli(&mut self) {
        self.inst = "CLI";
        self.set_interrupt_flag(false);
    }

    pub fn cld(&mut self) {
        self.inst = "CLD";
        self.set_decimal_mode_flag(false);
    }

    pub fn rol(&mut self, value: u8) -> u8 {
        self.inst = "ROL";
        let new_carry = value & 0x80 > 0;
        let mut value = value.wrapping_shl(1);
        if self.get_carry_flag() {
            value |= 0x01;
        }
        self.set_negative_and_zero_flags(value);
        self.set_carry_flag(new_carry);
        value
    }

    pub fn ror(&mut self, value: u8) -> u8 {
        self.inst = "ROR";
        let new_carry = value & 0x01 > 0;
        let mut value = value.wrapping_shr(1);
        if self.get_carry_flag() {
            value |= 0x80;
        }
        self.set_negative_and_zero_flags(value);
        self.set_carry_flag(new_carry);
        value
    }

    pub fn lsr(&mut self, value: u8) -> u8 {
        self.inst = "LSR";
        self.set_carry_flag(value & 0x01 > 0);
        let value = value.wrapping_shr(1);
        self.set_negative_and_zero_flags(value);
        value
    }

    pub fn asl(&mut self, value: u8) -> u8 {
        self.inst = "ASL";
        self.set_carry_flag(value & 0x80 > 0);
        let value = value.wrapping_shl(1);
        self.set_negative_and_zero_flags(value);
        value
    }

    pub fn txs(&mut self) {
        self.inst = "TXS";
        self.sp = self.x;
    }

    pub fn tax(&mut self) {
        self.inst = "TAX";
        self.x = self.a;
        self.set_negative_and_zero_flags(self.x);
    }

    pub fn txa(&mut self) {
        self.inst = "TXA";
        self.a = self.x;
        self.set_negative_and_zero_flags(self.a);
    }

    pub fn tay(&mut self) {
        self.inst = "TAY";
        self.y = self.a;
        self.set_negative_and_zero_flags(self.y);
    }

    pub fn tya(&mut self) {
        self.inst = "TYA";
        self.a = self.y;
        self.set_negative_and_zero_flags(self.a);
    }

    pub fn cmp(&mut self, value: u8) {
        self.inst = "CMP";
        let (value, overflow) = self.a.overflowing_sub(value);
        self.set_negative_and_zero_flags(value);
        self.set_carry_flag(!overflow);
    }

    pub fn cpy(&mut self, value: u8) {
        self.inst = "CPY";
        let (value, overflow) = self.y.overflowing_sub(value);
        self.set_negative_and_zero_flags(value);
        self.set_carry_flag(!overflow);
    }

    pub fn cpx(&mut self, value: u8) {
        self.inst = "CPX";
        let (value, overflow) = self.x.overflowing_sub(value);
        self.set_negative_and_zero_flags(value);
        self.set_carry_flag(!overflow);
    }

    pub fn bne(&mut self) -> bool {
        self.inst = "BNE";
        !self.get_zero_flag()
    }

    pub fn bcs(&mut self) -> bool {
        self.inst = "BCS";
        self.get_carry_flag()
    }

    pub fn bcc(&mut self) -> bool {
        self.inst = "BCC";
        !self.get_carry_flag()
    }

    pub fn beq(&mut self) -> bool {
        self.inst = "BEQ";
        self.get_zero_flag()
    }

    pub fn stx(&mut self) -> u8 {
        self.inst = "STX";
        self.x
    }

    pub fn sty(&mut self) -> u8 {
        self.inst = "STY";
        self.y
    }

    pub fn sta(&mut self) -> u8 {
        self.inst = "STA";
        self.a
    }

    pub fn dex(&mut self) {
        self.inst = "DEX";
        self.x = self.x.wrapping_sub(1);
        self.set_negative_and_zero_flags(self.x);
    }

    pub fn dey(&mut self) {
        self.inst = "DEY";
        self.y = self.y.wrapping_sub(1);
        self.set_negative_and_zero_flags(self.y);
    }

    pub fn iny(&mut self) {
        self.inst = "INY";
        self.y = self.y.wrapping_add(1);
        self.set_negative_and_zero_flags(self.y);
    }

    pub fn inx(&mut self) {
        self.inst = "INX";
        self.x = self.x.wrapping_add(1);
        self.set_negative_and_zero_flags(self.x);
    }
}