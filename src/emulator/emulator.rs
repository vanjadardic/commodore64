use std::time::Duration;

use crate::emulator::cpu::Cpu;
use crate::emulator::cpu_logger::CpuLogger;
use crate::emulator::memory::Memory;

// const MASTER_CLOCK_PAL: u128 = 17_734_475;
// const MASTER_CLOCK_NTSC: u128 = 14_318_180;

const CLOCK_PAL: u128 = 985_248;
// const CLOCK_NTSC: u128 = 1_022_727;

// const CLOCK_VICII_PAL: u128 = CLOCK_PAL * 8;
// const CLOCK_VICII_NTSC: u128 = CLOCK_NTSC * 8;

const NANOS_PER_SEC: u128 = 1_000_000_000;
const CLOCK: u128 = CLOCK_PAL;

pub struct Emulator {
    tick_count: u64,
    memory: Memory,
    cpu: Cpu,
    /// tick counter within an instruction
    sub_tick: u8,
    /// currently executing instruction
    opcode: u8,
    low: u8,
    high: u8,
    fix_high: bool,
    cpu_logger: CpuLogger,
    latch: u8,
}

impl Emulator {
    pub fn new() -> Emulator {
        let memory = Memory::new();
        let mut cpu = Cpu::new();
        let low = memory.get_from_word(0xFFFC);
        let high = memory.get_from_word(0xFFFD);
        cpu.pc = ((low as u16) & 0x00FF) | (((high as u16) << 8) & 0xFF00);
        Emulator {
            tick_count: 0,
            memory,
            cpu,
            sub_tick: 1,
            opcode: 0,
            low: 0,
            high: 0,
            fix_high: false,
            cpu_logger: CpuLogger::new(),
            latch: 0,
        }
    }

    pub fn step(&mut self, elapsed: Duration) -> Result<(), String> {
        let want_ticks = ((elapsed.as_nanos() * CLOCK) / NANOS_PER_SEC) as u64;
        while self.tick_count < want_ticks {
            self.tick()?;
            self.tick_count += 1;
        }
        Ok(())
    }

