use crate::emulator::cpu::Cpu;
use crate::emulator::memory::cia1::Cia1;

pub struct TimerA {
    control_register_address: u16,
    value_low_address: u16,
    value_high_address: u16,
    latch_loaded: bool,
    value: u16,
    clear_port_b: bool,
}

impl TimerA {
    pub fn new() -> TimerA {
        TimerA {
            control_register_address: 0xDC0E,
            value_low_address: 0xDC04,
            value_high_address: 0xDC05,
            latch_loaded: false,
            value: 0,
            clear_port_b: false,
        }
    }

    pub fn tick(&mut self, cpu: &mut Cpu, cia1: &mut Cia1)  {
        if self.clear_port_b {
            //todo revert cia1.port_b_read_and(!0x40);
            //debug!("port b low");
            self.clear_port_b = false;
        }
        if cia1.timer_a_control() & 0x10 == 0x10 {
            if !self.latch_loaded {
                self.value = (cia1.timer_a_start_value_low() as u16) | ((cia1.timer_a_start_value_high() as u16) << 8);
                //debug!("start value loaded initially {:04X}", self.value);
                self.latch_loaded = true;
            }
        } else {
            if self.latch_loaded {
                self.latch_loaded = false;
            }
        }

        if cia1.timer_a_control() & 0x01 == 0x01 {
            //timer is active
            if self.value == 0 {
                if cia1.timer_a_control() & 0x02 == 0x02 {
                    panic!("timer a underflow")
                }
                if cia1.timer_a_control() & 0x04 == 0x04 {
                    panic!("timer a underflow")
                } else {
                    //todo revert cia1.port_b_read_or(0x40);
                    //debug!("port b high");
                    self.clear_port_b = true;
                }
                if cia1.timer_a_control() & 0x08 == 0x08 {
                    panic!("timer a underflow")
                } else {
                    self.value = (cia1.timer_a_start_value_low() as u16) | ((cia1.timer_a_start_value_high() as u16) << 8);
                    //debug!("start value loaded {:04X}", self.value);
                }
                if cia1.interrupt_timer_a() {
                    cpu.interrupt();
                }
            } else {
                self.value -= 1;
            }
        }
    }
}