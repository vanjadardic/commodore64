use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

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
    /// tick counter within an instruction
    sub_tick: u8,
    /// currently executing instruction
    opcode: u8,
    low: u8,
    high: u8,
    fix_high: bool,
    latch: u8,
    cpu_logger: Rc<RefCell<CpuLogger>>,
}

impl Emulator {
    pub fn new() -> Emulator {
        let cpu_logger = Rc::new(RefCell::new(CpuLogger::new()));
        let memory = Memory::new();
        let mut cpu = Cpu::new(Rc::clone(&cpu_logger));
        let low = memory.get_from_word(0xFFFC);
        let high = memory.get_from_word(0xFFFC + 1);
        cpu.set_pc(low, high);
        Emulator {
            tick_count: 0,
            memory,
            cpu,
            sub_tick: 1,
            opcode: 0,
            low: 0,
            high: 0,
            fix_high: false,
            latch: 0,
            cpu_logger,
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
            self.cpu_logger.borrow_mut().init(self.tick_count, &self.cpu);
            // if 5337 == self.tick_count {
            //     println!()
            // }
            let pc = self.cpu.get_and_increment_pc();
            // if pc == 0xFF5E {
            //     println!();
            // }
            self.opcode = self.memory.get_from_word(pc);
            self.cpu_logger.borrow_mut().opcode(self.opcode);
            self.sub_tick += 1;
            return Ok(());
        }
        match self.opcode {
            x @ 0x09 => self.immediate_addressing(Cpu::ora, x),
            x @ 0x0D => self.absolute_addressing_read(Cpu::ora, x),
            x @ 0x10 => self.relative_addressing(Cpu::bpl, x),
            x @ 0x18 => self.implied_addressing(Cpu::clc, x),
            x @ 0x20 => { //absolute addressing
                if self.sub_tick == 2 {
                    self.low = self.memory.get_from_word(self.cpu.get_and_increment_pc());
                    self.cpu_logger.borrow_mut().operand(self.low);
                    self.sub_tick += 1;
                    return Ok(());
                }
                if self.sub_tick == 3 {
                    self.sub_tick += 1;
                    return Ok(());
                }
                if self.sub_tick == 4 {
                    self.memory.set_stack(self.cpu.sp, self.cpu.get_pch());
                    self.cpu.sp = self.cpu.sp.wrapping_sub(1);
                    self.sub_tick += 1;
                    return Ok(());
                }
                if self.sub_tick == 5 {
                    self.memory.set_stack(self.cpu.sp, self.cpu.get_pcl());
                    self.cpu.sp = self.cpu.sp.wrapping_sub(1);
                    self.sub_tick += 1;
                    return Ok(());
                }
                if self.sub_tick == 6 {
                    self.high = self.memory.get_from_word(self.cpu.pc);
                    self.cpu_logger.borrow_mut().operand(self.high);
                    self.cpu.set_pc(self.low, self.high);
                    self.cpu_logger.borrow_mut().instruction("JSR");
                    self.sub_tick = 1;
                    return Ok(());
                }
                Err(format!("Illegal sub_tick {} for opcode {:02X}", self.sub_tick, x))
            }
            x @ 0x29 => self.immediate_addressing(Cpu::and, x),
            x @ 0x2A => self.accumulator_addressing(Cpu::rol, x),
            x @ 0x2D => self.absolute_addressing_read(Cpu::and, x),
            x @ 0x30 => self.relative_addressing(Cpu::bmi, x),
            x @ 0x40 => { //implied/stack
                if self.sub_tick == 2 {
                    self.sub_tick += 1;
                    return Ok(());
                }
                if self.sub_tick == 3 {
                    self.cpu.sp = self.cpu.sp.wrapping_add(1);
                    self.sub_tick += 1;
                    return Ok(());
                }
                if self.sub_tick == 4 {
                    self.cpu.p = self.memory.get_stack(self.cpu.sp);
                    self.cpu.sp = self.cpu.sp.wrapping_add(1);
                    self.sub_tick += 1;
                    return Ok(());
                }
                if self.sub_tick == 5 {
                    self.cpu.set_pcl(self.memory.get_stack(self.cpu.sp));
                    self.cpu.sp = self.cpu.sp.wrapping_add(1);
                    self.sub_tick += 1;
                    return Ok(());
                }
                if self.sub_tick == 6 {
                    self.cpu.set_pch(self.memory.get_stack(self.cpu.sp));
                    self.cpu_logger.borrow_mut().instruction("RTI");
                    self.sub_tick = 1;
                    return Ok(());
                }
                Err(format!("Illegal sub_tick {} for opcode {:02X}", self.sub_tick, x))
            }
            x @ 0x4C => { // absolute
                if self.sub_tick == 2 {
                    self.low = self.memory.get_from_word(self.cpu.get_and_increment_pc());
                    self.cpu_logger.borrow_mut().operand(self.low);
                    self.sub_tick += 1;
                    return Ok(());
                }
                if self.sub_tick == 3 {
                    self.high = self.memory.get_from_word(self.cpu.pc);
                    self.cpu_logger.borrow_mut().operand(self.high);
                    self.cpu.set_pc(self.low, self.high);
                    self.cpu_logger.borrow_mut().instruction("JMP");
                    self.sub_tick = 1;
                    return Ok(());
                }
                Err(format!("Illegal sub_tick {} for opcode {:02X}", self.sub_tick, x))
            }
            x @ 0x60 => { //implied/stack
                if self.sub_tick == 2 {
                    self.sub_tick += 1;
                    return Ok(());
                }
                if self.sub_tick == 3 {
                    self.cpu.sp = self.cpu.sp.wrapping_add(1);
                    self.sub_tick += 1;
                    return Ok(());
                }
                if self.sub_tick == 4 {
                    self.cpu.set_pcl(self.memory.get_stack(self.cpu.sp));
                    self.cpu.sp = self.cpu.sp.wrapping_add(1);
                    self.sub_tick += 1;
                    return Ok(());
                }
                if self.sub_tick == 5 {
                    self.cpu.set_pch(self.memory.get_stack(self.cpu.sp));
                    self.sub_tick += 1;
                    return Ok(());
                }
                if self.sub_tick == 6 {
                    self.cpu.get_and_increment_pc();
                    self.cpu_logger.borrow_mut().instruction("RTS");
                    self.sub_tick = 1;
                    return Ok(());
                }
                Err(format!("Illegal sub_tick {} for opcode {:02X}", self.sub_tick, x))
            }
            x @ 0x68 => { //implied/stack
                if self.sub_tick == 2 {
                    self.sub_tick += 1;
                    return Ok(());
                }
                if self.sub_tick == 3 {
                    self.cpu.sp = self.cpu.sp.wrapping_add(1);
                    self.sub_tick += 1;
                    return Ok(());
                }
                if self.sub_tick == 4 {
                    self.cpu.a = self.memory.get_stack(self.cpu.sp);
                    self.cpu.set_negative_and_zero_flags(self.cpu.a);
                    self.cpu_logger.borrow_mut().instruction("PLA");
                    self.sub_tick = 1;
                    return Ok(());
                }
                Err(format!("Illegal sub_tick {} for opcode {:02X}", self.sub_tick, x))
            }
            x @ 0x69 => self.immediate_addressing(Cpu::adc, x),
            x @ 0x6C => { //absolute indirect
                if self.sub_tick == 2 {
                    self.low = self.memory.get_from_word(self.cpu.get_and_increment_pc());
                    self.cpu_logger.borrow_mut().operand(self.low);
                    self.sub_tick += 1;
                    return Ok(());
                }
                if self.sub_tick == 3 {
                    self.high = self.memory.get_from_word(self.cpu.get_and_increment_pc());
                    self.cpu_logger.borrow_mut().operand(self.high);
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
                    self.cpu_logger.borrow_mut().instruction("JMP");
                    self.sub_tick = 1;
                    return Ok(());
                }
                Err(format!("Illegal sub_tick {} for opcode {:02X}", self.sub_tick, x))
            }
            x @ 0x78 => self.implied_addressing(Cpu::sei, x),
            x @ 0x84 => self.zero_page_addressing_write(Cpu::sty, x),
            x @ 0x85 => self.zero_page_addressing_write(Cpu::sta, x),
            x @ 0x86 => self.zero_page_addressing_write(Cpu::stx, x),
            x @ 0x88 => self.implied_addressing(Cpu::dey, x),
            x @ 0x8A => self.implied_addressing(Cpu::txa, x),
            x @ 0x8C => self.absolute_addressing_write(Cpu::sty, x),
            x @ 0x8D => self.absolute_addressing_write(Cpu::sta, x),
            x @ 0x8E => self.absolute_addressing_write(Cpu::stx, x),
            x @ 0x90 => self.relative_addressing(Cpu::bcc, x),
            x @ 0x91 => self.indirect_indexed_addressing_write(Cpu::sta, x),
            x @ 0x94 => self.zero_page_indexed_addressing_write_x(Cpu::sty, x),
            x @ 0x95 => self.zero_page_indexed_addressing_write_x(Cpu::sta, x),
            x @ 0x98 => self.implied_addressing(Cpu::tya, x),
            x @ 0x99 => self.absolute_indexed_addressing_write_y(Cpu::sta, x),
            x @ 0x9A => self.implied_addressing(Cpu::txs, x),
            x @ 0x9D => self.absolute_indexed_addressing_write_x(Cpu::sta, x),
            x @ 0xA0 => self.immediate_addressing(Cpu::ldy, x),
            x @ 0xA2 => self.immediate_addressing(Cpu::ldx, x),
            x @ 0xA4 => self.zero_page_addressing_read(Cpu::ldy, x),
            x @ 0xA5 => self.zero_page_addressing_read(Cpu::lda, x),
            x @ 0xA6 => self.zero_page_addressing_read(Cpu::ldx, x),
            x @ 0xA8 => self.implied_addressing(Cpu::tay, x),
            x @ 0xA9 => self.immediate_addressing(Cpu::lda, x),
            x @ 0xAA => self.implied_addressing(Cpu::tax, x),
            x @ 0xAD => self.absolute_addressing_read(Cpu::lda, x),
            x @ 0xAE => self.absolute_addressing_read(Cpu::ldx, x),
            x @ 0xB0 => self.relative_addressing(Cpu::bcs, x),
            x @ 0xB1 => self.indirect_indexed_addressing_read(Cpu::lda, x),
            x @ 0xB4 => self.zero_page_indexed_addressing_read_x(Cpu::ldy, x),
            x @ 0xB5 => self.zero_page_indexed_addressing_read_x(Cpu::lda, x),
            x @ 0xB9 => self.absolute_indexed_addressing_read_y(Cpu::lda, x),
            x @ 0xBD => self.absolute_indexed_addressing_read_x(Cpu::lda, x),
            x @ 0xC8 => self.implied_addressing(Cpu::iny, x),
            x @ 0xC9 => self.immediate_addressing(Cpu::cmp, x),
            x @ 0xCA => self.implied_addressing(Cpu::dex, x),
            x @ 0xCD => self.absolute_addressing_read(Cpu::cmp, x),
            x @ 0xD0 => self.relative_addressing(Cpu::bne, x),
            x @ 0xD1 => self.indirect_indexed_addressing_read(Cpu::cmp, x),
            x @ 0xD8 => self.implied_addressing(Cpu::cld, x),
            x @ 0xDD => self.absolute_indexed_addressing_read_x(Cpu::cmp, x),
            x @ 0xE0 => self.immediate_addressing(Cpu::cpx, x),
            x @ 0xE6 => self.zero_page_addressing_read_modify_write(Cpu::inc, x),
            x @ 0xE8 => self.implied_addressing(Cpu::inx, x),
            x @ 0xEC => self.absolute_addressing_read(Cpu::cpx, x),
            x @ 0xF0 => self.relative_addressing(Cpu::beq, x),
            x => Err(format!("Illegal opcode {:02X} at {:04X}", x, self.cpu.pc - 1))
        }
    }

