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

#[derive(Debug)]
pub enum Timer8bitType {
    A,
    B,
}

pub struct Timer8bit {
    count: u16,
    last_cycle: u64,
    last_mode: Mode,
    is_up_phase: bool,
    timer_type: Timer8bitType,
    sram: Rc<RefCell<SRAM>>,

    tcnt: RegisterAddr,
    tccra: RegisterAddr,
    tccrb: RegisterAddr,
    ocra: RegisterAddr,
    ocrb: RegisterAddr,
    tov: RegisterBitAddr,
    ocfa: RegisterBitAddr,
    ocfb: RegisterBitAddr,
}

impl Timer8bit {
    pub fn new(
        timer_type: Timer8bitType,
        sram: Rc<RefCell<SRAM>>,
        tcnt: RegisterAddr,
        tccra: RegisterAddr,
        tccrb: RegisterAddr,
        ocra: RegisterAddr,
        ocrb: RegisterAddr,
        tov: RegisterBitAddr,
        ocfa: RegisterBitAddr,
        ocfb: RegisterBitAddr,
    ) -> Timer8bit {
        Timer8bit {
            count: 0,
            last_cycle: 0,
            last_mode: Mode::Normal,
            is_up_phase: true,
            timer_type: timer_type,
            sram: sram,
            tcnt: tcnt,
            tccra: tccra,
            tccrb: tccrb,
            ocra: ocra,
            ocrb: ocrb,
            tov: tov,
            ocfa: ocfa,
            ocfb: ocfb,
        }
    }

    fn tcnt(&self) -> u8 {
        self.sram.borrow().get(self.tcnt)
    }

    fn tccra(&self) -> u8 {
        self.sram.borrow().get(self.tccra)
    }

    fn tccrb(&self) -> u8 {
        self.sram.borrow().get(self.tccrb)
    }

    fn ocra(&self) -> u8 {
        self.sram.borrow().get(self.ocra)
    }

    fn ocrb(&self) -> u8 {
        self.sram.borrow().get(self.ocrb)
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

    fn prescale_a(&self) -> Option<u16> {
        match self.tccrb() & 0b111 {
            0b001 => Some(1),
            0b010 => Some(8),
            0b011 => Some(64),
            0b100 => Some(256),
            0b101 => Some(1024),
            _ => None,
        }
    }

    fn prescale_b(&self) -> Option<u16> {
        let tccrb = self.sram.borrow().get(self.tccrb);
        match tccrb & 0b111 {
            0b001 => Some(1),
            0b010 => Some(8),
            0b011 => Some(32),
            0b100 => Some(64),
            _ => None,
        }
    }

    fn prescale(&self) -> Option<u16> {
        match self.timer_type {
            Timer8bitType::A => self.prescale_a(),
            Timer8bitType::B => self.prescale_b(),
        }
    }

    fn mode(&self) -> Mode {
        match ((self.tccrb() & 0b1000) >> 3, self.tccra() & 0b11) {
            (0b0, 0b00) => Mode::Normal,
            (0b0, 0b01) => Mode::PhaseCorrectPWM,
            (0b0, 0b10) => Mode::CTC,
            (0b0, 0b11) => Mode::FastPWM,
            (0b1, 0b01) => Mode::PhaseCorrectPWM,
            (0b1, 0b11) => Mode::FastPWM,
            (_, _) => Mode::Normal,
        }
    }

    fn top(&self) -> u8 {
        match ((self.tccrb() & 0b1000) >> 3, self.tccra() & 0b11) {
            (0b0, 0b00) => 0xff,
            (0b0, 0b01) => 0xff,
            (0b0, 0b10) => self.ocra(),
            (0b0, 0b11) => 0xff,
            (0b1, 0b01) => self.ocra(),
            (0b1, 0b11) => self.ocra(),
            (_, _) => 0xff,
        }
    }

    pub fn clk_io(&mut self, cycle: u64) {
        if !self.is_on() {
            self.last_cycle = cycle;
            return;
        }

        if self.last_mode != self.mode() && self.count > 0 {
            self.count -= 1;
        }

        let diff_clk = cycle - self.last_cycle;
        self.count += diff_clk as u16;

        let prescale = self.prescale().unwrap();
        if self.count > prescale {
            self.count -= prescale;
            if self.is_up_phase {
                self.sram.borrow_mut().set(self.tcnt, self.tcnt() + 1);
            } else {
                self.sram.borrow_mut().set(self.tcnt, self.tcnt() - 1);
            };
        }

        match self.mode() {
            Mode::Normal => {
                if self.tcnt() >= self.top() {
                    self.sram.borrow_mut().set(self.tcnt, 0);
                    self.sram.borrow_mut().set_bit(self.tov, true);
                }
            }
            Mode::CTC => (),
            Mode::FastPWM => {
                // TODO: compare match with OCRA, OCRB and update OCnA, OCnB
                if self.tcnt() >= self.top() {
                    self.sram.borrow_mut().set(self.tcnt, 0);
                    // WIP: update OCnA, OCnB
                }
            }
            Mode::PhaseCorrectPWM => {
                if self.tcnt() >= self.top() {
                    self.is_up_phase = false;
                }
                if self.tcnt() <= 0 {
                    self.is_up_phase = true;
                }

                // うまく実装できないが、登り始めでセットされる..
                if 1 <= self.tcnt() && self.tcnt() < 10 && self.is_up_phase {
                    self.sram.borrow_mut().set_bit(self.tov, true);
                }
            }
        }

        self.last_cycle = cycle;
        self.last_mode = self.mode();
    }
}

impl fmt::Display for Timer8bit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "8bit timer =====
    power: {},    mode: {:?},    prescale: {:?},    top: {},
    count: {:3},    tcnt:  {:3},
    tccra: {:3},    tccrb: {:3},    ocra: {:3},    ocrb: {:3}",
            if self.is_on() { "ON" } else { "OFF" },
            self.mode(),
            self.prescale(),
            self.top(),
            self.count,
            self.tcnt(),
            self.tccra(),
            self.tccrb(),
            self.ocra(),
            self.ocrb(),
        )
    }
}
