use log::debug;

use crate::emulator::addressing::Addressing;
use crate::emulator::cpu::Cpu;

pub struct CpuLogger {
    tick: u64,
    pc: u16,
    sp: u8,
    p: u8,
    a: u8,
    x: u8,
    y: u8,
    opcode: u8,
    disabled: bool,
    disabled_until: u16,
}

#[cfg(debug_assertions)]
impl CpuLogger {
    pub fn new() -> CpuLogger {
        CpuLogger {
            tick: 0,
            pc: 0,
            sp: 0,
            p: 0,
            a: 0,
            x: 0,
            y: 0,
            opcode: 0,
            disabled: false,
            disabled_until: 0,
        }
    }

    pub fn init(&mut self, tick: u64, cpu: &Cpu) {
        self.tick = tick;
        self.pc = cpu.pc;
        self.sp = cpu.sp;
        self.p = cpu.p;
        self.a = cpu.a;
        self.x = cpu.x;
        self.y = cpu.y;
    }

    pub fn opcode(&mut self, opcode: u8) {
        self.opcode = opcode;
    }

    pub fn log(&mut self, cpu: &Cpu, addressing: &Addressing) {
        let mut line = String::new();
        // line.push_str(format!("({:4})[sp={:02X},p={:02X},a={:02X},x={:02X},y={:02X}] {:04X}   {:02X}", self.tick, self.sp, self.p, self.a, self.x, self.y, self.pc, self.opcode).as_str());
        line.push_str(format!("{:04X}   {:02X}", self.pc, self.opcode).as_str());
        line.push_str(match addressing.operand1() {
            Some(x) => format!(" {:02X}", x),
            None => "   ".to_string()
        }.as_str());
        line.push_str(match addressing.operand2() {
            Some(x) => format!(" {:02X}", x),
            None => "   ".to_string()
        }.as_str());
        line.push_str(format!("   {} {}", cpu.inst, addressing.description()).as_str());
        if self.pc == 0xFD50 { //initialise memory pointers
            self.disabled = true;
            self.disabled_until = 0xFD9A;
            debug!("{:04X} . {:04X} skip '{}'", self.pc,self.disabled_until ,"initialise memory pointers");
        }
        if self.pc == 0xFD1A { //set I/O vectors depending on XY
            self.disabled = true;
            self.disabled_until = 0xFD2F;
            debug!("{:04X} . {:04X} skip '{}'", self.pc,self.disabled_until ,"set I/O vectors depending on XY");
        }
        if self.pc == 0xE518 { //initialise screen and keyboard
            self.disabled = true;
            self.disabled_until = 0xE598;
            debug!("{:04X} . {:04X} skip '{}'", self.pc,self.disabled_until ,"initialise screen and keyboard");
        }
        // if self.pc == 0xFF5E { //some loop
        //     self.disabled = true;
        //     self.disabled_until = 0xFF61;
        // }

        if !self.disabled {
            debug!("{}", line);
        }

        if self.disabled && self.pc == self.disabled_until {
            self.disabled = false;
            debug!("            end skip");
        }
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

