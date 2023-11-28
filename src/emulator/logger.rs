use log::debug;

use crate::emulator::cpu::Cpu;

#[cfg(debug_assertions)]
pub struct CpuLogger {
    tick: u64,
    pc: u16,
    sp: u8,
    a: u8,
    x: u8,
    y: u8,
    opcode: u8,
    operand1: Option<u8>,
    operand2: Option<u8>,
}

#[cfg(debug_assertions)]
impl CpuLogger {
    pub fn new() -> CpuLogger {
        CpuLogger {
            tick: 0,
            pc: 0,
            sp: 0,
            a: 0,
            x: 0,
            y: 0,
            opcode: 0,
            operand1: None,
            operand2: None,
        }
    }

    pub fn init(&mut self, tick: u64, cpu: &Cpu) {
        self.tick = tick;
        self.pc = cpu.pc;
        self.sp = cpu.sp;
        self.a = cpu.a;
        self.x = cpu.x;
        self.y = cpu.y;
        self.operand1 = None;
        self.operand2 = None;
    }

    pub fn opcode(&mut self, opcode: u8) {
        self.opcode = opcode;
    }

    pub fn operand(&mut self, op: u8) {
        if self.operand1 == None {
            self.operand1 = Some(op);
        } else {
            self.operand2 = Some(op);
        }
    }

    pub fn instruction(&mut self, inst: &'static str) {
        let mut line = String::new();
        line.push_str(format!("({:4})[a={:3}, x={:3}, y={:3}] {:04X}   {:02X}", self.tick, self.a, self.x, self.y, self.pc, self.opcode).as_str());
        line.push_str(match self.operand1 {
            Some(x) => format!(" {:02X}", x),
            None => "   ".to_string()
        }.as_str());
        line.push_str(match self.operand2 {
            Some(x) => format!(" {:02X}", x),
            None => "   ".to_string()
        }.as_str());
        line.push_str(format!("   {}", inst).as_str());
        if (self.pc >= 0xFD50 && self.pc <= 0xFD9A) || //initialise memory pointers
            (self.pc >= 0xFD1A && self.pc <= 0xFD2F) { //set I/O vectors depending on XY
            return;
        }


        debug!("{}", line);
    }
}

#[cfg(not(debug_assertions))]
impl CpuLogger {
    pub fn new() -> CpuLogger {
        CpuLogger {
            line: String::with_capacity(0)
        }
    }

    #[inline(always)]
    pub fn start(&mut self, arguments: Arguments) {}

    #[inline(always)]
    pub fn add(&mut self, arguments: Arguments) {}

    #[inline(always)]
    pub fn finish(&mut self, arguments: Arguments) {}
}

