use log::debug;

#[cfg(debug_assertions)]
pub struct CpuLogger {
    pc: u16,
    opcode: u8,
    operand1: Option<u8>,
    operand2: Option<u8>,
}

#[cfg(debug_assertions)]
impl CpuLogger {
    pub fn new() -> CpuLogger {
        CpuLogger {
            pc: 0,
            opcode: 0,
            operand1: None,
            operand2: None,
        }
    }

    pub fn pc(&mut self, pc: u16) {
        self.pc = pc;
        self.operand1 = None;
        self.operand2 = None;
    }

    pub fn opcode(&mut self, opcode: u8) {
        self.opcode = opcode;
    }

    pub fn op(&mut self, op: u8) {
        if self.operand1 == None {
            self.operand1 = Some(op);
        } else {
            self.operand2 = Some(op);
        }
    }

    pub fn inst(&mut self, inst: &'static str) {
        let mut line = String::new();
        line.push_str(format!("{:04X}   {:02X}", self.pc, self.opcode).as_str());
        line.push_str(match self.operand1 {
            Some(x) => format!(" {:02X}", x),
            None => "   ".to_string()
        }.as_str());
        line.push_str(match self.operand2 {
            Some(x) => format!(" {:02X}", x),
            None => "   ".to_string()
        }.as_str());
        line.push_str(format!("   {}", inst).as_str());
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

