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
    cpu: Cpu
}

impl Emulator {
    pub fn new() -> Emulator {
        Emulator {
            tick_count: 0,
            memory: Memory::new(),
            cpu: Cpu::new()
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
        Ok(())
    }
}