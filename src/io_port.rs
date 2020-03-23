use super::sram::*;
use super::util::bit::*;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

pub struct IOPort {
    sram: Rc<RefCell<SRAM>>,
    last_portx: u8,
    last_ddrx: u8,
    last_pinx: u8,
    portx: RegisterAddr,
    ddrx: RegisterAddr,
    pinx: RegisterAddr,
}

impl IOPort {
    pub fn new(
        sram: Rc<RefCell<SRAM>>,
        portx: RegisterAddr,
        ddrx: RegisterAddr,
        pinx: RegisterAddr,
    ) -> IOPort {
        IOPort {
            sram: sram,
            last_portx: 0,
            last_ddrx: 0,
            last_pinx: 0,
            portx: portx,
            ddrx: ddrx,
            pinx: pinx,
        }
    }

    fn portx(&self) -> u8 {
        self.sram.borrow().get(self.portx)
    }

    fn ddrx(&self) -> u8 {
        self.sram.borrow().get(self.ddrx)
    }

    fn pinx(&self) -> u8 {
        self.sram.borrow().get(self.pinx)
    }
}

impl Iterator for IOPort {
    type Item = ();
    // TODO: 1 cycle ずれている
    fn next(&mut self) -> Option<()> {
        if self.last_portx != self.portx()
            || self.last_ddrx != self.ddrx()
            || self.last_pinx != self.pinx()
        {
            // update pinx
            for n in 0..8 {
                // 出力 buffer が ON（ DDXn が ON ）ならば、PORTXn を PINXn に反映
                if bit(self.ddrx(), n) {
                    self.sram
                        .borrow_mut()
                        .set_bit((self.pinx, n), bit(self.portx(), n))
                }
            }
            self.last_portx = self.portx();
            self.last_ddrx = self.ddrx();
            self.last_pinx = self.pinx();
        }
        Some(())
    }
}

impl fmt::Display for IOPort {
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
