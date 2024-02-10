pub struct Cia1 {
    port_a_write: u8,
    port_b_read: u8,
    port_a_direction: u8,
    port_b_direction: u8,
    timer_a_start_value_low: u8,
    timer_a_start_value_high: u8,
    interrupt_mask: u8,
    interrupt_data: u8,
    timer_a_control: u8,
    timer_b_control: u8,
}

impl Cia1 {
    pub fn new() -> Cia1 {
        Cia1 {
            port_a_write: 0,
            port_b_read: 0xFF,
            port_a_direction: 0,
            port_b_direction: 0,
            timer_a_start_value_low: 0,
            timer_a_start_value_high: 0,
            interrupt_mask: 0,
            interrupt_data: 0,
            timer_a_control: 0,
            timer_b_control: 0,
        }
    }

    pub fn get(&mut self, loc: usize) -> u8 {
        //debug!("cia1 get {:04X} ", loc);
        if loc == 0xDC01 {
            return self.port_b_read;
        }
        if loc == 0xDC0D {
            let i = self.interrupt_data;
            self.interrupt_data = 0;
            return i;
        }
        if loc == 0xDC0E {
            return self.timer_a_control;
        }
        panic!("cia1 get {:04X} ", loc);
        0
    }

    pub fn set(&mut self, loc: usize, value: u8) {
        //debug!("cia1 set {:04X} = {:02X}", loc, value);
        match loc {
            0xDC00 => self.port_a_write = value,
            0xDC02 => self.port_a_direction = value,
            0xDC03 => self.port_b_direction = value,
            0xDC04 => self.timer_a_start_value_low = value,
            0xDC05 => self.timer_a_start_value_high = value,
            0xDC0D => {
                if value & 0x80 == 0x80 {
                    self.interrupt_mask |= value & 0x1F;
                } else {
                    self.interrupt_mask &= !value & 0x1F;
                }
            }
            0xDC0E => self.timer_a_control = value,
            0xDC0F => self.timer_b_control = value,
            _ => panic!("cia1 set {:04X} = {:02X}", loc, value),
        }
    }

    pub fn timer_a_start_value_low(&self) -> u8 {
        self.timer_a_start_value_low
    }

    pub fn timer_a_start_value_high(&self) -> u8 {
        self.timer_a_start_value_high
    }

    pub fn interrupt_timer_a(&mut self) -> bool {
        if self.interrupt_mask & 0x01 == 0x01 {
            self.interrupt_data = 0x81;
            return true;
        }
        false
    }

    pub fn timer_a_control(&self) -> u8 {
        self.timer_a_control
    }

    pub fn port_b_read_or(&mut self, value: u8) {
        self.port_b_read |= value;
    }

    pub fn port_b_read_and(&mut self, value: u8) {
        self.port_b_read &= value;
    }

    pub fn port_a_direction(&self) -> u8 {
        self.port_a_direction
    }
    pub fn port_b_direction(&self) -> u8 {
        self.port_b_direction
    }



    pub fn port_a_write(&self) -> u8 {
        self.port_a_write
    }

    //todo ovo treba biti private
    // pub fn port_b_read(&self) -> u8 {
    //     self.port_b_read
    // }
}