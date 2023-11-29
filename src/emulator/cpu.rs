use std::cell::RefCell;
use std::rc::Rc;

use crate::emulator::logger::CpuLogger;

pub struct Cpu {
    pub pc: u16,
    pub sp: u8,
    pub p: u8,
    pub a: u8,
    pub x: u8,
    pub y: u8,
    cpu_logger: Rc<RefCell<CpuLogger>>,
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
    pub fn new(cpu_logger: Rc<RefCell<CpuLogger>>) -> Cpu {
        Cpu {
            pc: 0,
            sp: 0,
            p: 0x20,
            a: 0,
            x: 0,
            y: 0,
            cpu_logger,
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
        self.cpu_logger.borrow_mut().instruction("INC");
        let new_value = value.wrapping_add(1);
        self.set_negative_and_zero_flags(new_value);
        new_value
    }

    pub fn lda(&mut self, value: u8) {
        self.cpu_logger.borrow_mut().instruction("LDA");
        self.a = value;
        self.set_negative_and_zero_flags(self.a);
    }

    pub fn ldx(&mut self, value: u8) {
        self.cpu_logger.borrow_mut().instruction("LDX");
        self.x = value;
        self.set_negative_and_zero_flags(self.x);
    }

    pub fn and(&mut self, value: u8) {
        self.cpu_logger.borrow_mut().instruction("AND");
        self.a &= value;
        self.set_negative_and_zero_flags(self.a);
    }

    pub fn bmi(&mut self) -> bool {
        self.cpu_logger.borrow_mut().instruction("BMI");
        self.get_negative_flag()
    }

    pub fn bpl(&mut self) -> bool {
        self.cpu_logger.borrow_mut().instruction("BPL");
        !self.get_negative_flag()
    }

    pub fn ora(&mut self, value: u8) {
        self.cpu_logger.borrow_mut().instruction("ORA");
        self.a |= value;
        self.set_negative_and_zero_flags(self.a);
    }

    pub fn clc(&mut self) {
        self.cpu_logger.borrow_mut().instruction("CLC");
        self.set_carry_flag(false);
    }

    pub fn ldy(&mut self, value: u8) {
        self.cpu_logger.borrow_mut().instruction("LDY");
        self.y = value;
        self.set_negative_and_zero_flags(self.y);
    }

    pub fn adc(&mut self, value: u8) {
        self.cpu_logger.borrow_mut().instruction("ADC");
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

    pub fn sei(&mut self) {
        self.cpu_logger.borrow_mut().instruction("SEI");
        self.set_interrupt_flag(true);
    }

    pub fn cld(&mut self) {
        self.cpu_logger.borrow_mut().instruction("CLD");
        self.set_decimal_mode_flag(false);
    }

    pub fn rol(&mut self, value: u8) -> u8 {
        self.cpu_logger.borrow_mut().instruction("ROL");
        let (value, carry) = value.overflowing_shl(1);
        self.set_negative_and_zero_flags(value);
        self.set_carry_flag(carry);
        value
    }

    pub fn txs(&mut self) {
        self.cpu_logger.borrow_mut().instruction("TXS");
        self.sp = self.x;
    }

    pub fn tax(&mut self) {
        self.cpu_logger.borrow_mut().instruction("TAX");
        self.x = self.a;
        self.set_negative_and_zero_flags(self.x);
    }

    pub fn txa(&mut self) {
        self.cpu_logger.borrow_mut().instruction("TXA");
        self.a = self.x;
        self.set_negative_and_zero_flags(self.a);
    }

    pub fn tay(&mut self) {
        self.cpu_logger.borrow_mut().instruction("TAY");
        self.y = self.a;
        self.set_negative_and_zero_flags(self.y);
    }

    pub fn tya(&mut self) {
        self.cpu_logger.borrow_mut().instruction("TYA");
        self.a = self.y;
        self.set_negative_and_zero_flags(self.a);
    }

    pub fn cmp(&mut self, value: u8) {
        self.cpu_logger.borrow_mut().instruction("CMP");
        let (value, overflow) = self.a.overflowing_sub(value);
        self.set_negative_and_zero_flags(value);
        self.set_carry_flag(overflow);
    }

    pub fn cpx(&mut self, value: u8) {
        self.cpu_logger.borrow_mut().instruction("CPX");
        let (value, overflow) = self.x.overflowing_sub(value);
        self.set_negative_and_zero_flags(value);
        self.set_carry_flag(overflow);
    }

    pub fn bne(&mut self) -> bool {
        self.cpu_logger.borrow_mut().instruction("BNE");
        !self.get_zero_flag()
    }

    pub fn bcs(&mut self) -> bool {
        self.cpu_logger.borrow_mut().instruction("BCS");
        self.get_carry_flag()
    }

    pub fn bcc(&mut self) -> bool {
        self.cpu_logger.borrow_mut().instruction("BCC");
        !self.get_carry_flag()
    }

    pub fn beq(&mut self) -> bool {
        self.cpu_logger.borrow_mut().instruction("BEQ");
        self.get_zero_flag()
    }

    pub fn stx(&mut self) -> u8 {
        self.cpu_logger.borrow_mut().instruction("STX");
        self.x
    }

    pub fn sty(&mut self) -> u8 {
        self.cpu_logger.borrow_mut().instruction("STY");
        self.y
    }

    pub fn sta(&mut self) -> u8 {
        self.cpu_logger.borrow_mut().instruction("STA");
        self.a
    }

    pub fn dex(&mut self) {
        self.cpu_logger.borrow_mut().instruction("DEX");
        self.x = self.x.wrapping_sub(1);
        self.set_negative_and_zero_flags(self.x);
    }

    pub fn dey(&mut self) {
        self.cpu_logger.borrow_mut().instruction("DEY");
        self.y = self.y.wrapping_sub(1);
        self.set_negative_and_zero_flags(self.y);
    }

    pub fn iny(&mut self) {
        self.cpu_logger.borrow_mut().instruction("INY");
        self.y = self.y.wrapping_add(1);
        self.set_negative_and_zero_flags(self.y);
    }

    pub fn inx(&mut self) {
        self.cpu_logger.borrow_mut().instruction("INX");
        self.x = self.x.wrapping_add(1);
        self.set_negative_and_zero_flags(self.x);
    }
}