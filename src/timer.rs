use super::avr::*;
use std::fmt;

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

// 8 bit timer ================================================================
pub struct Timer8bit<'a> {
    pub timer_type: Timer8bitType,
    pub count: u16,
    pub avr: &'a dyn AVR,
    pub last_cycle: u64,
    pub last_mode: Mode,
    pub is_up_phase: bool,

    pub tcnt: RegisterAddr,
    pub tccra: RegisterAddr,
    pub tccrb: RegisterAddr,
    pub ocra: RegisterAddr,
    pub ocrb: RegisterAddr,
    pub tov: RegisterBitAddr,
    pub ocfa: RegisterBitAddr,
    pub ocfb: RegisterBitAddr,
}

impl<'a> Timer8bit<'a> {
    pub fn new(
        timer_type: Timer8bitType,
        avr: &'a dyn AVR,
        tcnt: RegisterAddr,
        tccra: RegisterAddr,
        tccrb: RegisterAddr,
        ocra: RegisterAddr,
        ocrb: RegisterAddr,
        tov: RegisterBitAddr,
        ocfa: RegisterBitAddr,
        ocfb: RegisterBitAddr,
    ) -> Timer8bit<'a> {
        Timer8bit {
            timer_type: timer_type,
            count: 0,
            avr: avr,
            last_cycle: 0,
            last_mode: Mode::Normal,
            is_up_phase: true,

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

    pub fn tcnt(&self) -> u8 {
        self.avr.get_register(self.tcnt)
    }

    pub fn tccra(&self) -> u8 {
        self.avr.get_register(self.tccra)
    }

    pub fn tccrb(&self) -> u8 {
        self.avr.get_register(self.tccrb)
    }

    pub fn ocra(&self) -> u8 {
        self.avr.get_register(self.ocra)
    }

    pub fn ocrb(&self) -> u8 {
        self.avr.get_register(self.ocrb)
    }

    pub fn tov(&self) -> bool {
        self.avr.get_bit(self.tov)
    }

    pub fn ocfa(&self) -> bool {
        self.avr.get_bit(self.ocfa)
    }

    pub fn ocfb(&self) -> bool {
        self.avr.get_bit(self.ocfb)
    }

    pub fn is_on(&self) -> bool {
        self.prescale().is_some()
    }

    pub fn prescale_a(&self) -> Option<u16> {
        match self.tccrb() & 0b111 {
            0b001 => Some(1),
            0b010 => Some(8),
            0b011 => Some(64),
            0b100 => Some(256),
            0b101 => Some(1024),
            _ => None,
        }
    }

    pub fn prescale_b(&self) -> Option<u16> {
        match self.tccrb() & 0b111 {
            0b001 => Some(1),
            0b010 => Some(8),
            0b011 => Some(32),
            0b100 => Some(64),
            _ => None,
        }
    }

    pub fn prescale(&self) -> Option<u16> {
        match self.timer_type {
            Timer8bitType::A => self.prescale_a(),
            Timer8bitType::B => self.prescale_b(),
        }
    }

    pub fn mode(&self) -> Mode {
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

    pub fn top(&self) -> u8 {
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

    pub fn clk_io(&mut self) {
        if !self.is_on() {
            self.last_cycle = self.avr.cycle();
            return;
        }

        if self.last_mode != self.mode() && self.count > 0 {
            self.count -= 1;
        }

        let diff_clk = self.avr.cycle() - self.last_cycle;
        self.count += diff_clk as u16;

        let prescale = self.prescale().unwrap();
        if self.count > prescale {
            self.count -= prescale;
            if self.is_up_phase {
                self.avr.set_register(self.tcnt, self.tcnt() + 1);
            } else {
                self.avr.set_register(self.tcnt, self.tcnt() - 1);
            };
        }

        match self.mode() {
            Mode::Normal => {
                if self.tcnt() >= self.top() {
                    self.avr.set_register(self.tcnt, 0);
                    self.avr.set_bit(self.tov, true);
                }
            }
            Mode::CTC => (),
            Mode::FastPWM => {
                // WIP: compare match with OCRA, OCRB and update OCnA, OCnB
                if self.tcnt() >= self.top() {
                    self.avr.set_register(self.tcnt, 0);
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
                    self.avr.set_bit(self.tov, true);
                }
            }
        }

        // update state
        self.last_cycle = self.avr.cycle();
        self.last_mode = self.mode();
    }
}

impl<'a> fmt::Display for Timer8bit<'a> {
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

// 16 bit timer ================================================================
pub struct Timer16bit<'a> {
    pub count: u16,
    pub avr: &'a dyn AVR,
    pub last_cycle: u64,
    pub last_prescale: Option<u16>,
    pub is_up_phase: bool,

    pub tcnt: RegisterWordAddr,
    pub tccra: RegisterAddr,
    pub tccrb: RegisterAddr,
    pub tccrc: RegisterAddr,
    pub icr: RegisterWordAddr,
    pub ocra: RegisterWordAddr,
    pub ocrb: RegisterWordAddr,
    pub tov: RegisterBitAddr,
    pub ocfa: RegisterBitAddr,
    pub ocfb: RegisterBitAddr,
}

impl<'a> Timer16bit<'a> {
    pub fn new(
        avr: &'a dyn AVR,
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
    ) -> Timer16bit<'a> {
        Timer16bit {
            count: 0,
            avr: avr,
            last_cycle: 0,
            last_prescale: None,

            tcnt: tcnt,
            is_up_phase: true,
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

    pub fn tcnt(&self) -> u16 {
        self.avr.get_word(self.tcnt)
    }

    pub fn tccra(&self) -> u8 {
        self.avr.get_register(self.tccra)
    }

    pub fn tccrb(&self) -> u8 {
        self.avr.get_register(self.tccrb)
    }

    pub fn tccrc(&self) -> u8 {
        self.avr.get_register(self.tccrc)
    }

    pub fn icr(&self) -> u16 {
        self.avr.get_word(self.icr)
    }

    pub fn ocra(&self) -> u16 {
        self.avr.get_word(self.ocra)
    }

    pub fn ocrb(&self) -> u16 {
        self.avr.get_word(self.ocrb)
    }

    pub fn tov(&self) -> bool {
        self.avr.get_bit(self.tov)
    }

    pub fn ocfa(&self) -> bool {
        self.avr.get_bit(self.ocfa)
    }

    pub fn ocfb(&self) -> bool {
        self.avr.get_bit(self.ocfb)
    }

    pub fn is_on(&self) -> bool {
        self.prescale().is_some()
    }

    pub fn prescale(&self) -> Option<u16> {
        match self.tccrb() & 0b111 {
            1 => Some(1),
            2 => Some(8),
            3 => Some(64),
            4 => Some(256),
            5 => Some(1024),
            _ => None,
        }
    }

    pub fn top(&self) -> u16 {
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

    pub fn mode(&self) -> Mode {
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

    pub fn clk_io(&mut self) {
        if !self.is_on() {
            self.last_cycle = self.avr.cycle();
            return;
        }

        if self.last_prescale != self.prescale() {
            // WIP: prescale が増加した場合、その増加の比率だけ count を進め、
            // tcnt は +1 される、としている.
            if self.last_prescale.is_some() && self.prescale().is_some() {
                let last_prescale = self.last_prescale.unwrap();
                let prescale = self.prescale().unwrap();
                if prescale > last_prescale {
                    self.count = self.count * prescale / last_prescale;
                    if self.is_up_phase {
                        self.avr.set_word(self.tcnt, self.tcnt() + 1);
                    } else {
                        self.avr.set_word(self.tcnt, self.tcnt() - 1);
                    };
                }
                self.count += 1;
            } else {
                self.count = 0;
            }
        } else {
            let diff_clk = self.avr.cycle() - self.last_cycle;
            self.count += diff_clk as u16;

            let prescale = self.prescale().unwrap();
            if self.count > prescale {
                self.count -= prescale;
                if self.is_up_phase {
                    self.avr.set_word(self.tcnt, self.tcnt() + 1);
                } else {
                    self.avr.set_word(self.tcnt, self.tcnt() - 1);
                };
            }
        }

        // check tcnt's compare match
        match self.mode() {
            Mode::Normal => {
                if self.tcnt() >= self.top() {
                    self.avr.set_word(self.tcnt, 0);
                    self.avr.set_bit(self.tov, !self.tov());
                }
            }
            Mode::CTC => (),
            Mode::FastPWM => {
                // compare match with OCRA, OCRB and update OCnA, OCnB
                if self.tcnt() >= self.top() {
                    self.avr.set_word(self.tcnt, 0);
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
        self.last_cycle = self.avr.cycle();
        self.last_prescale = self.prescale();
    }
}

impl<'a> fmt::Display for Timer16bit<'a> {
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
