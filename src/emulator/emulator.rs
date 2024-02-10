use std::time::Duration;

use crate::emulator::addressing::Addressing;
use crate::emulator::cpu::Cpu;
use crate::emulator::gpu::Gpu;
use crate::emulator::keyboard::{Key, Keyboard};
use crate::emulator::logger::CpuLogger;
use crate::emulator::memory::Memory;
use crate::emulator::timer_a::TimerA;

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
    pub gpu: Gpu,
    addressing: Addressing,
    cpu_logger: CpuLogger,
    timer_a: TimerA,
    keyboard: Keyboard,
}

impl Emulator {
    pub fn new() -> Emulator {
        let mut memory = Memory::new();
        let mut cpu = Cpu::new();
        let low = memory.get_from_word(0xFFFC);
        let high = memory.get_from_word(0xFFFD);
        cpu.set_pc(low, high);
        Emulator {
            tick_count: 0,
            memory,
            cpu,
            gpu: Gpu::new(),
            addressing: Addressing::new(),
            cpu_logger: CpuLogger::new(),
            timer_a: TimerA::new(),
            keyboard: Keyboard::new(),
        }
    }

    pub fn step(&mut self, elapsed: Duration) -> Result<(), String> {
        let want_ticks = ((elapsed.as_nanos() * CLOCK) / NANOS_PER_SEC) as u64;
        while self.tick_count < want_ticks {
            self.memory.cia1().port_b_read_or(0xFF);

            for (i, pressed) in self.keyboard.pressed().iter().enumerate() {
                if *pressed {
                    let col = 1 << (i % 8);
                    if self.memory.cia1().port_a_direction() & col > 0 {
                        //debug!("port_a_write={}", self.memory.cia1().port_a_write());
                        if self.memory.cia1().port_a_write() & col == 0 {
                            let row = 1 << (i / 8);
                            self.memory.cia1().port_b_read_and(!row);
                            //debug!("port_b_read={}", self.memory.cia1().port_b_read());
                        }
                    }
                }
            }

            self.cpu_logger.set_tick(self.tick_count);
            if self.tick_count == 2118528 {
                //debug!("{}", self.tick_count);
            }
            self.gpu.tick(&self.memory);
            self.timer_a.tick(&mut self.cpu, self.memory.cia1());
            self.cpu.tick(&mut self.cpu_logger, &mut self.memory, &mut self.addressing)?;
            self.tick_count += 1;
        }
        Ok(())
    }

    pub fn key_change(&mut self, key: Key, pressed: bool) {
        self.keyboard.change_key_state(key, pressed);
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::emulator::cpu::Cpu;
//     use crate::emulator::emulator::Emulator;
//
//     #[test]
//     fn relative_addressing() {
//         let mut e = Emulator::new();
//         e.sub_tick = 3;
//         e.latch = (12 as i8) as u8;
//         e.cpu.pc = 100;
//         e.cpu.set_pch(1);
//         e.relative_addressing(|_: &mut Cpu| -> bool{ true }, 0).unwrap();
//         assert_eq!(e.cpu.get_pcl(), 112);
//         assert_eq!(e.sub_tick, 1);
//
//         e.sub_tick = 3;
//         e.latch = (-12 as i8) as u8;
//         e.cpu.pc = 100;
//         e.cpu.set_pch(1);
//         e.relative_addressing(|_: &mut Cpu| -> bool{ true }, 0).unwrap();
//         assert_eq!(e.cpu.get_pcl(), 88);
//         assert_eq!(e.sub_tick, 1);
//
//         e.sub_tick = 3;
//         e.latch = (-12 as i8) as u8;
//         e.cpu.pc = 12;
//         e.cpu.set_pch(1);
//         e.relative_addressing(|_: &mut Cpu| -> bool{ true }, 0).unwrap();
//         assert_eq!(e.cpu.get_pcl(), 0);
//         assert_eq!(e.sub_tick, 1);
//
//         e.sub_tick = 3;
//         e.latch = (-12 as i8) as u8;
//         e.cpu.pc = 3;
//         e.cpu.set_pch(1);
//         e.relative_addressing(|_: &mut Cpu| -> bool{ true }, 0).unwrap();
//         assert_eq!(e.cpu.get_pcl(), 247);
//         assert_ne!(e.sub_tick, 1);
//         assert_eq!(e.high, 0);
//
//         e.sub_tick = 3;
//         e.latch = (100 as i8) as u8;
//         e.cpu.pc = 200;
//         e.cpu.set_pch(1);
//         e.relative_addressing(|_: &mut Cpu| -> bool{ true }, 0).unwrap();
//         assert_eq!(e.cpu.get_pcl(), 44);
//         assert_ne!(e.sub_tick, 1);
//         assert_eq!(e.high, 2);
//     }
// }