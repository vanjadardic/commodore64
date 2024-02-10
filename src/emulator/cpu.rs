use crate::emulator::addressing::Addressing;
use crate::emulator::logger::CpuLogger;
use crate::emulator::memory::Memory;

pub struct Cpu {
    pub pc: u16,
    pub sp: u8,
    pub p: u8,
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub state: u8,
    pub opcode: u8,
    // debug
    pub inst: &'static str,
    pub interrupted: bool,
    pub interrupted_started: bool,
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
            state: 1,
            opcode: 0,
            inst: "",
            interrupted: false,
            interrupted_started: false,
        }
    }

    pub fn tick(&mut self, cpu_logger: &mut CpuLogger, memory: &mut Memory, addressing: &mut Addressing) -> Result<(), String> {
        // if self.tick_count == 2085967 {
        //     println!();
        // }
        if self.state == 1 {
            cpu_logger.init(&self);
            if self.interrupted {
                self.state += 1;
                self.interrupted_started = true;
                return Ok(());
            }
            self.interrupted_started = false;
            let pc = self.get_and_increment_pc();
            if pc == 0xBC9B {
                //println!();
            }
            self.opcode = memory.get_from_word(pc);
            cpu_logger.opcode(self.opcode);
            self.state += 1;
            return Ok(());
        }
        if self.interrupted_started {
            self.state = addressing.implied_irq(self.state, self, memory)?;
        } else {
            self.state = match self.opcode {
                x @ 0x05 => addressing.zero_page_read(self.state, self, memory, Cpu::ora, x),
                x @ 0x06 => addressing.zero_page_read_modify_write(self.state, self, memory, Cpu::asl, x),
                x @ 0x08 => addressing.implied_php_pha(self.state, self, memory, Cpu::php, x),
                x @ 0x09 => addressing.immediate(self.state, self, memory, Cpu::ora, x),
                x @ 0x0A => addressing.accumulator(self.state, self, Cpu::asl, x),
                x @ 0x0D => addressing.absolute_read(self.state, self, memory, Cpu::ora, x),
                x @ 0x10 => addressing.relative(self.state, self, memory, Cpu::bpl, x),
                x @ 0x16 => addressing.zero_page_indexed_read_modify_write_x(self.state, self, memory, Cpu::asl, x),
                x @ 0x18 => addressing.implied(self.state, self, Cpu::clc, x),
                x @ 0x20 => addressing.absolute_jsr(self.state, self, memory, x),
                x @ 0x24 => addressing.zero_page_read(self.state, self, memory, Cpu::bit, x),
                x @ 0x26 => addressing.zero_page_read_modify_write(self.state, self, memory, Cpu::rol, x),
                x @ 0x28 => addressing.implied_plp_pla(self.state, self, memory, Cpu::plp, x),
                x @ 0x29 => addressing.immediate(self.state, self, memory, Cpu::and, x),
                x @ 0x2A => addressing.accumulator(self.state, self, Cpu::rol, x),
                x @ 0x2C => addressing.absolute_read(self.state, self, memory, Cpu::bit, x),
                x @ 0x2D => addressing.absolute_read(self.state, self, memory, Cpu::and, x),
                x @ 0x30 => addressing.relative(self.state, self, memory, Cpu::bmi, x),
                x @ 0x38 => addressing.implied(self.state, self, Cpu::sec, x),
                x @ 0x40 => addressing.implied_rti(self.state, self, memory, x),
                x @ 0x45 => addressing.zero_page_read(self.state, self, memory, Cpu::eor, x),
                x @ 0x46 => addressing.zero_page_read_modify_write(self.state, self, memory, Cpu::lsr, x),
                x @ 0x48 => addressing.implied_php_pha(self.state, self, memory, Cpu::pha, x),
                x @ 0x49 => addressing.immediate(self.state, self, memory, Cpu::eor, x),
                x @ 0x4A => addressing.accumulator(self.state, self, Cpu::lsr, x),
                x @ 0x4C => addressing.absolute_jmp(self.state, self, memory, x),
                x @ 0x56 => addressing.zero_page_indexed_read_modify_write_x(self.state, self, memory, Cpu::lsr, x),
                x @ 0x58 => addressing.implied(self.state, self, Cpu::cli, x),
                x @ 0x60 => addressing.implied_rts(self.state, self, memory, x),
                x @ 0x65 => addressing.zero_page_read(self.state, self, memory, Cpu::adc, x),
                x @ 0x66 => addressing.zero_page_read_modify_write(self.state, self, memory, Cpu::ror, x),
                x @ 0x68 => addressing.implied_plp_pla(self.state, self, memory, Cpu::pla, x),
                x @ 0x69 => addressing.immediate(self.state, self, memory, Cpu::adc, x),
                x @ 0x6A => addressing.accumulator(self.state, self, Cpu::ror, x),
                x @ 0x6C => addressing.absolute_indirect_jmp(self.state, self, memory, x),
                x @ 0x70 => addressing.relative(self.state, self, memory, Cpu::bvs, x),
                x @ 0x76 => addressing.zero_page_indexed_read_modify_write_x(self.state, self, memory, Cpu::ror, x),
                x @ 0x78 => addressing.implied(self.state, self, Cpu::sei, x),
                x @ 0x79 => addressing.absolute_indexed_read_y(self.state, self, memory, Cpu::adc, x),
                x @ 0x84 => addressing.zero_page_write(self.state, self, memory, Cpu::sty, x),
                x @ 0x85 => addressing.zero_page_write(self.state, self, memory, Cpu::sta, x),
                x @ 0x86 => addressing.zero_page_write(self.state, self, memory, Cpu::stx, x),
                x @ 0x88 => addressing.implied(self.state, self, Cpu::dey, x),
                x @ 0x8A => addressing.implied(self.state, self, Cpu::txa, x),
                x @ 0x8C => addressing.absolute_write(self.state, self, memory, Cpu::sty, x),
                x @ 0x8D => addressing.absolute_write(self.state, self, memory, Cpu::sta, x),
                x @ 0x8E => addressing.absolute_write(self.state, self, memory, Cpu::stx, x),
                x @ 0x90 => addressing.relative(self.state, self, memory, Cpu::bcc, x),
                x @ 0x91 => addressing.indirect_indexed_write(self.state, self, memory, Cpu::sta, x),
                x @ 0x94 => addressing.zero_page_indexed_write_x(self.state, self, memory, Cpu::sty, x),
                x @ 0x95 => addressing.zero_page_indexed_write_x(self.state, self, memory, Cpu::sta, x),
                x @ 0x98 => addressing.implied(self.state, self, Cpu::tya, x),
                x @ 0x99 => addressing.absolute_indexed_write_y(self.state, self, memory, Cpu::sta, x),
                x @ 0x9A => addressing.implied(self.state, self, Cpu::txs, x),
                x @ 0x9D => addressing.absolute_indexed_write_x(self.state, self, memory, Cpu::sta, x),
                x @ 0xA0 => addressing.immediate(self.state, self, memory, Cpu::ldy, x),
                x @ 0xA2 => addressing.immediate(self.state, self, memory, Cpu::ldx, x),
                x @ 0xA4 => addressing.zero_page_read(self.state, self, memory, Cpu::ldy, x),
                x @ 0xA5 => addressing.zero_page_read(self.state, self, memory, Cpu::lda, x),
                x @ 0xA6 => addressing.zero_page_read(self.state, self, memory, Cpu::ldx, x),
                x @ 0xA8 => addressing.implied(self.state, self, Cpu::tay, x),
                x @ 0xA9 => addressing.immediate(self.state, self, memory, Cpu::lda, x),
                x @ 0xAA => addressing.implied(self.state, self, Cpu::tax, x),
                x @ 0xAC => addressing.absolute_read(self.state, self, memory, Cpu::ldy, x),
                x @ 0xAD => addressing.absolute_read(self.state, self, memory, Cpu::lda, x),
                x @ 0xAE => addressing.absolute_read(self.state, self, memory, Cpu::ldx, x),
                x @ 0xB0 => addressing.relative(self.state, self, memory, Cpu::bcs, x),
                x @ 0xB1 => addressing.indirect_indexed_read(self.state, self, memory, Cpu::lda, x),
                x @ 0xB4 => addressing.zero_page_indexed_read_x(self.state, self, memory, Cpu::ldy, x),
                x @ 0xB5 => addressing.zero_page_indexed_read_x(self.state, self, memory, Cpu::lda, x),
                x @ 0xB9 => addressing.absolute_indexed_read_y(self.state, self, memory, Cpu::lda, x),
                x @ 0xBA => addressing.implied(self.state, self, Cpu::tsx, x),
                x @ 0xBD => addressing.absolute_indexed_read_x(self.state, self, memory, Cpu::lda, x),
                x @ 0xC0 => addressing.immediate(self.state, self, memory, Cpu::cpy, x),
                x @ 0xC4 => addressing.zero_page_read(self.state, self, memory, Cpu::cpy, x),
                x @ 0xC5 => addressing.zero_page_read(self.state, self, memory, Cpu::cmp, x),
                x @ 0xC6 => addressing.zero_page_read_modify_write(self.state, self, memory, Cpu::dec, x),
                x @ 0xC8 => addressing.implied(self.state, self, Cpu::iny, x),
                x @ 0xC9 => addressing.immediate(self.state, self, memory, Cpu::cmp, x),
                x @ 0xCA => addressing.implied(self.state, self, Cpu::dex, x),
                x @ 0xCD => addressing.absolute_read(self.state, self, memory, Cpu::cmp, x),
                x @ 0xCE => addressing.absolute_read_modify_write(self.state, self, memory, Cpu::dec, x),
                x @ 0xD0 => addressing.relative(self.state, self, memory, Cpu::bne, x),
                x @ 0xD1 => addressing.indirect_indexed_read(self.state, self, memory, Cpu::cmp, x),
                x @ 0xD8 => addressing.implied(self.state, self, Cpu::cld, x),
                x @ 0xDD => addressing.absolute_indexed_read_x(self.state, self, memory, Cpu::cmp, x),
                x @ 0xE0 => addressing.immediate(self.state, self, memory, Cpu::cpx, x),
                x @ 0xE4 => addressing.zero_page_read(self.state, self, memory, Cpu::cpx, x),
                x @ 0xE5 => addressing.zero_page_read(self.state, self, memory, Cpu::sbc, x),
                x @ 0xE6 => addressing.zero_page_read_modify_write(self.state, self, memory, Cpu::inc, x),
                x @ 0xE8 => addressing.implied(self.state, self, Cpu::inx, x),
                x @ 0xE9 => addressing.immediate(self.state, self, memory, Cpu::sbc, x),
                x @ 0xEA => addressing.implied(self.state, self, Cpu::nop, x),
                x @ 0xEC => addressing.absolute_read(self.state, self, memory, Cpu::cpx, x),
                x @ 0xEE => addressing.absolute_read_modify_write(self.state, self, memory, Cpu::inc, x),
                x @ 0xF0 => addressing.relative(self.state, self, memory, Cpu::beq, x),
                x @ 0xF1 => addressing.indirect_indexed_read(self.state, self, memory, Cpu::sbc, x),
                x @ 0xF9 => addressing.absolute_indexed_read_y(self.state, self, memory, Cpu::sbc, x),
                x => Err(format!("Illegal opcode {:02X} at {:04X}", x, self.pc - 1))
            }?;
        }

        if self.state == 1 {
            cpu_logger.log(&self, &addressing);
        }

        Ok(())
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

    pub fn get_interrupt_flag(&self) -> bool {
        (self.p & 0x04) > 0
    }

    pub fn get_negative_flag(&self) -> bool {
        (self.p & 0x80) > 0
    }

    pub fn get_overflow_flag(&self) -> bool {
        (self.p & 0x40) > 0
    }

    pub fn interrupt(&mut self) {
        if !self.get_interrupt_flag() {
            self.interrupted = true;
        }
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
        self.p = (self.p & !0xC0) | (value & 0xC0);
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

    pub fn tsx(&mut self) {
        self.inst = "TSX";
        self.x = self.sp;
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

    pub fn bvs(&mut self) -> bool {
        self.inst = "BVS";
        self.get_overflow_flag()
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

    pub fn nop(&mut self) {
        self.inst = "NOP";
        self.x = self.x.wrapping_add(1);
        self.set_negative_and_zero_flags(self.x);
    }
}