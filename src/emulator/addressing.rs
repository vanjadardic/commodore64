use crate::emulator::cpu::Cpu;
use crate::emulator::memory::Memory;

enum AddressingType {
    Implied,
    AbsoluteJSR,
    AbsoluteIndexedReadX,
    AbsoluteIndexedReadY,
    Relative,
    ImpliedRTS,
    Immediate,
    AbsoluteWrite,
    ZeroPageWrite,
    AbsoluteRead,
    AbsoluteJMP,
    AbsoluteIndexedWriteX,
    AbsoluteIndexedWriteY,
    ZeroPageReadModifyWrite,
    IndirectIndexedRead,
    IndirectIndexedWrite,
    Accumulator,
    ZeroPageRead,
    ZeroPageIndexedWriteX,
    // ZeroPageIndexedWriteY,
    ZeroPageIndexedReadX,
    // ZeroPageIndexedReadY,
    ZeroPageIndexedReadModifyWriteX,
    // ZeroPageIndexedReadModifyWriteY,
    AbsoluteIndirectJMP,
    ImpliedPHA,
    ImpliedPLA,
}

pub struct Addressing {
    addressing_type: AddressingType,
    operand1: Option<u8>,
    operand2: Option<u8>,
    low: u8,
    high: u8,
    fix_high: bool,
    latch: u8,
}

impl Addressing {
    pub fn new() -> Self {
        Self {
            addressing_type: AddressingType::Immediate,
            operand1: None,
            operand2: None,
            low: 0,
            high: 0,
            fix_high: false,
            latch: 0,
        }
    }

    fn set_addressing_type(&mut self, addressing_type: AddressingType) {
        self.addressing_type = addressing_type;
        self.operand1 = None;
        self.operand2 = None;
    }

    fn read_operand(&mut self, memory: &Memory, cpu: &mut Cpu) -> u8 {
        let value = memory.get_from_word(cpu.get_and_increment_pc());
        if self.operand1 == None {
            self.operand1 = Some(value);
        } else {
            self.operand2 = Some(value);
        }
        value
    }

    pub fn operand1(&self) -> Option<u8> {
        self.operand1
    }

    pub fn operand2(&self) -> Option<u8> {
        self.operand2
    }

    pub fn description(&self) -> String {
        match self.addressing_type {
            AddressingType::Immediate => {
                format!("#${:02X}", self.operand1.unwrap())
            }
            AddressingType::Implied => {
                String::from("")
            }
            AddressingType::AbsoluteJSR => {
                format!("${:02X}{:02X}", self.operand2.unwrap(), self.operand1.unwrap())
            }
            AddressingType::AbsoluteIndexedReadX => {
                format!("${:02X}{:02X},X", self.operand2.unwrap(), self.operand1.unwrap())
            }
            AddressingType::AbsoluteIndexedReadY => {
                format!("${:02X}{:02X},Y", self.operand2.unwrap(), self.operand1.unwrap())
            }
            AddressingType::Relative => {
                format!("${:02X}{:02X}", self.high, self.low)
            }
            AddressingType::ImpliedRTS => {
                String::from("")
            }
            AddressingType::AbsoluteWrite => {
                format!("${:02X}{:02X}", self.operand2.unwrap(), self.operand1.unwrap())
            }
            AddressingType::ZeroPageWrite => {
                format!("#${:02X}", self.operand1.unwrap())
            }
            AddressingType::AbsoluteRead => {
                format!("${:02X}{:02X}", self.operand2.unwrap(), self.operand1.unwrap())
            }
            AddressingType::AbsoluteJMP => {
                format!("${:02X}{:02X}", self.operand2.unwrap(), self.operand1.unwrap())
            }
            AddressingType::AbsoluteIndexedWriteX => {
                format!("${:02X}{:02X},X", self.operand2.unwrap(), self.operand1.unwrap())
            }
            AddressingType::AbsoluteIndexedWriteY => {
                format!("${:02X}{:02X},Y", self.operand2.unwrap(), self.operand1.unwrap())
            }
            AddressingType::ZeroPageReadModifyWrite => {
                format!("${:02X}", self.operand1.unwrap())
            }
            AddressingType::IndirectIndexedRead => {
                format!("(${:02X}),Y", self.operand1.unwrap())
            }
            AddressingType::IndirectIndexedWrite => {
                format!("(${:02X}),Y", self.operand1.unwrap())
            }
            AddressingType::Accumulator => {
                String::from("")
            }
            AddressingType::ZeroPageRead => {
                format!("#${:02X}", self.operand1.unwrap())
            }
            AddressingType::ZeroPageIndexedWriteX => {
                format!("${:02X},X", self.operand1.unwrap())
            }
            // AddressingType::ZeroPageIndexedWriteY => {
            //     format!("${:02X},Y", self.operand1.unwrap())
            // }
            AddressingType::ZeroPageIndexedReadX => {
                format!("${:02X},X", self.operand1.unwrap())
            }
            // AddressingType::ZeroPageIndexedReadY => {
            //     format!("${:02X},Y", self.operand1.unwrap())
            // }
            AddressingType::AbsoluteIndirectJMP => {
                format!("(${:02X}{:02X})", self.operand2.unwrap(), self.operand1.unwrap())
            }
            AddressingType::ImpliedPHA => {
                String::from("")
            }
            AddressingType::ImpliedPLA => {
                String::from("")
            }
            AddressingType::ZeroPageIndexedReadModifyWriteX => {
                format!("${:02X},X", self.operand1.unwrap())
            }
            // AddressingType::ZeroPageIndexedReadModifyWriteY => {
            //     format!("${:02X},Y", self.operand1.unwrap())
            // }
        }
    }