    fn tick(&mut self) -> Result<(), String> {
        if self.sub_tick == 1 {
            let pc = self.cpu.get_and_increment_pc();
            self.cpu_logger.pc(pc);
            self.opcode = self.memory.get_from_word(pc);
            self.cpu_logger.opcode(self.opcode);
            self.sub_tick += 1;
            return Ok(());
        }
        match self.opcode {
            x @ 0x09 => self.immediate_addressing(Emulator::ora, x),
            x @ 0x20 => {
                if self.sub_tick == 2 {
                    self.low = self.memory.get_from_word(self.cpu.get_and_increment_pc());
                    self.cpu_logger.op(self.low);
                    self.sub_tick += 1;
                    return Ok(());
                }
                if self.sub_tick == 3 {
                    self.sub_tick += 1;
                    return Ok(());
                }
                if self.sub_tick == 4 {
                    self.stack_push(self.cpu.get_pch());
                    self.sub_tick += 1;
                    return Ok(());
                }
                if self.sub_tick == 5 {
                    self.stack_push(self.cpu.get_pcl());
                    self.sub_tick += 1;
                    return Ok(());
                }
                if self.sub_tick == 6 {
                    self.high = self.memory.get_from_word(self.cpu.pc);
                    self.cpu_logger.op(self.high);
                    self.cpu.set_pc(self.low, self.high);
                    self.cpu_logger.inst("JSR");
                    self.sub_tick = 1;
                    return Ok(());
                }
                Err(format!("Illegal sub_tick {} for opcode {:02X}", self.sub_tick, x))
            }
            x @ 0x29 => self.immediate_addressing(Emulator::and, x),
            x @ 0x30 => self.relative_addressing(Emulator::bmi, x),
            x @ 0x4C => {
                if self.sub_tick == 2 {
                    self.low = self.memory.get_from_word(self.cpu.get_and_increment_pc());
                    self.cpu_logger.op(self.low);
                    self.sub_tick += 1;
                    return Ok(());
                }
                if self.sub_tick == 3 {
                    self.high = self.memory.get_from_word(self.cpu.pc);
                    self.cpu_logger.op(self.low);
                    self.cpu.set_pc(self.low, self.high);
                    self.sub_tick = 1;
                    return Ok(());
                }
                Err(format!("Illegal sub_tick {} for opcode {:02X}", self.sub_tick, x))
            }
            x @ 0x60 => {
                if self.sub_tick == 2 {
                    self.sub_tick += 1;
                    return Ok(());
                }
                if self.sub_tick == 3 {
                    self.cpu.sp += 1;
                    self.sub_tick += 1;
                    return Ok(());
                }
                if self.sub_tick == 4 {
                    self.cpu.set_pc(self.memory.get_stack(self.cpu.sp), self.cpu.get_pch());
                    self.cpu.sp += 1;
                    self.sub_tick += 1;
                    return Ok(());
                }
                if self.sub_tick == 5 {
                    self.cpu.set_pc(self.cpu.get_pcl(), self.memory.get_stack(self.cpu.sp));
                    self.sub_tick += 1;
                    return Ok(());
                }
                if self.sub_tick == 6 {
                    self.cpu.get_and_increment_pc();
                    self.cpu_logger.inst("RTS");
                    self.sub_tick = 1;
                    return Ok(());
                }
                Err(format!("Illegal sub_tick {} for opcode {:02X}", self.sub_tick, x))
            }
            x @ 0x6C => {
                if self.sub_tick == 2 {
                    self.low = self.memory.get_from_word(self.cpu.get_and_increment_pc());
                    self.cpu_logger.op(self.low);
                    self.sub_tick += 1;
                    return Ok(());
                }
                if self.sub_tick == 3 {
                    self.high = self.memory.get_from_word(self.cpu.get_and_increment_pc());
                    self.cpu_logger.op(self.high);
                    self.sub_tick += 1;
                    return Ok(());
                }
                if self.sub_tick == 4 {
                    self.latch = self.memory.get_from_low_high(self.low, self.high);
                    self.sub_tick += 1;
                    return Ok(());
                }
                if self.sub_tick == 5 {
                    self.cpu.set_pc(self.latch, self.memory.get_from_low_high(self.low.wrapping_add(1), self.high));
                    self.cpu_logger.inst("JMP");
                    self.sub_tick = 1;
                    return Ok(());
                }
                Err(format!("Illegal sub_tick {} for opcode {:02X}", self.sub_tick, x))
            }
            x @ 0x78 => self.implied_addressing(Emulator::sei, x),
            x @ 0x85 => self.zero_page_addressing_write(Emulator::sta, x),
            x @ 0x8D => self.absolute_addressing_write(Emulator::sta, x),
            x @ 0x8E => self.absolute_addressing_write(Emulator::stx, x),
            x @ 0x99 => self.absolute_indexed_addressing_write_y(Emulator::sta, x),
            x @ 0x9A => self.implied_addressing(Emulator::txs, x),
            x @ 0xA2 => self.immediate_addressing(Emulator::ldx, x),
            x @ 0xA8 => self.implied_addressing(Emulator::tay, x),
            x @ 0xA9 => self.immediate_addressing(Emulator::lda, x),
            x @ 0xAA => self.implied_addressing(Emulator::tax, x),
            x @ 0xAD => self.absolute_addressing(Emulator::lda, x),
            x @ 0xAE => self.absolute_addressing(Emulator::ldx, x),
            //x @ 0xAC => self.absolute_addressing(Emulator::ldy, x),
            x @ 0xBD => self.absolute_indexed_addressing_x(Emulator::lda, x),
            x @ 0xC8 => self.implied_addressing(Emulator::iny, x),
            x @ 0xCA => self.implied_addressing(Emulator::dex, x),
            x @ 0xCD => self.absolute_addressing(Emulator::cmp, x),
            x @ 0xD8 => self.implied_addressing(Emulator::cld, x),
            x @ 0xD0 => self.relative_addressing(Emulator::bne, x),
            x @ 0xDD => self.absolute_indexed_addressing_x(Emulator::cmp, x),
            x @ 0xE8 => self.implied_addressing(Emulator::inx, x),
            x @ 0xEC => self.absolute_addressing(Emulator::cpx, x),
            x @ 0xF0 => self.relative_addressing(Emulator::beq, x),
            x => Err(format!("Illegal opcode {:02X} at {:04X}", x, self.cpu.pc - 1))
        }
    }

