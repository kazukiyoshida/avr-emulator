use super::avr::*;
use std::fmt;

pub struct IOPort<'a> {
    pub mode: Mode,
    pub avr: &'a dyn AVR,

    pub portx: RegisterAddr,
    pub ddrx: RegisterAddr,
    pub pinx: RegisterAddr,
}

impl<'a> fmt::Display for IOPort<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "IO Port {{
    mode: {:?}, portx: {}, ddrx: {}, pinx: {}
}}",
            self.mode,
            self.avr.get_register(self.portx),
            self.avr.get_register(self.ddrx),
            self.avr.get_register(self.pinx),
        )
    }
}

impl<'a> IOPort<'a> {
    pub fn new(
        avr: &'a dyn AVR,
        portx: RegisterAddr,
        ddrx: RegisterAddr,
        pinx: RegisterAddr,
    ) -> IOPort<'a> {
        IOPort {
            mode: Mode::OUT,
            avr: avr,
            portx: portx,
            ddrx: ddrx,
            pinx: pinx,
        }
    }
}

#[derive(Debug)]
pub enum Mode {
    IN,
    OUT,
}