    fn accumulator_addressing(&mut self, op: fn(&mut Cpu, u8) -> u8, opcode: u8) -> Result<(), String> {
        if self.sub_tick == 2 {
            let a = self.cpu.a;
            self.cpu.a = op(&mut self.cpu, a);
            self.sub_tick = 1;
            return Ok(());
        }
        Err(format!("Illegal sub_tick {} for opcode {:02X}", self.sub_tick, opcode))
    }

    fn absolute_addressing_write(&mut self, op: fn(&mut Cpu) -> u8, opcode: u8) -> Result<(), String> {
        if self.sub_tick == 2 {
            self.low = self.memory.get_from_word(self.cpu.get_and_increment_pc());
            self.cpu_logger.borrow_mut().operand(self.low);
            self.sub_tick += 1;
            return Ok(());
        }
        if self.sub_tick == 3 {
            self.high = self.memory.get_from_word(self.cpu.get_and_increment_pc());
            self.cpu_logger.borrow_mut().operand(self.high);
            self.sub_tick += 1;
            return Ok(());
        }
        if self.sub_tick == 4 {
            let value = op(&mut self.cpu);
            self.memory.set_from_low_high(self.low, self.high, value);
            self.sub_tick = 1;
            return Ok(());
        }
        Err(format!("Illegal sub_tick {} for opcode {:02X}", self.sub_tick, opcode))
    }