    fn absolute_addressing_write(&mut self, op: fn(&mut Emulator, u8, u8), opcode: u8) -> Result<(), String> {
        if self.sub_tick == 2 {
            self.low = self.memory.get_from_word(self.cpu.get_and_increment_pc());
            self.cpu_logger.op(self.low);
            self.sub_tick += 1;
            return Ok(());
        }
        if self.sub_tick == 3 {
            self.high = self.memory.get_from_word(self.cpu.get_and_increment_pc());
            self.cpu_logger.op(self.high);
            self.sub_tick += 1;
            return Ok(());
        }
        if self.sub_tick == 4 {
            op(self, self.low, self.high);
            self.sub_tick = 1;
            return Ok(());
        }
        Err(format!("Illegal sub_tick {} for opcode {:02X}", self.sub_tick, opcode))
    }

    fn absolute_addressing(&mut self, op: fn(&mut Emulator, u8), opcode: u8) -> Result<(), String> {
        if self.sub_tick == 2 {
            self.low = self.memory.get_from_word(self.cpu.get_and_increment_pc());
            self.cpu_logger.op(self.low);
            self.sub_tick += 1;
            return Ok(());
        }
        if self.sub_tick == 3 {
            self.high = self.memory.get_from_word(self.cpu.get_and_increment_pc());
            self.cpu_logger.op(self.high);
            self.sub_tick += 1;
            return Ok(());
        }
        if self.sub_tick == 4 {
            let value = self.memory.get_from_low_high(self.low, self.high);
            op(self, value);
            self.sub_tick = 1;
            return Ok(());
        }
        Err(format!("Illegal sub_tick {} for opcode {:02X}", self.sub_tick, opcode))
    }

    fn immediate_addressing(&mut self, op: fn(&mut Emulator, u8), opcode: u8) -> Result<(), String> {
        if self.sub_tick == 2 {
            let value = self.memory.get_from_word(self.cpu.get_and_increment_pc());
            self.cpu_logger.op(value);
            op(self, value);
            self.sub_tick = 1;
            return Ok(());
        }
        Err(format!("Illegal sub_tick {} for opcode {:02X}", self.sub_tick, opcode))
    }

    fn implied_addressing(&mut self, op: fn(&mut Emulator), opcode: u8) -> Result<(), String> {
        if self.sub_tick == 2 {
            op(self);
            self.sub_tick = 1;
            return Ok(());
        }
        Err(format!("Illegal sub_tick {} for opcode {:02X}", self.sub_tick, opcode))
    }

    fn absolute_indexed_addressing(&mut self, op: fn(&mut Emulator, u8), index: u8, opcode: u8) -> Result<(), String> {
        if self.sub_tick == 2 {
            self.low = self.memory.get_from_word(self.cpu.get_and_increment_pc());
            self.cpu_logger.op(self.low);
            self.sub_tick += 1;
            return Ok(());
        }
        if self.sub_tick == 3 {
            (self.low, self.fix_high) = self.low.overflowing_add(index);
            self.high = self.memory.get_from_word(self.cpu.get_and_increment_pc());
            self.cpu_logger.op(self.high);
            self.sub_tick += 1;
            return Ok(());
        }
        if self.sub_tick == 4 {
            let value = self.memory.get_from_low_high(self.low, self.high);
            op(self, value);
            if self.fix_high {
                self.high += 1;
                self.sub_tick += 1;
            } else {
                self.sub_tick = 1;
            }
            return Ok(());
        }
        if self.sub_tick == 5 {
            let value = self.memory.get_from_low_high(self.low, self.high);
            op(self, value);
            self.sub_tick = 1;
            return Ok(());
        }
        Err(format!("Illegal sub_tick {} for opcode {:02X}", self.sub_tick, opcode))
    }