    pub fn immediate(&mut self, sub_tick: u8, cpu: &mut Cpu, memory: &Memory, inst: fn(&mut Cpu, u8), opcode: u8) -> Result<u8, String> {
        if sub_tick == 2 {
            self.set_addressing_type(AddressingType::Immediate);
            let value = self.read_operand(memory, cpu);
            inst(cpu, value);
            return Ok(1);
        }
        Err(format!("Illegal sub_tick {} for opcode {:02X}", sub_tick, opcode))
    }

    pub fn implied(&mut self, sub_tick: u8, cpu: &mut Cpu, inst: fn(&mut Cpu), opcode: u8) -> Result<u8, String> {
        if sub_tick == 2 {
            self.set_addressing_type(AddressingType::Implied);
            inst(cpu);
            return Ok(1);
        }
        Err(format!("Illegal sub_tick {} for opcode {:02X}", sub_tick, opcode))
    }

    pub fn absolute_jsr(&mut self, sub_tick: u8, cpu: &mut Cpu, memory: &mut Memory, opcode: u8) -> Result<u8, String> {
        if sub_tick == 2 {
            self.set_addressing_type(AddressingType::AbsoluteJSR);
            self.low = self.read_operand(memory, cpu);
            return Ok(sub_tick + 1);
        }
        if sub_tick == 3 {
            return Ok(sub_tick + 1);
        }
        if sub_tick == 4 {
            memory.set_stack(cpu.sp, cpu.get_pch());
            cpu.sp = cpu.sp.wrapping_sub(1);
            return Ok(sub_tick + 1);
        }
        if sub_tick == 5 {
            memory.set_stack(cpu.sp, cpu.get_pcl());
            cpu.sp = cpu.sp.wrapping_sub(1);
            return Ok(sub_tick + 1);
        }
        if sub_tick == 6 {
            self.high = self.read_operand(memory, cpu);
            cpu.set_pc(self.low, self.high);
            cpu.inst = "JSR";
            return Ok(1);
        }
        Err(format!("Illegal sub_tick {} for opcode {:02X}", sub_tick, opcode))
    }

    fn absolute_indexed_read(&mut self, sub_tick: u8, cpu: &mut Cpu, memory: &Memory, inst: fn(&mut Cpu, u8), index: u8, opcode: u8) -> Result<u8, String> {
        if sub_tick == 2 {
            self.low = self.read_operand(memory, cpu);
            return Ok(sub_tick + 1);
        }
        if sub_tick == 3 {
            (self.low, self.fix_high) = self.low.overflowing_add(index);
            self.high = self.read_operand(memory, cpu);
            return Ok(sub_tick + 1);
        }
        if sub_tick == 4 {
            return if self.fix_high {
                self.high += 1;
                Ok(sub_tick + 1)
            } else {
                inst(cpu, memory.get_from_low_high(self.low, self.high));
                Ok(1)
            };
        }
        if sub_tick == 5 {
            inst(cpu, memory.get_from_low_high(self.low, self.high));
            return Ok(1);
        }
        Err(format!("Illegal sub_tick {} for opcode {:02X}", sub_tick, opcode))
    }

