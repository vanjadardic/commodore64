use std::time::Duration;

use crate::emulator::cpu::Cpu;
use crate::emulator::memory::Memory;

const MASTER_CLOCK_PAL: u128 = 17_734_475;
const MASTER_CLOCK_NTSC: u128 = 14_318_180;

const CLOCK_PAL: u128 = 985_248;
const CLOCK_NTSC: u128 = 1_022_727;

const CLOCK_VICII_PAL: u128 = CLOCK_PAL * 8;
const CLOCK_VICII_NTSC: u128 = CLOCK_NTSC * 8;

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
        Emulator {
            tick_count: 0,
            memory: Memory::new(),
            cpu: Cpu::new(),
            sub_tick: 1,
            opcode: 0,
            low_address_byte: 0,
            high_address_byte: 0,
        }
    }

    pub fn load(&mut self) {
        self.memory.set(0x0000, 0x4C);
        self.memory.set(0x0001, 0xF0);
        self.memory.set(0x0002, 0x33);
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
            x @ 0xAD => { // LDA
                if self.sub_tick == 2 || self.sub_tick == 3 {
                    self.absolute_addressing();
                    return Ok(());
                }
                if self.sub_tick == 4 {
                    self.cpu.a = self.memory.get_from_low_high(self.low_address_byte, self.high_address_byte);
                    self.sub_tick = 1;
                    return Ok(());
                }
                Err(format!("Illegal sub_tick {} for opcode {:#04x}", self.sub_tick, x))
            }
            x @ 0xAE => { // LDX
                if self.sub_tick == 2 || self.sub_tick == 3 {
                    self.absolute_addressing();
                    return Ok(());
                }
                if self.sub_tick == 4 {
                    self.cpu.x = self.memory.get_from_low_high(self.low_address_byte, self.high_address_byte);
                    self.sub_tick = 1;
                    return Ok(());
                }
                Err(format!("Illegal sub_tick {} for opcode {:#04x}", self.sub_tick, x))
            }
            x @ 0xAC => { // LDY
                if self.sub_tick == 2 || self.sub_tick == 3 {
                    self.absolute_addressing();
                    return Ok(());
                }
                if self.sub_tick == 4 {
                    self.cpu.y = self.memory.get_from_low_high(self.low_address_byte, self.high_address_byte);
                    self.sub_tick = 1;
                    return Ok(());
                }
                Err(format!("Illegal sub_tick {} for opcode {:#04x}", self.sub_tick, x))
            }
            x => Err(format!("Unknown opcode {:#04x}", x))
        }
    }

    fn absolute_addressing(&mut self) {
        if self.sub_tick == 2 {
            self.low_address_byte = self.memory.get_from_pc(self.cpu.get_and_increment_pc());
            self.sub_tick += 1;
        }
        if self.sub_tick == 3 {
            self.high_address_byte = self.memory.get_from_pc(self.cpu.get_and_increment_pc());
            self.sub_tick += 1;
        }
    }
}