use super::avr::*;
use std::fmt;

#[derive(Debug)]
pub enum Mode {
    Normal,
    CTC,
    FastPWM,
    PhaseCorrectPWM,
}

// 8 bit timer ================================================================
pub struct Timer8bit<'a> {
    pub count: u16,
    pub avr: &'a dyn AVR,
    pub mode: Mode,
    pub last_cycle: u64,
    pub is_up_phase: bool,

    pub tcnt: RegisterAddr,
    pub tccra: RegisterAddr,
    pub tccrb: RegisterAddr,
    pub ocra: RegisterAddr,
    pub ocrb: RegisterAddr,
}

impl<'a> Timer8bit<'a> {
    pub fn new(
        avr: &'a dyn AVR,
        tcnt: RegisterAddr,
        tccra: RegisterAddr,
        tccrb: RegisterAddr,
        ocra: RegisterAddr,
        ocrb: RegisterAddr,
    ) -> Timer8bit<'a> {
        Timer8bit {
            count: 0,
            avr: avr,
            mode: Mode::Normal,
            last_cycle: 0,
            is_up_phase: true,

            tcnt: tcnt,
            tccra: tccra,
            tccrb: tccrb,
            ocra: ocra,
            ocrb: ocrb,
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

    pub fn is_on(&self) -> bool {
        self.tccrb() & 0b111 != 0
    }

    pub fn prescale(&self) -> Option<u16> {
        match self.tccrb() & 0b111 {
            0b001 => Some(1),
            0b010 => Some(8),
            0b011 => Some(64),
            0b100 => Some(256),
            0b101 => Some(1024),
            _ => None,
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

    pub fn update_count(&mut self) {
        let cycle_diff = self.avr.cycle() - self.last_cycle;
        self.last_cycle = self.avr.cycle();
        self.count += cycle_diff as u16;
    }

    pub fn input_clk(&mut self) {
        self.update_count();

        if !self.is_on() {
            return;
        }

        match self.mode() {
            Mode::Normal => match self.prescale() {
                Some(n) => {
                    if self.count > n {
                        self.avr.set_register(self.tcnt, self.tcnt() + 1);
                        self.count = 0;
                    }
                }
                None => (),
            },
            Mode::CTC => (),
            Mode::FastPWM => (),
            Mode::PhaseCorrectPWM => match self.prescale() {
                Some(n) => {
                    // update tcnt
                    if self.count > n {
                        self.count = 0;
                        if self.is_up_phase {
                            self.avr.set_register(self.tcnt, self.tcnt() + 1);
                        } else {
                            self.avr.set_register(self.tcnt, self.tcnt() - 1);
                        };
                    }

                    // update up/down phase
                    if self.tcnt() >= self.top() {
                        self.is_up_phase = false;
                    }
                    if self.tcnt() <= 0 {
                        self.is_up_phase = true;
                    }
                }
                None => (),
            },
        };
    }
}

impl<'a> fmt::Display for Timer8bit<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "8bit timer =====
    power: {},    mode: {:?},    prescale: {:?},
    count: {:3},    tcnt:  {:3},
    tccra: {:3},    tccrb: {:3},    ocra: {:3},    ocrb: {:3}",
            if self.is_on() { "ON" } else { "OFF" },
            self.mode,
            self.prescale(),
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
    pub mode: Mode,
    pub last_cycle: u64,
    pub is_up_phase: bool,

    pub tcnt: RegisterWordAddr,
    pub tccra: RegisterAddr,
    pub tccrb: RegisterAddr,
    pub tccrc: RegisterAddr,
    pub icr: RegisterWordAddr,
    pub ocra: RegisterWordAddr,
    pub ocrb: RegisterWordAddr,
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
    ) -> Timer16bit<'a> {
        Timer16bit {
            count: 0,
            avr: avr,
            mode: Mode::Normal,
            last_cycle: 0,

            tcnt: tcnt,
            is_up_phase: true,
            tccra: tccra,
            tccrb: tccrb,
            tccrc: tccrc,
            icr: icr,
            ocra: ocra,
            ocrb: ocrb,
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

    pub fn is_on(&self) -> bool {
        self.tccrb() & 0b111 != 0
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

    pub fn phase_correct_max(&self) -> u16 {
        match self.tccra() & 0b11 {
            0b01 => 0xff,
            0b10 => 0x1ff,
            0b11 => 0x3ff,
            _ => 0,
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

    pub fn update_count(&mut self) {
        let cycle_diff = self.avr.cycle() - self.last_cycle;
        self.last_cycle = self.avr.cycle();
        self.count += cycle_diff as u16;
    }

    pub fn input_clk(&mut self) {
        self.update_count();

        if !self.is_on() {
            return;
        }

        match self.mode() {
            Mode::Normal => match self.prescale() {
                Some(n) => {
                    if self.count > n {
                        self.avr.set_word(self.tcnt, self.tcnt() + 1);
                        self.count = 0;
                    }
                }
                None => (),
            },
            Mode::CTC => (),
            Mode::FastPWM => (),
            Mode::PhaseCorrectPWM => match self.prescale() {
                Some(n) => {
                    // update tcnt
                    if self.count > n {
                        self.count = 0;
                        if self.is_up_phase {
                            self.avr.set_word(self.tcnt, self.tcnt() + 1);
                        } else {
                            self.avr.set_word(self.tcnt, self.tcnt() - 1);
                        };
                    }

                    // update up/down phase
                    if self.tcnt() >= self.phase_correct_max() {
                        self.is_up_phase = false;
                    }
                    if self.tcnt() <= 0 {
                        self.is_up_phase = true;
                    }
                }
                None => (),
            },
        };
    }
}

impl<'a> fmt::Display for Timer16bit<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "16bit timer =====
    power: {},    mode: {:?},    prescale: {:?},
    count: {:3},    tcnt:  {:3},
    tccra: {:3},    tccrb: {:3},    tccrc: {:3},    icr: {:3},    ocra: {:3},    ocrb: {:3},",
            if self.is_on() { "ON" } else { "OFF" },
            self.mode(),
            self.prescale(),
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