    pub fn absolute_indexed_read_x(&mut self, sub_tick: u8, cpu: &mut Cpu, memory: &Memory, inst: fn(&mut Cpu, u8), opcode: u8) -> Result<u8, String> {
        if sub_tick == 2 {
            self.set_addressing_type(AddressingType::AbsoluteIndexedReadX);
        }
        self.absolute_indexed_read(sub_tick, cpu, memory, inst, cpu.x, opcode)
    }

    pub fn absolute_indexed_read_y(&mut self, sub_tick: u8, cpu: &mut Cpu, memory: &Memory, inst: fn(&mut Cpu, u8), opcode: u8) -> Result<u8, String> {
        if sub_tick == 2 {
            self.set_addressing_type(AddressingType::AbsoluteIndexedReadY);
        }
        self.absolute_indexed_read(sub_tick, cpu, memory, inst, cpu.y, opcode)
    }

    pub fn relative(&mut self, sub_tick: u8, cpu: &mut Cpu, memory: &Memory, inst: fn(&mut Cpu) -> bool, opcode: u8) -> Result<u8, String> {
        if sub_tick == 2 {
            self.set_addressing_type(AddressingType::Relative);
            self.latch = self.read_operand(memory, cpu);
            return if inst(cpu) {
                Ok(sub_tick + 1)
            } else {
                let (r, fix) = cpu.get_pcl().overflowing_add(self.latch);
                self.high = cpu.get_pch().checked_add_signed(self.relative_fix_high(fix, cpu.get_pcl())).unwrap();
                self.low = r;
                Ok(1)
            };
        }
        if sub_tick == 3 {
            let (r, fix) = cpu.get_pcl().overflowing_add(self.latch);
            let fix_high = self.relative_fix_high(fix, cpu.get_pcl());
            self.high = cpu.get_pch().checked_add_signed(fix_high).unwrap();
            self.low = r;
            cpu.set_pcl(self.low);
            return if fix_high != 0 {
                Ok(sub_tick + 1)
            } else {
                Ok(1)
            };
        }
        if sub_tick == 4 {
            cpu.set_pch(self.high);
            return Ok(1);
        }
        Err(format!("Illegal sub_tick {} for opcode {:02X}", sub_tick, opcode))
    }

    fn relative_fix_high(&self, fix: bool, low: u8) -> i8 {
        if self.latch & 0x80 == 0 {
            if fix { 1 } else { 0 }
        } else {
            let magnitude = !self.latch + 1;
            if magnitude > low { -1 } else { 0 }
        }
    }

    pub fn implied_rts(&mut self, sub_tick: u8, cpu: &mut Cpu, memory: &Memory, opcode: u8) -> Result<u8, String> {
        if sub_tick == 2 {
            self.set_addressing_type(AddressingType::ImpliedRTS);
            return Ok(sub_tick + 1);
        }
        if sub_tick == 3 {
            cpu.sp = cpu.sp.wrapping_add(1);
            return Ok(sub_tick + 1);
        }
        if sub_tick == 4 {
            cpu.set_pcl(memory.get_stack(cpu.sp));
            cpu.sp = cpu.sp.wrapping_add(1);
            return Ok(sub_tick + 1);
        }
        if sub_tick == 5 {
            cpu.set_pch(memory.get_stack(cpu.sp));
            return Ok(sub_tick + 1);
        }
        if sub_tick == 6 {
            cpu.get_and_increment_pc();
            cpu.inst = "RTS";
            return Ok(1);
        }
        Err(format!("Illegal sub_tick {} for opcode {:02X}", sub_tick, opcode))
    }

