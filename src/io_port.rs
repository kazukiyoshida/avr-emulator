use super::avr::*;
use std::fmt;

pub struct IOPort<'a> {
    pub avr: &'a dyn AVR,

    pub portx: RegisterAddr,
    pub ddrx: RegisterAddr,
    pub pinx: RegisterAddr,
}

impl<'a> IOPort<'a> {
    pub fn new(
        avr: &'a dyn AVR,
        portx: RegisterAddr,
        ddrx: RegisterAddr,
        pinx: RegisterAddr,
    ) -> IOPort<'a> {
        IOPort {
            avr: avr,
            portx: portx,
            ddrx: ddrx,
            pinx: pinx,
        }
    }

    pub fn portx(&self) -> u8 {
        self.avr.get_register(self.portx)
    }

    pub fn ddrx(&self) -> u8 {
        self.avr.get_register(self.ddrx)
    }

    pub fn pinx(&self) -> u8 {
        self.avr.get_register(self.pinx)
    }
}

impl<'a> fmt::Display for IOPort<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "portx: {:08b}    ddrx: {:08b}    pinx: {:08b}",
            self.portx(),
            self.ddrx(),
            self.pinx(),
        )
    }
}
