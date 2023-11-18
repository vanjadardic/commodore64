use std::time::Duration;

use crate::emulator::cpu::Cpu;
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
    low_address_byte: u8,
    high_address_byte: u8,
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
            low_address_byte: 0,
            high_address_byte: 0,
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
            x @ 0x4C => { // JMP
                if self.sub_tick == 2 {
                    self.low_address_byte = self.memory.get_from_pc(self.cpu.get_and_increment_pc());
                    self.sub_tick += 1;
                    return Ok(());
                }
                if self.sub_tick == 3 {
                    self.high_address_byte = self.memory.get_from_pc(self.cpu.pc);
                    self.cpu.set_pc(self.low_address_byte, self.high_address_byte);
                    self.sub_tick = 1;
                    return Ok(());
                }
                Err(format!("Illegal sub_tick {} for opcode {:#04x}", self.sub_tick, x))
            }
            x @ 0xAD => self.absolute_addressing(Emulator::lda, x),
            x @ 0xA2 => self.immediate_addressing(Emulator::ldx, x),
            x @ 0xAE => self.absolute_addressing(Emulator::ldx, x),
            x @ 0xAC => self.absolute_addressing(Emulator::ldy, x),
            x => Err(format!("Illegal opcode {:#04x}", x))
        }
    }

    fn absolute_addressing(&mut self, op: fn(&mut Emulator, u8), opcode: u8) -> Result<(), String> {
        if self.sub_tick == 2 {
            self.low_address_byte = self.memory.get_from_pc(self.cpu.get_and_increment_pc());
            self.sub_tick += 1;
            return Ok(());
        }
        if self.sub_tick == 3 {
            self.high_address_byte = self.memory.get_from_pc(self.cpu.get_and_increment_pc());
            self.sub_tick += 1;
            return Ok(());
        }
        if self.sub_tick == 4 {
            let value = self.memory.get_from_low_high(self.low_address_byte, self.high_address_byte);
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

    fn lda(&mut self, value: u8) {
        self.cpu.a = value;
    }

    fn ldx(&mut self, value: u8) {
        self.cpu.x = value;
    }

    fn ldy(&mut self, value: u8) {
        self.cpu.y = value;
    }
}