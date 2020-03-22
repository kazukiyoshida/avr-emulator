use super::sram::*;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub enum Mode {
    Normal,
    CTC,
    FastPWM,
    PhaseCorrectPWM,
}

pub struct Timer16bit {
    count: u16,
    last_cycle: u64,
    last_mode: Mode,
    last_prescale: Option<u16>,
    is_up_phase: bool,
    sram: Rc<RefCell<SRAM>>,

    tcnt: RegisterWordAddr,
    tccra: RegisterAddr,
    tccrb: RegisterAddr,
    tccrc: RegisterAddr,
    icr: RegisterWordAddr,
    ocra: RegisterWordAddr,
    ocrb: RegisterWordAddr,
    tov: RegisterBitAddr,
    ocfa: RegisterBitAddr,
    ocfb: RegisterBitAddr,
}

impl Timer16bit {
    pub fn new(
        sram: Rc<RefCell<SRAM>>,
        tcnt: RegisterWordAddr,
        tccra: RegisterAddr,
        tccrb: RegisterAddr,
        tccrc: RegisterAddr,
        icr: RegisterWordAddr,
        ocra: RegisterWordAddr,
        ocrb: RegisterWordAddr,
        tov: RegisterBitAddr,
        ocfa: RegisterBitAddr,
        ocfb: RegisterBitAddr,
    ) -> Timer16bit {
        Timer16bit {
            count: 0,
            last_cycle: 0,
            last_mode: Mode::Normal,
            last_prescale: None,
            is_up_phase: true,
            sram: sram,
            tcnt: tcnt,
            tccra: tccra,
            tccrb: tccrb,
            tccrc: tccrc,
            icr: icr,
            ocra: ocra,
            ocrb: ocrb,
            tov: tov,
            ocfa: ocfa,
            ocfb: ocfb,
        }
    }

    fn tcnt(&self) -> u16 {
        self.sram.borrow().get_word(self.tcnt)
    }

    fn tccra(&self) -> u8 {
        self.sram.borrow().get(self.tccra)
    }

    fn tccrb(&self) -> u8 {
        self.sram.borrow().get(self.tccrb)
    }

    fn tccrc(&self) -> u8 {
        self.sram.borrow().get(self.tccrc)
    }

    fn icr(&self) -> u16 {
        self.sram.borrow().get_word(self.icr)
    }

    fn ocra(&self) -> u16 {
        self.sram.borrow().get_word(self.ocra)
    }

    fn ocrb(&self) -> u16 {
        self.sram.borrow().get_word(self.ocrb)
    }

    fn tov(&self) -> bool {
        self.sram.borrow().get_bit(self.tov)
    }

    fn ocfa(&self) -> bool {
        self.sram.borrow().get_bit(self.ocfa)
    }

    fn ocfb(&self) -> bool {
        self.sram.borrow().get_bit(self.ocfb)
    }

    fn is_on(&self) -> bool {
        self.prescale().is_some()
    }

    fn prescale(&self) -> Option<u16> {
        match self.tccrb() & 0b111 {
            1 => Some(1),
            2 => Some(8),
            3 => Some(64),
            4 => Some(256),
            5 => Some(1024),
            _ => None,
        }
    }

    fn top(&self) -> u16 {
        match ((self.tccrb() & 0b11000) >> 3, self.tccra() & 0b11) {
            (0b00, 0b00) => 0xffff,
            (0b00, 0b01) => 0x00ff,
            (0b00, 0b10) => 0x01ff,
            (0b00, 0b11) => 0x03ff,
            (0b01, 0b00) => self.ocra(),
            (0b01, 0b01) => 0x00ff,
            (0b01, 0b10) => 0x01ff,
            (0b01, 0b11) => 0x03ff,
            (0b10, 0b00) => self.icr(),
            (0b10, 0b01) => self.ocra(),
            (0b10, 0b10) => self.icr(),
            (0b10, 0b11) => self.ocra(),
            (0b11, 0b00) => self.icr(),
            (0b11, 0b10) => self.icr(),
            (0b11, 0b11) => self.ocra(),
            (_, _) => 0xffff,
        }
    }