    fn absolute_addressing_read(&mut self, op: fn(&mut Cpu, u8), opcode: u8) -> Result<(), String> {
        if self.sub_tick == 2 {
            self.low = self.memory.get_from_word(self.cpu.get_and_increment_pc());
            self.cpu_logger.borrow_mut().operand(self.low);
            self.sub_tick += 1;
            return Ok(());
        }
        if self.sub_tick == 3 {
            self.high = self.memory.get_from_word(self.cpu.get_and_increment_pc());
            self.cpu_logger.borrow_mut().operand(self.high);
            self.sub_tick += 1;
            return Ok(());
        }
        if self.sub_tick == 4 {
            let value = self.memory.get_from_low_high(self.low, self.high);
            op(&mut self.cpu, value);
            self.sub_tick = 1;
            return Ok(());
        }
        Err(format!("Illegal sub_tick {} for opcode {:02X}", self.sub_tick, opcode))
    }

    fn immediate_addressing(&mut self, op: fn(&mut Cpu, u8), opcode: u8) -> Result<(), String> {
        if self.sub_tick == 2 {
            let value = self.memory.get_from_word(self.cpu.get_and_increment_pc());
            self.cpu_logger.borrow_mut().operand(value);
            op(&mut self.cpu, value);
            self.sub_tick = 1;
            return Ok(());
        }
        Err(format!("Illegal sub_tick {} for opcode {:02X}", self.sub_tick, opcode))
    }

