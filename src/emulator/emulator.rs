use std::time::Duration;

use crate::emulator::cpu::{Cpu, Flag};
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
}

impl Emulator {
    pub fn new() -> Emulator {
        let memory = Memory::new();
        let mut cpu = Cpu::new();
        let low = memory.get_from_pc(0xFFFC);
        let high = memory.get_from_pc(0xFFFD);
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
            self.opcode = self.memory.get_from_pc(self.cpu.get_and_increment_pc());
            self.sub_tick += 1;
            return Ok(());
        }
        match self.opcode {
            x @ 0x20 => { // JSR
                if self.sub_tick == 2 {
                    self.low = self.memory.get_from_pc(self.cpu.get_and_increment_pc());
                    self.sub_tick += 1;
                    return Ok(());
                }
                if self.sub_tick == 3 {
                    // internal operation (predecrement S?)
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
                    self.high = self.memory.get_from_pc(self.cpu.pc);
                    self.cpu.set_pc(self.low, self.high);
                    self.sub_tick = 1;
                    return Ok(());
                }
                Err(format!("Illegal sub_tick {} for opcode {:#04x}", self.sub_tick, x))
            }
            x @ 0x4C => { // JMP
                if self.sub_tick == 2 {
                    self.low = self.memory.get_from_pc(self.cpu.get_and_increment_pc());
                    self.sub_tick += 1;
                    return Ok(());
                }
                if self.sub_tick == 3 {
                    self.high = self.memory.get_from_pc(self.cpu.pc);
                    self.cpu.set_pc(self.low, self.high);
                    self.sub_tick = 1;
                    return Ok(());
                }
                Err(format!("Illegal sub_tick {} for opcode {:#04x}", self.sub_tick, x))
            }
            x @ 0x78 => self.implied_addressing(Emulator::sei, x),
            x @ 0x9A => self.implied_addressing(Emulator::txs, x),
            x @ 0xAD => self.absolute_addressing(Emulator::lda, x),
            x @ 0xA2 => self.immediate_addressing(Emulator::ldx, x),
            x @ 0xAE => self.absolute_addressing(Emulator::ldx, x),
            x @ 0xAC => self.absolute_addressing(Emulator::ldy, x),
            x @ 0xBD => self.absolute_indexed_addressing_x(Emulator::lda, x),
            x @ 0xD8 => self.implied_addressing(Emulator::cld, x),
            x @ 0xD0 => self.relative_addressing(Emulator::bne, x),
            x @ 0xDD => self.absolute_indexed_addressing_x(Emulator::cmp, x),
            x => Err(format!("Illegal opcode {:#04x}", x))
        }
    }

    fn absolute_addressing(&mut self, op: fn(&mut Emulator, u8), opcode: u8) -> Result<(), String> {
        if self.sub_tick == 2 {
            self.low = self.memory.get_from_pc(self.cpu.get_and_increment_pc());
            self.sub_tick += 1;
            return Ok(());
        }
        if self.sub_tick == 3 {
            self.high = self.memory.get_from_pc(self.cpu.get_and_increment_pc());
            self.sub_tick += 1;
            return Ok(());
        }
        if self.sub_tick == 4 {
            let value = self.memory.get_from_low_high(self.low, self.high);
            op(self, value);
            self.sub_tick = 1;
            return Ok(());
        }
        Err(format!("Illegal sub_tick {} for opcode {:#04x}", self.sub_tick, opcode))
    }

    fn immediate_addressing(&mut self, op: fn(&mut Emulator, u8), opcode: u8) -> Result<(), String> {
        if self.sub_tick == 2 {
            let value = self.memory.get_from_pc(self.cpu.get_and_increment_pc());
            op(self, value);
            self.sub_tick = 1;
            return Ok(());
        }
        Err(format!("Illegal sub_tick {} for opcode {:#04x}", self.sub_tick, opcode))
    }

    fn implied_addressing(&mut self, op: fn(&mut Emulator), opcode: u8) -> Result<(), String> {
        if self.sub_tick == 2 {
            op(self);
            self.sub_tick = 1;
            return Ok(());
        }
        Err(format!("Illegal sub_tick {} for opcode {:#04x}", self.sub_tick, opcode))
    }

    fn absolute_indexed_addressing(&mut self, op: fn(&mut Emulator, u8), index: u8, opcode: u8) -> Result<(), String> {
        if self.sub_tick == 2 {
            self.low = self.memory.get_from_pc(self.cpu.get_and_increment_pc());
            self.sub_tick += 1;
            return Ok(());
        }
        if self.sub_tick == 3 {
            (self.low, self.fix_high) = self.low.overflowing_add(index);
            self.high = self.memory.get_from_pc(self.cpu.get_and_increment_pc());
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
        Err(format!("Illegal sub_tick {} for opcode {:#04x}", self.sub_tick, opcode))
    }

    fn absolute_indexed_addressing_x(&mut self, op: fn(&mut Emulator, u8), opcode: u8) -> Result<(), String> {
        self.absolute_indexed_addressing(op, self.cpu.x, opcode)
    }

    fn relative_addressing(&mut self, op: fn(&mut Emulator) -> bool, opcode: u8) -> Result<(), String> {
        if self.sub_tick == 2 {
            self.low = self.memory.get_from_pc(self.cpu.get_and_increment_pc());
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
        Err(format!("Illegal sub_tick {} for opcode {:#04x}", self.sub_tick, opcode))
    }

    fn lda(&mut self, value: u8) {
        self.cpu.a = value;
    }

    fn ldx(&mut self, value: u8) {
        self.cpu.x = value;
    }

    fn ldy(&mut self, value: u8) {
        self.cpu.y = value;
    }

    fn sei(&mut self) {
        self.cpu.set_flag(Flag::I);
    }

    fn cld(&mut self) {
        self.cpu.reset_flag(Flag::D);
    }

    fn txs(&mut self) {
        self.cpu.sp = self.cpu.x;
    }

    fn stack_push(&mut self, value: u8) {
        self.memory.set_stack(self.cpu.sp, value);
        self.cpu.sp -= 1;
    }

    fn cmp(&mut self, value: u8) {
        if self.cpu.a < value { self.cpu.set_flag(Flag::N); } else { self.cpu.reset_flag(Flag::N); }
        if self.cpu.a == value { self.cpu.set_flag(Flag::Z); } else { self.cpu.reset_flag(Flag::Z); }
        if self.cpu.a >= value { self.cpu.set_flag(Flag::C); } else { self.cpu.reset_flag(Flag::C); }
    }

    fn bne(&mut self) -> bool {
        !self.cpu.get_flag(Flag::Z)
    }
}