    fn absolute_indexed_addressing_x(&mut self, op: fn(&mut Emulator, u8), opcode: u8) -> Result<(), String> {
        self.absolute_indexed_addressing(op, self.cpu.x, opcode)
    }

    fn absolute_indexed_addressing_y(&mut self, op: fn(&mut Emulator, u8), opcode: u8) -> Result<(), String> {
        self.absolute_indexed_addressing(op, self.cpu.y, opcode)
    }

    fn absolute_indexed_addressing_write(&mut self, op: fn(&mut Emulator, u8, u8), index: u8, opcode: u8) -> Result<(), String> {
        if self.sub_tick == 2 {
            self.low = self.memory.get_from_word(self.cpu.get_and_increment_pc());
            self.cpu_logger.op(self.low);
            self.sub_tick += 1;
            return Ok(());
        }
        if self.sub_tick == 3 {
            (self.low, self.fix_high) = self.low.overflowing_add(index);
            self.high = self.memory.get_from_word(self.cpu.get_and_increment_pc());
            self.cpu_logger.op(self.high);
            self.sub_tick += 1;
            return Ok(());
        }
        if self.sub_tick == 4 {
            if self.fix_high {
                self.high += 1;
            }
            self.sub_tick += 1;
            return Ok(());
        }
        if self.sub_tick == 5 {
            op(self, self.low, self.high);
            self.sub_tick = 1;
            return Ok(());
        }
        Err(format!("Illegal sub_tick {} for opcode {:02X}", self.sub_tick, opcode))
    }

    fn absolute_indexed_addressing_write_y(&mut self, op: fn(&mut Emulator, u8, u8), opcode: u8) -> Result<(), String> {
        self.absolute_indexed_addressing_write(op, self.cpu.y, opcode)
    }

    fn relative_addressing(&mut self, op: fn(&mut Emulator) -> bool, opcode: u8) -> Result<(), String> {
        if self.sub_tick == 2 {
            self.low = self.memory.get_from_word(self.cpu.get_and_increment_pc());
            self.cpu_logger.op(self.low);
            if op(self) {
                self.sub_tick += 1;
            } else {
                self.sub_tick = 1;
            }
            return Ok(());
        }
        if self.sub_tick == 3 {
            (self.low, self.fix_high) = self.low.overflowing_add(self.cpu.get_pcl());
            if self.fix_high {
                self.high = self.cpu.get_pch() + 1;
                self.sub_tick += 1;
            } else {
                self.cpu.set_pc(self.low, self.cpu.get_pch());
                self.sub_tick = 1;
            }
            return Ok(());
        }
        if self.sub_tick == 4 {
            self.cpu.set_pc(self.low, self.high);
            self.sub_tick = 1;
            return Ok(());
        }
        Err(format!("Illegal sub_tick {} for opcode {:02X}", self.sub_tick, opcode))
    }

    fn zero_page_addressing_write(&mut self, op: fn(&mut Emulator, u8, u8), opcode: u8) -> Result<(), String> {
        if self.sub_tick == 2 {
            self.low = self.memory.get_from_word(self.cpu.get_and_increment_pc());
            self.cpu_logger.op(self.low);
            self.sub_tick += 1;
            return Ok(());
        }
        if self.sub_tick == 3 {
            op(self, self.low, 0);
            self.sub_tick = 1;
            return Ok(());
        }
        Err(format!("Illegal sub_tick {} for opcode {:02X}", self.sub_tick, opcode))
    }