    pub fn absolute_write(&mut self, sub_tick: u8, cpu: &mut Cpu, memory: &mut Memory, inst: fn(&mut Cpu) -> u8, opcode: u8) -> Result<u8, String> {
        if sub_tick == 2 {
            self.set_addressing_type(AddressingType::AbsoluteWrite);
            self.low = self.read_operand(memory, cpu);
            return Ok(sub_tick + 1);
        }
        if sub_tick == 3 {
            self.high = self.read_operand(memory, cpu);
            return Ok(sub_tick + 1);
        }
        if sub_tick == 4 {
            memory.set_from_low_high(self.low, self.high, inst(cpu));
            return Ok(1);
        }
        Err(format!("Illegal sub_tick {} for opcode {:02X}", sub_tick, opcode))
    }

    pub fn zero_page_write(&mut self, sub_tick: u8, cpu: &mut Cpu, memory: &mut Memory, inst: fn(&mut Cpu) -> u8, opcode: u8) -> Result<u8, String> {
        if sub_tick == 2 {
            self.set_addressing_type(AddressingType::ZeroPageWrite);
            self.low = self.read_operand(memory, cpu);
            return Ok(sub_tick + 1);
        }
        if sub_tick == 3 {
            memory.set_from_low(self.low, inst(cpu));
            return Ok(1);
        }
        Err(format!("Illegal sub_tick {} for opcode {:02X}", sub_tick, opcode))
    }

    pub fn absolute_read(&mut self, sub_tick: u8, cpu: &mut Cpu, memory: &Memory, inst: fn(&mut Cpu, u8), opcode: u8) -> Result<u8, String> {
        if sub_tick == 2 {
            self.set_addressing_type(AddressingType::AbsoluteRead);
            self.low = self.read_operand(memory, cpu);
            return Ok(sub_tick + 1);
        }
        if sub_tick == 3 {
            self.high = self.read_operand(memory, cpu);
            return Ok(sub_tick + 1);
        }
        if sub_tick == 4 {
            inst(cpu, memory.get_from_low_high(self.low, self.high));
            return Ok(1);
        }
        Err(format!("Illegal sub_tick {} for opcode {:02X}", sub_tick, opcode))
    }

    pub fn absolute_jmp(&mut self, sub_tick: u8, cpu: &mut Cpu, memory: &Memory, opcode: u8) -> Result<u8, String> {
        if sub_tick == 2 {
            self.set_addressing_type(AddressingType::AbsoluteJMP);
            self.low = self.read_operand(memory, cpu);
            return Ok(sub_tick + 1);
        }
        if sub_tick == 3 {
            self.high = self.read_operand(memory, cpu);
            cpu.set_pc(self.low, self.high);
            cpu.inst = "JMP";
            return Ok(1);
        }
        Err(format!("Illegal sub_tick {} for opcode {:02X}", sub_tick, opcode))
    }

    fn absolute_indexed_write(&mut self, sub_tick: u8, cpu: &mut Cpu, memory: &mut Memory, inst: fn(&mut Cpu) -> u8, index: u8, opcode: u8) -> Result<u8, String> {
        if sub_tick == 2 {
            self.low = self.read_operand(memory, cpu);
            return Ok(sub_tick + 1);
        }
        if sub_tick == 3 {
            (self.low, self.fix_high) = self.low.overflowing_add(index);
            self.high = self.read_operand(memory, cpu);
            return Ok(sub_tick + 1);
        }
        if sub_tick == 4 {
            if self.fix_high {
                self.high += 1;
            }
            return Ok(sub_tick + 1);
        }
        if sub_tick == 5 {
            memory.set_from_low_high(self.low, self.high, inst(cpu));
            return Ok(1);
        }
        Err(format!("Illegal sub_tick {} for opcode {:02X}", sub_tick, opcode))
    }

    pub fn absolute_indexed_write_x(&mut self, sub_tick: u8, cpu: &mut Cpu, memory: &mut Memory, inst: fn(&mut Cpu) -> u8, opcode: u8) -> Result<u8, String> {
        if sub_tick == 2 {
            self.set_addressing_type(AddressingType::AbsoluteIndexedWriteX);
        }
        self.absolute_indexed_write(sub_tick, cpu, memory, inst, cpu.x, opcode)
    }