    fn zero_page_addressing_read(&mut self, op: fn(&mut Cpu, u8), opcode: u8) -> Result<(), String> {
        if self.sub_tick == 2 {
            self.low = self.memory.get_from_word(self.cpu.get_and_increment_pc());
            self.cpu_logger.borrow_mut().operand(self.low);
            self.sub_tick += 1;
            return Ok(());
        }
        if self.sub_tick == 3 {
            let value = self.memory.get_from_low(self.low);
            op(&mut self.cpu, value);
            self.sub_tick = 1;
            return Ok(());
        }
        Err(format!("Illegal sub_tick {} for opcode {:02X}", self.sub_tick, opcode))
    }

    fn zero_page_addressing_read_modify_write(&mut self, op: fn(&mut Cpu, u8) -> u8, opcode: u8) -> Result<(), String> {
        if self.sub_tick == 2 {
            self.low = self.memory.get_from_word(self.cpu.get_and_increment_pc());
            self.cpu_logger.borrow_mut().operand(self.low);
            self.sub_tick += 1;
            return Ok(());
        }
        if self.sub_tick == 3 {
            self.latch = self.memory.get_from_low(self.low);
            self.sub_tick += 1;
            return Ok(());
        }
        if self.sub_tick == 4 {
            self.latch = op(&mut self.cpu, self.latch);
            self.sub_tick += 1;
            return Ok(());
        }
        if self.sub_tick == 5 {
            self.memory.set_from_low(self.low, self.latch);
            self.sub_tick = 1;
            return Ok(());
        }
        Err(format!("Illegal sub_tick {} for opcode {:02X}", self.sub_tick, opcode))
    }