    fn lda(&mut self, value: u8) {
        self.cpu_logger.inst("LDA");
        self.cpu.a = value;
        self.cpu.set_negative_and_zero_flags(self.cpu.a);
    }

    fn ldx(&mut self, value: u8) {
        self.cpu_logger.inst("LDX");
        self.cpu.x = value;
        self.cpu.set_negative_and_zero_flags(self.cpu.x);
    }

    fn and(&mut self, value: u8) {
        self.cpu_logger.inst("AND");
        self.cpu.a &= value;
        self.cpu.set_negative_and_zero_flags(self.cpu.a);
    }

    fn bmi(&mut self) -> bool {
        self.cpu_logger.inst("BMI");
        self.cpu.get_negative_flag()
    }

    fn ora(&mut self, value: u8) {
        self.cpu_logger.inst("ORA");
        self.cpu.a |= value;
        self.cpu.set_negative_and_zero_flags(self.cpu.a);
    }

    fn ldy(&mut self, value: u8) {
        self.cpu_logger.inst("LDY");
        self.cpu.y = value;
        self.cpu.set_negative_and_zero_flags(self.cpu.y);
    }

    fn sei(&mut self) {
        self.cpu_logger.inst("SEI");
        self.cpu.set_interrupt_flag(true);
    }

    fn cld(&mut self) {
        self.cpu_logger.inst("CLD");
        self.cpu.set_decimal_mode_flag(false);
    }

    fn txs(&mut self) {
        self.cpu_logger.inst("TXS");
        self.cpu.sp = self.cpu.x;
    }

    fn tax(&mut self) {
        self.cpu_logger.inst("TAX");
        self.cpu.x = self.cpu.a;
        self.cpu.set_negative_and_zero_flags(self.cpu.x);
    }

    fn tay(&mut self) {
        self.cpu_logger.inst("TAY");
        self.cpu.y = self.cpu.a;
        self.cpu.set_negative_and_zero_flags(self.cpu.y);
    }

    fn stack_push(&mut self, value: u8) {
        self.memory.set_stack(self.cpu.sp, value);
        self.cpu.sp -= 1;
    }

    fn cmp(&mut self, value: u8) {
        self.cpu_logger.inst("CMP");
        let (value, overflow) = self.cpu.a.overflowing_sub(value);
        self.cpu.set_negative_and_zero_flags(value);
        self.cpu.set_carry_flag(overflow);
    }

    fn cpx(&mut self, value: u8) {
        self.cpu_logger.inst("CPX");
        let (value, overflow) = self.cpu.x.overflowing_sub(value);
        self.cpu.set_negative_and_zero_flags(value);
        self.cpu.set_carry_flag(overflow);
    }

    fn bne(&mut self) -> bool {
        self.cpu_logger.inst("BNE");
        !self.cpu.get_zero_flag()
    }

    fn beq(&mut self) -> bool {
        self.cpu_logger.inst("BEQ");
        self.cpu.get_zero_flag()
    }

    fn stx(&mut self, low: u8, high: u8) {
        self.cpu_logger.inst("STX");
        self.memory.set_from_low_high(low, high, self.cpu.x);
    }

    fn sta(&mut self, low: u8, high: u8) {
        self.cpu_logger.inst("STA");
        self.memory.set_from_low_high(low, high, self.cpu.a);
    }

    fn dex(&mut self) {
        self.cpu_logger.inst("DEX");
        self.cpu.x = self.cpu.x.wrapping_sub(1);
        self.cpu.set_negative_and_zero_flags(self.cpu.x);
    }

    fn iny(&mut self) {
        self.cpu_logger.inst("INY");
        self.cpu.y = self.cpu.y.wrapping_add(1);
        self.cpu.set_negative_and_zero_flags(self.cpu.y);
    }

    fn inx(&mut self) {
        self.cpu_logger.inst("INX");
        self.cpu.x = self.cpu.x.wrapping_add(1);
        self.cpu.set_negative_and_zero_flags(self.cpu.x);
    }
}