    pub fn absolute_indexed_write_y(&mut self, sub_tick: u8, cpu: &mut Cpu, memory: &mut Memory, inst: fn(&mut Cpu) -> u8, opcode: u8) -> Result<u8, String> {
        if sub_tick == 2 {
            self.set_addressing_type(AddressingType::AbsoluteIndexedWriteY);
        }
        self.absolute_indexed_write(sub_tick, cpu, memory, inst, cpu.y, opcode)
    }

    pub fn zero_page_read_modify_write(&mut self, sub_tick: u8, cpu: &mut Cpu, memory: &mut Memory, inst: fn(&mut Cpu, u8) -> u8, opcode: u8) -> Result<u8, String> {
        if sub_tick == 2 {
            self.set_addressing_type(AddressingType::ZeroPageReadModifyWrite);
            self.low = self.read_operand(memory, cpu);
            return Ok(sub_tick + 1);
        }
        if sub_tick == 3 {
            self.latch = memory.get_from_low(self.low);
            return Ok(sub_tick + 1);
        }
        if sub_tick == 4 {
            self.latch = inst(cpu, self.latch);
            return Ok(sub_tick + 1);
        }
        if sub_tick == 5 {
            memory.set_from_low(self.low, self.latch);
            return Ok(1);
        }
        Err(format!("Illegal sub_tick {} for opcode {:02X}", sub_tick, opcode))
    }

    pub fn indirect_indexed_read(&mut self, sub_tick: u8, cpu: &mut Cpu, memory: &Memory, inst: fn(&mut Cpu, u8), opcode: u8) -> Result<u8, String> {
        if sub_tick == 2 {
            self.set_addressing_type(AddressingType::IndirectIndexedRead);
            self.latch = self.read_operand(memory, cpu);
            return Ok(sub_tick + 1);
        }
        if sub_tick == 3 {
            self.low = memory.get_from_low(self.latch);
            return Ok(sub_tick + 1);
        }
        if sub_tick == 4 {
            self.high = memory.get_from_low(self.latch.wrapping_add(1));
            (self.low, self.fix_high) = self.low.overflowing_add(cpu.y);
            return Ok(sub_tick + 1);
        }
        if sub_tick == 5 {
            return if self.fix_high {
                self.high += 1;
                Ok(sub_tick + 1)
            } else {
                inst(cpu, memory.get_from_low_high(self.low, self.high));
                Ok(1)
            };
        }
        if sub_tick == 6 {
            inst(cpu, memory.get_from_low_high(self.low, self.high));
            return Ok(1);
        }
        Err(format!("Illegal sub_tick {} for opcode {:02X}", sub_tick, opcode))
    }

    pub fn indirect_indexed_write(&mut self, sub_tick: u8, cpu: &mut Cpu, memory: &mut Memory, inst: fn(&mut Cpu) -> u8, opcode: u8) -> Result<u8, String> {
        if sub_tick == 2 {
            self.set_addressing_type(AddressingType::IndirectIndexedWrite);
            self.latch = self.read_operand(memory, cpu);
            return Ok(sub_tick + 1);
        }
        if sub_tick == 3 {
            self.low = memory.get_from_low(self.latch);
            return Ok(sub_tick + 1);
        }
        if sub_tick == 4 {
            self.high = memory.get_from_low(self.latch.wrapping_add(1));
            (self.low, self.fix_high) = self.low.overflowing_add(cpu.y);
            return Ok(sub_tick + 1);
        }
        if sub_tick == 5 {
            if self.fix_high {
                self.high += 1;
            }
            return Ok(sub_tick + 1);
        }
        if sub_tick == 6 {
            memory.set_from_low_high(self.low, self.high, inst(cpu));
            return Ok(1);
        }
        Err(format!("Illegal sub_tick {} for opcode {:02X}", sub_tick, opcode))
    }

    pub fn accumulator(&mut self, sub_tick: u8, cpu: &mut Cpu, inst: fn(&mut Cpu, u8) -> u8, opcode: u8) -> Result<u8, String> {
        if sub_tick == 2 {
            self.set_addressing_type(AddressingType::Accumulator);
            cpu.a = inst(cpu, cpu.a);
            return Ok(1);
        }
        Err(format!("Illegal sub_tick {} for opcode {:02X}", sub_tick, opcode))
    }