    fn zero_page_indexed_addressing_read(&mut self, op: fn(&mut Cpu, u8), index: u8, opcode: u8) -> Result<(), String> {
        if self.sub_tick == 2 {
            self.low = self.memory.get_from_word(self.cpu.get_and_increment_pc());
            self.cpu_logger.borrow_mut().operand(self.low);
            self.sub_tick += 1;
            return Ok(());
        }
        if self.sub_tick == 3 {
            self.low = self.low.wrapping_add(index);
            self.sub_tick += 1;
            return Ok(());
        }
        if self.sub_tick == 4 {
            let value = self.memory.get_from_low(self.low);
            op(&mut self.cpu, value);
            self.sub_tick = 1;
            return Ok(());
        }
        Err(format!("Illegal sub_tick {} for opcode {:02X}", self.sub_tick, opcode))
    }

    fn zero_page_indexed_addressing_read_x(&mut self, op: fn(&mut Cpu, u8), opcode: u8) -> Result<(), String> {
        self.zero_page_indexed_addressing_read(op, self.cpu.x, opcode)
    }

    fn zero_page_indexed_addressing_read_y(&mut self, op: fn(&mut Cpu, u8), opcode: u8) -> Result<(), String> {
        self.zero_page_indexed_addressing_read(op, self.cpu.y, opcode)
    }

    fn zero_page_indexed_addressing_write(&mut self, op: fn(&mut Cpu) -> u8, index: u8, opcode: u8) -> Result<(), String> {
        if self.sub_tick == 2 {
            self.low = self.memory.get_from_word(self.cpu.get_and_increment_pc());
            self.cpu_logger.borrow_mut().operand(self.low);
            self.sub_tick += 1;
            return Ok(());
        }
        if self.sub_tick == 3 {
            self.low = self.low.wrapping_add(index);
            self.sub_tick += 1;
            return Ok(());
        }
        if self.sub_tick == 4 {
            let value = op(&mut self.cpu);
            self.memory.set_from_low(self.low, value);
            self.sub_tick = 1;
            return Ok(());
        }
        Err(format!("Illegal sub_tick {} for opcode {:02X}", self.sub_tick, opcode))
    }

    fn zero_page_indexed_addressing_write_x(&mut self, op: fn(&mut Cpu) -> u8, opcode: u8) -> Result<(), String> {
        self.zero_page_indexed_addressing_write(op, self.cpu.x, opcode)
    }

    fn zero_page_indexed_addressing_write_y(&mut self, op: fn(&mut Cpu) -> u8, opcode: u8) -> Result<(), String> {
        self.zero_page_indexed_addressing_write(op, self.cpu.y, opcode)
    }

    fn implied_addressing(&mut self, op: fn(&mut Cpu), opcode: u8) -> Result<(), String> {
        if self.sub_tick == 2 {
            op(&mut self.cpu);
            self.sub_tick = 1;
            return Ok(());
        }
        Err(format!("Illegal sub_tick {} for opcode {:02X}", self.sub_tick, opcode))
    }

    fn absolute_indexed_addressing_read(&mut self, op: fn(&mut Cpu, u8), index: u8, opcode: u8) -> Result<(), String> {
        if self.sub_tick == 2 {
            self.low = self.memory.get_from_word(self.cpu.get_and_increment_pc());
            self.cpu_logger.borrow_mut().operand(self.low);
            self.sub_tick += 1;
            return Ok(());
        }
        if self.sub_tick == 3 {
            (self.low, self.fix_high) = self.low.overflowing_add(index);
            self.high = self.memory.get_from_word(self.cpu.get_and_increment_pc());
            self.cpu_logger.borrow_mut().operand(self.high);
            self.sub_tick += 1;
            return Ok(());
        }
        if self.sub_tick == 4 {
            if self.fix_high {
                self.high += 1;
                self.sub_tick += 1;
            } else {
                let value = self.memory.get_from_low_high(self.low, self.high);
                op(&mut self.cpu, value);
                self.sub_tick = 1;
            }
            return Ok(());
        }
        if self.sub_tick == 5 {
            let value = self.memory.get_from_low_high(self.low, self.high);
            op(&mut self.cpu, value);
            self.sub_tick = 1;
            return Ok(());
        }
        Err(format!("Illegal sub_tick {} for opcode {:02X}", self.sub_tick, opcode))
    }