    fn mode(&self) -> Mode {
        match ((self.tccrb() & 0b11000) >> 3, self.tccra() & 0b11) {
            (0b00, 0b00) => Mode::Normal,
            (0b00, 0b01) => Mode::PhaseCorrectPWM,
            (0b00, 0b10) => Mode::PhaseCorrectPWM,
            (0b00, 0b11) => Mode::PhaseCorrectPWM,
            (0b01, 0b00) => Mode::CTC,
            (0b01, 0b01) => Mode::FastPWM,
            (0b01, 0b10) => Mode::FastPWM,
            (0b01, 0b11) => Mode::FastPWM,
            (0b10, 0b00) => Mode::PhaseCorrectPWM,
            (0b10, 0b01) => Mode::PhaseCorrectPWM,
            (0b10, 0b10) => Mode::PhaseCorrectPWM,
            (0b10, 0b11) => Mode::PhaseCorrectPWM,
            (0b11, 0b00) => Mode::CTC,
            (0b11, 0b01) => Mode::Normal,
            (0b11, 0b10) => Mode::FastPWM,
            (0b11, 0b11) => Mode::FastPWM,
            (_, _) => Mode::Normal,
        }
    }

    pub fn clk_io(&mut self, cycle: u64) {
        if !self.is_on() {
            self.last_cycle = cycle;
            return;
        }

        if self.last_prescale != self.prescale() {
            // TODO: prescale が増加した場合、その増加の比率だけ count を進め、
            //       tcnt は +1 される、としている.
            if self.last_prescale.is_some() && self.prescale().is_some() {
                let last_prescale = self.last_prescale.unwrap();
                let prescale = self.prescale().unwrap();
                if prescale > last_prescale {
                    self.count = self.count * prescale / last_prescale;
                    if self.is_up_phase {
                        self.sram.borrow_mut().set_word(self.tcnt, self.tcnt() + 1);
                    } else {
                        self.sram.borrow_mut().set_word(self.tcnt, self.tcnt() - 1);
                    };
                }
                self.count += 1;
            } else {
                self.count = 0;
            }
        } else {
            let diff_clk = cycle - self.last_cycle;
            self.count += diff_clk as u16;

            let prescale = self.prescale().unwrap();
            if self.count > prescale {
                self.count -= prescale;
                if self.is_up_phase {
                    self.sram.borrow_mut().set_word(self.tcnt, self.tcnt() + 1);
                } else {
                    self.sram.borrow_mut().set_word(self.tcnt, self.tcnt() - 1);
                };
            }
        }

        // check tcnt's compare match
        match self.mode() {
            Mode::Normal => {
                if self.tcnt() >= self.top() {
                    self.sram.borrow_mut().set_word(self.tcnt, 0);
                    self.sram.borrow_mut().set_bit(self.tov, !self.tov());
                }
            }
            Mode::CTC => (),
            Mode::FastPWM => {
                // compare match with OCRA, OCRB and update OCnA, OCnB
                if self.tcnt() >= self.top() {
                    self.sram.borrow_mut().set_word(self.tcnt, 0);
                    // update OCnA, OCnB
                }
            }
            Mode::PhaseCorrectPWM => {
                if self.tcnt() >= self.top() {
                    self.is_up_phase = false;
                }
                if self.tcnt() <= 0 {
                    self.is_up_phase = true;
                }
            }
        }

        // update state
        self.last_cycle = cycle;
        self.last_prescale = self.prescale();
    }
}

impl fmt::Display for Timer16bit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "16bit timer =====
    power: {},    mode: {:?},    prescale: {:?},    top: {},
    count: {:3},    tcnt:  {:3},
    tccra: {:3},    tccrb: {:3},    tccrc: {:3},    icr: {:3},    ocra: {:3},    ocrb: {:3},",
            if self.is_on() { "ON" } else { "OFF" },
            self.mode(),
            self.prescale(),
            self.top(),
            self.count,
            self.tcnt(),
            self.tccra(),
            self.tccrb(),
            self.tccrc(),
            self.icr(),
            self.ocra(),
            self.ocrb(),
        )
    }
}