    pub fn zero_page_read(&mut self, sub_tick: u8, cpu: &mut Cpu, memory: &Memory, inst: fn(&mut Cpu, u8), opcode: u8) -> Result<u8, String> {
        if sub_tick == 2 {
            self.set_addressing_type(AddressingType::ZeroPageRead);
            self.low = self.read_operand(memory, cpu);
            return Ok(sub_tick + 1);
        }
        if sub_tick == 3 {
            inst(cpu, memory.get_from_low(self.low));
            return Ok(1);
        }
        Err(format!("Illegal sub_tick {} for opcode {:02X}", sub_tick, opcode))
    }

    fn zero_page_indexed_write(&mut self, sub_tick: u8, cpu: &mut Cpu, memory: &mut Memory, inst: fn(&mut Cpu) -> u8, index: u8, opcode: u8) -> Result<u8, String> {
        if sub_tick == 2 {
            self.low = self.read_operand(memory, cpu);
            return Ok(sub_tick + 1);
        }
        if sub_tick == 3 {
            self.low = self.low.wrapping_add(index);
            return Ok(sub_tick + 1);
        }
        if sub_tick == 4 {
            memory.set_from_low(self.low, inst(cpu));
            return Ok(1);
        }
        Err(format!("Illegal sub_tick {} for opcode {:02X}", sub_tick, opcode))
    }

    pub fn zero_page_indexed_write_x(&mut self, sub_tick: u8, cpu: &mut Cpu, memory: &mut Memory, inst: fn(&mut Cpu) -> u8, opcode: u8) -> Result<u8, String> {
        if sub_tick == 2 {
            self.set_addressing_type(AddressingType::ZeroPageIndexedWriteX);
        }
        self.zero_page_indexed_write(sub_tick, cpu, memory, inst, cpu.x, opcode)
    }

    // pub fn zero_page_indexed_write_y(&mut self, sub_tick: u8, cpu: &mut Cpu, memory: &mut Memory, inst: fn(&mut Cpu) -> u8, opcode: u8) -> Result<u8, String> {
    //     if sub_tick == 2 {
    //         self.set_addressing_type(AddressingType::ZeroPageIndexedWriteY);
    //     }
    //     self.zero_page_indexed_write(sub_tick, cpu, memory, inst, cpu.y, opcode)
    // }

    fn zero_page_indexed_read(&mut self, sub_tick: u8, cpu: &mut Cpu, memory: &Memory, inst: fn(&mut Cpu, u8), index: u8, opcode: u8) -> Result<u8, String> {
        if sub_tick == 2 {
            self.low = self.read_operand(memory, cpu);
            return Ok(sub_tick + 1);
        }
        if sub_tick == 3 {
            self.low = self.low.wrapping_add(index);
            return Ok(sub_tick + 1);
        }
        if sub_tick == 4 {
            inst(cpu, memory.get_from_low(self.low));
            return Ok(1);
        }
        Err(format!("Illegal sub_tick {} for opcode {:02X}", sub_tick, opcode))
    }

    pub fn zero_page_indexed_read_x(&mut self, sub_tick: u8, cpu: &mut Cpu, memory: &Memory, inst: fn(&mut Cpu, u8), opcode: u8) -> Result<u8, String> {
        if sub_tick == 2 {
            self.set_addressing_type(AddressingType::ZeroPageIndexedReadX);
        }
        self.zero_page_indexed_read(sub_tick, cpu, memory, inst, cpu.x, opcode)
    }

    // pub fn zero_page_indexed_read_y(&mut self, sub_tick: u8, cpu: &mut Cpu, memory: &Memory, inst: fn(&mut Cpu, u8), opcode: u8) -> Result<u8, String> {
    //     if sub_tick == 2 {
    //         self.set_addressing_type(AddressingType::ZeroPageIndexedReadY);
    //     }
    //     self.zero_page_indexed_read(sub_tick, cpu, memory, inst, cpu.y, opcode)
    // }