    fn indirect_indexed_addressing_read(&mut self, op: fn(&mut Cpu, u8), opcode: u8) -> Result<(), String> {
        if self.sub_tick == 2 {
            self.latch = self.memory.get_from_word(self.cpu.get_and_increment_pc());
            self.cpu_logger.borrow_mut().operand(self.latch);
            self.sub_tick += 1;
            return Ok(());
        }
        if self.sub_tick == 3 {
            self.low = self.memory.get_from_low(self.latch);
            self.sub_tick += 1;
            return Ok(());
        }
        if self.sub_tick == 4 {
            self.high = self.memory.get_from_low(self.latch.wrapping_add(1));
            (self.latch, self.fix_high) = self.low.overflowing_add(self.cpu.y);
            self.sub_tick += 1;
            return Ok(());
        }
        if self.sub_tick == 5 {
            if self.fix_high {
                self.high += 1;
                self.sub_tick += 1;
            } else {
                let value = self.memory.get_from_low_high(self.low, self.high);
                op(&mut self.cpu, value);
                self.sub_tick = 1;
            }
            return Ok(());
        }
        if self.sub_tick == 6 {
            let value = self.memory.get_from_low_high(self.low, self.high);
            op(&mut self.cpu, value);
            self.sub_tick = 1;
            return Ok(());
        }
        Err(format!("Illegal sub_tick {} for opcode {:02X}", self.sub_tick, opcode))
    }

    fn indirect_indexed_addressing_write(&mut self, op: fn(&mut Cpu) -> u8, opcode: u8) -> Result<(), String> {
        if self.sub_tick == 2 {
            self.latch = self.memory.get_from_word(self.cpu.get_and_increment_pc());
            self.cpu_logger.borrow_mut().operand(self.latch);
            self.sub_tick += 1;
            return Ok(());
        }
        if self.sub_tick == 3 {
            self.low = self.memory.get_from_low(self.latch);
            self.sub_tick += 1;
            return Ok(());
        }
        if self.sub_tick == 4 {
            self.high = self.memory.get_from_low(self.latch.wrapping_add(1));
            (self.latch, self.fix_high) = self.low.overflowing_add(self.cpu.y);
            self.sub_tick += 1;
            return Ok(());
        }
        if self.sub_tick == 5 {
            if self.fix_high {
                self.high += 1;
            }
            self.sub_tick += 1;
            return Ok(());
        }
        if self.sub_tick == 6 {
            let value = op(&mut self.cpu);
            self.memory.set_from_low_high(self.low, self.high, value);
            self.sub_tick = 1;
            return Ok(());
        }
        Err(format!("Illegal sub_tick {} for opcode {:02X}", self.sub_tick, opcode))
    }

    fn absolute_indexed_addressing_read_x(&mut self, op: fn(&mut Cpu, u8), opcode: u8) -> Result<(), String> {
        self.absolute_indexed_addressing_read(op, self.cpu.x, opcode)
    }

    fn absolute_indexed_addressing_read_y(&mut self, op: fn(&mut Cpu, u8), opcode: u8) -> Result<(), String> {
        self.absolute_indexed_addressing_read(op, self.cpu.y, opcode)
    }

    fn absolute_indexed_addressing_write(&mut self, op: fn(&mut Cpu) -> u8, index: u8, opcode: u8) -> Result<(), String> {
        if self.sub_tick == 2 {
            self.low = self.memory.get_from_word(self.cpu.get_and_increment_pc());
            self.cpu_logger.borrow_mut().operand(self.low);
            self.sub_tick += 1;
            return Ok(());
        }
        if self.sub_tick == 3 {
            (self.low, self.fix_high) = self.low.overflowing_add(index);
            self.high = self.memory.get_from_word(self.cpu.get_and_increment_pc());
            self.cpu_logger.borrow_mut().operand(self.high);
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
            let value = op(&mut self.cpu);
            self.memory.set_from_low_high(self.low, self.high, value);
            self.sub_tick = 1;
            return Ok(());
        }
        Err(format!("Illegal sub_tick {} for opcode {:02X}", self.sub_tick, opcode))
    }

    fn absolute_indexed_addressing_write_x(&mut self, op: fn(&mut Cpu) -> u8, opcode: u8) -> Result<(), String> {
        self.absolute_indexed_addressing_write(op, self.cpu.x, opcode)
    }

    fn absolute_indexed_addressing_write_y(&mut self, op: fn(&mut Cpu) -> u8, opcode: u8) -> Result<(), String> {
        self.absolute_indexed_addressing_write(op, self.cpu.y, opcode)
    }

