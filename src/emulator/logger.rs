use log::debug;

use crate::emulator::addressing::Addressing;
use crate::emulator::cpu::Cpu;

pub struct CpuLogger {
    enabled: bool,
    tick: u64,
    tick_tmp: u64,
    pc: u16,
    sp: u8,
    p: u8,
    a: u8,
    x: u8,
    y: u8,
    opcode: u8,
    disabled: bool,
    disabled_until: u16,
    cnt: usize,
    interrupted: bool,
}

#[cfg(debug_assertions)]
impl CpuLogger {
    pub fn new() -> CpuLogger {
        CpuLogger {
            enabled: false,
            tick: 0,
            tick_tmp: 0,
            pc: 0,
            sp: 0,
            p: 0,
            a: 0,
            x: 0,
            y: 0,
            opcode: 0,
            disabled: false,
            disabled_until: 0,
            cnt: 0,
            interrupted: false,
        }
    }

    pub fn set_tick(&mut self, tick: u64) {
        self.tick_tmp = tick;
    }

    pub fn init(&mut self, cpu: &Cpu) {
        self.tick = self.tick_tmp;
        self.pc = cpu.pc;
        self.sp = cpu.sp;
        self.p = cpu.p;
        self.a = cpu.a;
        self.x = cpu.x;
        self.y = cpu.y;
        // self.interrupted |= cpu.interrupted;
    }

    pub fn opcode(&mut self, opcode: u8) {
        self.opcode = opcode;
    }

    pub fn log(&mut self, cpu: &Cpu, addressing: &Addressing) {
        if !self.enabled {
            return;
        }
        let mut line = String::new();
        line.push_str(format!("({:4})[sp={:02X},p={:02X},a={:02X},x={:02X},y={:02X}] {:04X}   {:02X}", self.tick, self.sp, self.p, self.a, self.x, self.y, self.pc, self.opcode).as_str());
        // line.push_str(format!("{:04X}   {:02X}", self.pc, self.opcode).as_str());
        line.push_str(match addressing.operand1() {
            Some(x) => format!(" {:02X}", x),
            None => "   ".to_string()
        }.as_str());
        line.push_str(match addressing.operand2() {
            Some(x) => format!(" {:02X}", x),
            None => "   ".to_string()
        }.as_str());
        line.push_str(format!("   {} {}", cpu.inst, addressing.description()).as_str());

        if self.disabled && self.pc == self.disabled_until {
            self.disabled = false;
            debug!("            end skip");
        }


        if self.pc == 0xFD50 && !self.disabled{
            self.disabled = true;
            self.disabled_until = 0xFCF8;
            debug!("{:04X} . {:04X} skip '{}'", self.pc, self.disabled_until, "initialise memory pointers");
        }
        // if self.pc == 0xFD15 {
        //     self.disabled = true;
        //     self.disabled_until = 0xFCFB;
        //     debug!("{:04X} . {:04X} skip '{}'", self.pc, self.disabled_until, "restore I/O vectors");
        // }
        if self.pc == 0xFF5B && !self.disabled{
            // self.cnt = 1;
                self.disabled = true;
                self.disabled_until = 0xFCFE;
                debug!("{:04X} . {:04X} skip '{}'", self.pc, self.disabled_until, "initialise screen and keyboard");
        }
        if self.pc == 0xAB47 && !self.disabled{
            //self.cnt = 1;
            self.disabled = true;
            self.disabled_until = 0xAB4C;
            debug!("{:04X} . {:04X} skip '{}'", self.pc, self.disabled_until, "*** print character");
        }

        if self.pc == 0xAB21 && !self.disabled{
            //self.cnt = 1;
            self.disabled = true;
            self.disabled_until = 0xAAE7;
            debug!("{:04X} . {:04X} skip '{}'", self.pc, self.disabled_until, "*** print string from utility pointer");
        }
        if self.pc == 0xFCF2 && !self.disabled{
            //self.cnt = 1;
            self.disabled = true;
            self.disabled_until = 0xFCF5;
            debug!("{:04X} . {:04X} skip '{}'", self.pc, self.disabled_until, "initialise SID, CIA and IRQ");
        }
        if self.pc == 0xFCF5 && !self.disabled{
            //self.cnt = 1;
            self.disabled = true;
            self.disabled_until = 0xFCF8;
            debug!("{:04X} . {:04X} skip '{}'", self.pc, self.disabled_until, "RAM test and find RAM end");
        }
        if self.pc == 0xFCF8 && !self.disabled{
            //self.cnt = 1;
            self.disabled = true;
            self.disabled_until = 0xFCFB;
            debug!("{:04X} . {:04X} skip '{}'", self.pc, self.disabled_until, "restore default I/O vectors");
        }
        if self.pc == 0xFCFB && !self.disabled{
            //self.cnt = 1;
            self.disabled = true;
            self.disabled_until = 0xFCFE;
            debug!("{:04X} . {:04X} skip '{}'", self.pc, self.disabled_until, "initialise VIC and screen editor");
        }
        if self.pc == 0xE394 && !self.disabled{
            //self.cnt = 1;
            //self.disabled = true;
            self.disabled_until = 0xE39D;
            debug!("{:04X} . {:04X} skip '{}'", self.pc, self.disabled_until, "*** BASIC cold start entry point");
        }
        if self.pc == 0xB487 && !self.disabled{
            //self.cnt = 1;
            self.disabled = true;
            self.disabled_until = 0xB4F3;
            debug!("{:04X} . {:04X} skip '{}'", self.pc, self.disabled_until, "print \" terminated string to utility pointer");
        }



        if self.pc == 0xE5CD && !self.disabled{
            // self.cnt = 1;
            self.disabled = true;
            self.disabled_until = 0xE5D6;
            debug!("({:4}){:04X} . {:04X} skip '{}'",  self.tick, self.pc, self.disabled_until, "basic idle loop");
        }
        if self.cnt > 0 {
            self.cnt += 1;
            if self.cnt == 500 {
                panic!("rrr");
            }
        }


        if self.disabled && self.pc == self.disabled_until {
            self.disabled = false;
            //debug!("            end skip");
        }

        if self.interrupted || !self.disabled {
            debug!("{}", line);
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

