use std::time::Duration;

use crate::emulator::addressing::Addressing;
use crate::emulator::cpu::Cpu;
use crate::emulator::logger::CpuLogger;
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
    addressing: Addressing,
    /// tick counter within an instruction
    sub_tick: u8,
    /// currently executing instruction
    opcode: u8,
    cpu_logger: CpuLogger,
}

impl Emulator {
    pub fn new() -> Emulator {
        let memory = Memory::new();
        let mut cpu = Cpu::new();
        let low = memory.get_from_word(0xFFFC);
        let high = memory.get_from_word(0xFFFC + 1);
        cpu.set_pc(low, high);
        Emulator {
            tick_count: 0,
            memory,
            cpu,
            addressing: Addressing::new(),
            sub_tick: 1,
            opcode: 0,
            cpu_logger: CpuLogger::new(),
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
            self.cpu_logger.init(self.tick_count, &self.cpu);
            let pc = self.cpu.get_and_increment_pc();
            // if pc == 0xFD25 {
            //     println!();
            // }
            self.opcode = self.memory.get_from_word(pc);
            self.cpu_logger.opcode(self.opcode);
            self.sub_tick += 1;
            return Ok(());
        }
        self.sub_tick = match self.opcode {
            x @ 0x05 => self.addressing.zero_page_read(self.sub_tick, &mut self.cpu, &self.memory, Cpu::ora, x),
            x @ 0x09 => self.addressing.immediate(self.sub_tick, &mut self.cpu, &self.memory, Cpu::ora, x),
            x @ 0x0A => self.addressing.accumulator(self.sub_tick, &mut self.cpu, Cpu::asl, x),
            x @ 0x0D => self.addressing.absolute_read(self.sub_tick, &mut self.cpu, &self.memory, Cpu::ora, x),
            x @ 0x10 => self.addressing.relative(self.sub_tick, &mut self.cpu, &self.memory, Cpu::bpl, x),
            x @ 0x18 => self.addressing.implied(self.sub_tick, &mut self.cpu, Cpu::clc, x),
            x @ 0x20 => self.addressing.absolute_jsr(self.sub_tick, &mut self.cpu, &mut self.memory, x),
            x @ 0x24 => self.addressing.zero_page_read(self.sub_tick, &mut self.cpu, &self.memory, Cpu::bit, x),
            x @ 0x29 => self.addressing.immediate(self.sub_tick, &mut self.cpu, &self.memory, Cpu::and, x),
            x @ 0x2A => self.addressing.accumulator(self.sub_tick, &mut self.cpu, Cpu::rol, x),
            x @ 0x2D => self.addressing.absolute_read(self.sub_tick, &mut self.cpu, &self.memory, Cpu::and, x),
            x @ 0x30 => self.addressing.relative(self.sub_tick, &mut self.cpu, &self.memory, Cpu::bmi, x),
            x @ 0x38 => self.addressing.implied(self.sub_tick, &mut self.cpu, Cpu::sec, x),
            // x @ 0x40 => { //implied/stack
            //     if self.sub_tick == 2 {
            //         self.sub_tick += 1;
            //         return Ok(());
            //     }
            //     if self.sub_tick == 3 {
            //         self.cpu.sp = self.cpu.sp.wrapping_add(1);
            //         self.sub_tick += 1;
            //         return Ok(());
            //     }
            //     if self.sub_tick == 4 {
            //         self.cpu.p = self.memory.get_stack(self.cpu.sp);
            //         self.cpu.sp = self.cpu.sp.wrapping_add(1);
            //         self.sub_tick += 1;
            //         return Ok(());
            //     }
            //     if self.sub_tick == 5 {
            //         self.cpu.set_pcl(self.memory.get_stack(self.cpu.sp));
            //         self.cpu.sp = self.cpu.sp.wrapping_add(1);
            //         self.sub_tick += 1;
            //         return Ok(());
            //     }
            //     if self.sub_tick == 6 {
            //         self.cpu.set_pch(self.memory.get_stack(self.cpu.sp));
            //         self.cpu_logger.borrow_mut().instruction("RTI");
            //         self.sub_tick = 1;
            //         return Ok(());
            //     }
            //     Err(format!("Illegal sub_tick {} for opcode {:02X}", self.sub_tick, x))
            // }
            x @ 0x48 => self.addressing.implied_pha(self.sub_tick, &mut self.cpu, &mut self.memory, x),
            x @ 0x4C => self.addressing.absolute_jmp(self.sub_tick, &mut self.cpu, &self.memory, x),
            x @ 0x58 => self.addressing.implied(self.sub_tick, &mut self.cpu, Cpu::cli, x),
            x @ 0x60 => self.addressing.implied_rts(self.sub_tick, &mut self.cpu, &self.memory, x),
            x @ 0x68 => self.addressing.implied_pla(self.sub_tick, &mut self.cpu, &self.memory, x),
            x @ 0x69 => self.addressing.immediate(self.sub_tick, &mut self.cpu, &self.memory, Cpu::adc, x),
            x @ 0x6C => self.addressing.absolute_indirect_jmp(self.sub_tick, &mut self.cpu, &self.memory, x),
            x @ 0x78 => self.addressing.implied(self.sub_tick, &mut self.cpu, Cpu::sei, x),
            x @ 0x84 => self.addressing.zero_page_write(self.sub_tick, &mut self.cpu, &mut self.memory, Cpu::sty, x),
            x @ 0x85 => self.addressing.zero_page_write(self.sub_tick, &mut self.cpu, &mut self.memory, Cpu::sta, x),
            x @ 0x86 => self.addressing.zero_page_write(self.sub_tick, &mut self.cpu, &mut self.memory, Cpu::stx, x),
            x @ 0x88 => self.addressing.implied(self.sub_tick, &mut self.cpu, Cpu::dey, x),
            x @ 0x8A => self.addressing.implied(self.sub_tick, &mut self.cpu, Cpu::txa, x),
            x @ 0x8C => self.addressing.absolute_write(self.sub_tick, &mut self.cpu, &mut self.memory, Cpu::sty, x),
            x @ 0x8D => self.addressing.absolute_write(self.sub_tick, &mut self.cpu, &mut self.memory, Cpu::sta, x),
            x @ 0x8E => self.addressing.absolute_write(self.sub_tick, &mut self.cpu, &mut self.memory, Cpu::stx, x),
            x @ 0x90 => self.addressing.relative(self.sub_tick, &mut self.cpu, &self.memory, Cpu::bcc, x),
            x @ 0x91 => self.addressing.indirect_indexed_write(self.sub_tick, &mut self.cpu, &mut self.memory, Cpu::sta, x),
            x @ 0x94 => self.addressing.zero_page_indexed_write_x(self.sub_tick, &mut self.cpu, &mut self.memory, Cpu::sty, x),
            x @ 0x95 => self.addressing.zero_page_indexed_write_x(self.sub_tick, &mut self.cpu, &mut self.memory, Cpu::sta, x),
            x @ 0x98 => self.addressing.implied(self.sub_tick, &mut self.cpu, Cpu::tya, x),
            x @ 0x99 => self.addressing.absolute_indexed_write_y(self.sub_tick, &mut self.cpu, &mut self.memory, Cpu::sta, x),
            x @ 0x9A => self.addressing.implied(self.sub_tick, &mut self.cpu, Cpu::txs, x),
            x @ 0x9D => self.addressing.absolute_indexed_write_x(self.sub_tick, &mut self.cpu, &mut self.memory, Cpu::sta, x),
            x @ 0xA0 => self.addressing.immediate(self.sub_tick, &mut self.cpu, &self.memory, Cpu::ldy, x),
            x @ 0xA2 => self.addressing.immediate(self.sub_tick, &mut self.cpu, &self.memory, Cpu::ldx, x),
            x @ 0xA4 => self.addressing.zero_page_read(self.sub_tick, &mut self.cpu, &self.memory, Cpu::ldy, x),
            x @ 0xA5 => self.addressing.zero_page_read(self.sub_tick, &mut self.cpu, &self.memory, Cpu::lda, x),
            x @ 0xA6 => self.addressing.zero_page_read(self.sub_tick, &mut self.cpu, &self.memory, Cpu::ldx, x),
            x @ 0xA8 => self.addressing.implied(self.sub_tick, &mut self.cpu, Cpu::tay, x),
            x @ 0xA9 => self.addressing.immediate(self.sub_tick, &mut self.cpu, &self.memory, Cpu::lda, x),
            x @ 0xAA => self.addressing.implied(self.sub_tick, &mut self.cpu, Cpu::tax, x),
            x @ 0xAC => self.addressing.absolute_read(self.sub_tick, &mut self.cpu, &self.memory, Cpu::ldy, x),
            x @ 0xAD => self.addressing.absolute_read(self.sub_tick, &mut self.cpu, &self.memory, Cpu::lda, x),
            x @ 0xAE => self.addressing.absolute_read(self.sub_tick, &mut self.cpu, &self.memory, Cpu::ldx, x),
            x @ 0xB0 => self.addressing.relative(self.sub_tick, &mut self.cpu, &self.memory, Cpu::bcs, x),
            x @ 0xB1 => self.addressing.indirect_indexed_read(self.sub_tick, &mut self.cpu, &self.memory, Cpu::lda, x),
            x @ 0xB4 => self.addressing.zero_page_indexed_read_x(self.sub_tick, &mut self.cpu, &self.memory, Cpu::ldy, x),
            x @ 0xB5 => self.addressing.zero_page_indexed_read_x(self.sub_tick, &mut self.cpu, &self.memory, Cpu::lda, x),
            x @ 0xB9 => self.addressing.absolute_indexed_read_y(self.sub_tick, &mut self.cpu, &self.memory, Cpu::lda, x),
            x @ 0xBD => self.addressing.absolute_indexed_read_x(self.sub_tick, &mut self.cpu, &self.memory, Cpu::lda, x),
            x @ 0xC4 => self.addressing.zero_page_read(self.sub_tick, &mut self.cpu, &self.memory, Cpu::cpy, x),
            x @ 0xC5 => self.addressing.zero_page_read(self.sub_tick, &mut self.cpu, &self.memory, Cpu::cmp, x),
            x @ 0xC8 => self.addressing.implied(self.sub_tick, &mut self.cpu, Cpu::iny, x),
            x @ 0xC9 => self.addressing.immediate(self.sub_tick, &mut self.cpu, &self.memory, Cpu::cmp, x),
            x @ 0xCA => self.addressing.implied(self.sub_tick, &mut self.cpu, Cpu::dex, x),
            x @ 0xCD => self.addressing.absolute_read(self.sub_tick, &mut self.cpu, &self.memory, Cpu::cmp, x),
            x @ 0xD0 => self.addressing.relative(self.sub_tick, &mut self.cpu, &self.memory, Cpu::bne, x),
            x @ 0xD1 => self.addressing.indirect_indexed_read(self.sub_tick, &mut self.cpu, &self.memory, Cpu::cmp, x),
            x @ 0xD8 => self.addressing.implied(self.sub_tick, &mut self.cpu, Cpu::cld, x),
            x @ 0xDD => self.addressing.absolute_indexed_read_x(self.sub_tick, &mut self.cpu, &self.memory, Cpu::cmp, x),
            x @ 0xE0 => self.addressing.immediate(self.sub_tick, &mut self.cpu, &self.memory, Cpu::cpx, x),
            x @ 0xE4 => self.addressing.zero_page_read(self.sub_tick, &mut self.cpu, &self.memory, Cpu::cpx, x),
            x @ 0xE6 => self.addressing.zero_page_read_modify_write(self.sub_tick, &mut self.cpu, &mut self.memory, Cpu::inc, x),
            x @ 0xE8 => self.addressing.implied(self.sub_tick, &mut self.cpu, Cpu::inx, x),
            x @ 0xEC => self.addressing.absolute_read(self.sub_tick, &mut self.cpu, &self.memory, Cpu::cpx, x),
            x @ 0xF0 => self.addressing.relative(self.sub_tick, &mut self.cpu, &self.memory, Cpu::beq, x),
            x => Err(format!("Illegal opcode {:02X} at {:04X}", x, self.cpu.pc - 1))
        }?;

        if self.sub_tick == 1 {
            self.cpu_logger.log(&self.cpu, &self.addressing);
        }

        Ok(())
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