    fn relative_addressing(&mut self, op: fn(&mut Cpu) -> bool, opcode: u8) -> Result<(), String> {
        if self.sub_tick == 2 {
            self.latch = self.memory.get_from_word(self.cpu.get_and_increment_pc());
            self.cpu_logger.borrow_mut().operand(self.low);
            if op(&mut self.cpu) {
                self.sub_tick += 1;
            } else {
                self.sub_tick = 1;
            }
            return Ok(());
        }
        if self.sub_tick == 3 {
            let (res, fix) = self.cpu.get_pcl().overflowing_add(self.latch);
            if self.latch & 0x80 == 0 {
                if fix {
                    self.high = self.cpu.get_pch() + 1;
                    self.sub_tick += 1;
                } else {
                    self.sub_tick = 1;
                }
            } else {
                let magnitude = !self.latch + 1;
                if magnitude > self.cpu.get_pcl() {
                    self.high = self.cpu.get_pch() - 1;
                    self.sub_tick += 1;
                } else {
                    self.sub_tick = 1;
                }
            }
            self.cpu.set_pcl(res);

            return Ok(());
        }
        if self.sub_tick == 4 {
            self.cpu.set_pc(self.cpu.get_pcl(), self.high);
            self.sub_tick = 1;
            return Ok(());
        }
        Err(format!("Illegal sub_tick {} for opcode {:02X}", self.sub_tick, opcode))
    }

    fn zero_page_addressing_write(&mut self, op: fn(&mut Cpu) -> u8, opcode: u8) -> Result<(), String> {
        if self.sub_tick == 2 {
            self.low = self.memory.get_from_word(self.cpu.get_and_increment_pc());
            self.cpu_logger.borrow_mut().operand(self.low);
            self.sub_tick += 1;
            return Ok(());
        }
        if self.sub_tick == 3 {
            let value = op(&mut self.cpu);
            self.memory.set_from_low(self.low, value);
            self.sub_tick = 1;
            return Ok(());
        }
        Err(format!("Illegal sub_tick {} for opcode {:02X}", self.sub_tick, opcode))
    }
}

#[cfg(test)]
mod tests {
    use crate::emulator::cpu::Cpu;
    use crate::emulator::emulator::Emulator;

    #[test]
    fn relative_addressing() {
        let mut e = Emulator::new();
        e.sub_tick = 3;
        e.latch = (12 as i8) as u8;
        e.cpu.pc = 100;
        e.cpu.set_pch(1);
        e.relative_addressing(|_: &mut Cpu| -> bool{ true }, 0).unwrap();
        assert_eq!(e.cpu.get_pcl(), 112);
        assert_eq!(e.sub_tick, 1);

        e.sub_tick = 3;
        e.latch = (-12 as i8) as u8;
        e.cpu.pc = 100;
        e.cpu.set_pch(1);
        e.relative_addressing(|_: &mut Cpu| -> bool{ true }, 0).unwrap();
        assert_eq!(e.cpu.get_pcl(), 88);
        assert_eq!(e.sub_tick, 1);

        e.sub_tick = 3;
        e.latch = (-12 as i8) as u8;
        e.cpu.pc = 12;
        e.cpu.set_pch(1);
        e.relative_addressing(|_: &mut Cpu| -> bool{ true }, 0).unwrap();
        assert_eq!(e.cpu.get_pcl(), 0);
        assert_eq!(e.sub_tick, 1);

        e.sub_tick = 3;
        e.latch = (-12 as i8) as u8;
        e.cpu.pc = 3;
        e.cpu.set_pch(1);
        e.relative_addressing(|_: &mut Cpu| -> bool{ true }, 0).unwrap();
        assert_eq!(e.cpu.get_pcl(), 247);
        assert_ne!(e.sub_tick, 1);
        assert_eq!(e.high, 0);

        e.sub_tick = 3;
        e.latch = (100 as i8) as u8;
        e.cpu.pc = 200;
        e.cpu.set_pch(1);
        e.relative_addressing(|_: &mut Cpu| -> bool{ true }, 0).unwrap();
        assert_eq!(e.cpu.get_pcl(), 44);
        assert_ne!(e.sub_tick, 1);
        assert_eq!(e.high, 2);
    }
}