    fn zero_page_indexed_read_modify_write(&mut self, sub_tick: u8, cpu: &mut Cpu, memory: &mut Memory, inst: fn(&mut Cpu, u8) -> u8, index: u8, opcode: u8) -> Result<u8, String> {
        if sub_tick == 2 {
            self.low = self.read_operand(memory, cpu);
            return Ok(sub_tick + 1);
        }
        if sub_tick == 3 {
            self.low = self.low.wrapping_add(index);
            return Ok(sub_tick + 1);
        }
        if sub_tick == 4 {
            return Ok(sub_tick + 1);
        }
        if sub_tick == 5 {
            return Ok(sub_tick + 1);
        }
        if sub_tick == 6 {
            let value = inst(cpu, memory.get_from_low(self.low));
            memory.set_from_low(self.low, value);
            return Ok(1);
        }
        Err(format!("Illegal sub_tick {} for opcode {:02X}", sub_tick, opcode))
    }

    pub fn zero_page_indexed_read_modify_write_x(&mut self, sub_tick: u8, cpu: &mut Cpu, memory: &mut Memory, inst: fn(&mut Cpu, u8) -> u8, opcode: u8) -> Result<u8, String> {
        if sub_tick == 2 {
            self.set_addressing_type(AddressingType::ZeroPageIndexedReadModifyWriteX);
        }
        self.zero_page_indexed_read_modify_write(sub_tick, cpu, memory, inst, cpu.x, opcode)
    }

    // pub fn zero_page_indexed_read_modify_write_y(&mut self, sub_tick: u8, cpu: &mut Cpu, memory: &mut Memory, inst: fn(&mut Cpu, u8)->u8, opcode: u8) -> Result<u8, String> {
    //     if sub_tick == 2 {
    //         self.set_addressing_type(AddressingType::ZeroPageIndexedReadModifyWriteY);
    //     }
    //     self.zero_page_indexed_read_modify_write(sub_tick, cpu, memory, inst, cpu.y, opcode)
    // }

    pub fn absolute_indirect_jmp(&mut self, sub_tick: u8, cpu: &mut Cpu, memory: &Memory, opcode: u8) -> Result<u8, String> {
        if sub_tick == 2 {
            self.set_addressing_type(AddressingType::AbsoluteIndirectJMP);
            self.low = self.read_operand(memory, cpu);
            return Ok(sub_tick + 1);
        }
        if sub_tick == 3 {
            self.high = self.read_operand(memory, cpu);
            cpu.set_pc(self.low, self.high);
            return Ok(sub_tick + 1);
        }
        if sub_tick == 4 {
            self.latch = memory.get_from_low_high(self.low, self.high);
            return Ok(sub_tick + 1);
        }
        if sub_tick == 5 {
            cpu.set_pc(self.latch, memory.get_from_low_high(self.low.wrapping_add(1), self.high));
            cpu.inst = "JMP";
            return Ok(1);
        }
        Err(format!("Illegal sub_tick {} for opcode {:02X}", sub_tick, opcode))
    }

    pub fn implied_php_pha(&mut self, sub_tick: u8, cpu: &mut Cpu, memory: &mut Memory, inst: fn(&mut Cpu) -> u8, opcode: u8) -> Result<u8, String> {
        if sub_tick == 2 {
            self.set_addressing_type(AddressingType::ImpliedPHA);
            return Ok(sub_tick + 1);
        }
        if sub_tick == 3 {
            memory.set_stack(cpu.sp, inst(cpu));
            cpu.sp = cpu.sp.wrapping_sub(1);
            return Ok(1);
        }
        Err(format!("Illegal sub_tick {} for opcode {:02X}", sub_tick, opcode))
    }

    pub fn implied_plp_pla(&mut self, sub_tick: u8, cpu: &mut Cpu, memory: &Memory, inst: fn(&mut Cpu, u8), opcode: u8) -> Result<u8, String> {
        if sub_tick == 2 {
            self.set_addressing_type(AddressingType::ImpliedPLA);
            return Ok(sub_tick + 1);
        }
        if sub_tick == 3 {
            cpu.sp = cpu.sp.wrapping_add(1);
            return Ok(sub_tick + 1);
        }
        if sub_tick == 4 {
            inst(cpu, memory.get_stack(cpu.sp));
            return Ok(1);
        }
        Err(format!("Illegal sub_tick {} for opcode {:02X}", sub_tick, opcode))
    }
}