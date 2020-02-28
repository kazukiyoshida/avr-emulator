use super::avr::*;
use super::utils::*;
use std::fmt;

pub struct IOPort<'a> {
    pub avr: &'a dyn AVR,

    pub portx: RegisterAddr,
    pub ddrx: RegisterAddr,
    pub pinx: RegisterAddr,

    pub last_portx: u8,
    pub last_ddrx: u8,
    pub last_pinx: u8,
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
            last_portx: 0,
            last_ddrx: 0,
            last_pinx: 0,
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

    // WIP: 1 cycle ずれている
    pub fn clk_io(&mut self) {
        if self.last_portx != self.portx()
            || self.last_ddrx != self.ddrx()
            || self.last_pinx != self.pinx()
        {
            // update pinx
            for n in 0..8 {
                // 出力 buffer が ON（ DDXn が ON ）ならば、PORTXn を PINXn に反映
                if bit(self.ddrx(), n) {
                    self.avr.set_bit((self.pinx, n), bit(self.portx(), n))
                }
            }
            self.last_portx = self.portx();
            self.last_ddrx = self.ddrx();
            self.last_pinx = self.pinx();
        }
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
