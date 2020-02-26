use super::avr::*;
use std::fmt;

pub struct Timer8bit<'a> {
    pub count: u8,
    pub mode: Mode,
    pub avr: &'a dyn AVR,

    // Timer counter
    pub tcnt: RegisterAddr,

    // Timer counter control register
    pub tccr: RegisterAddr,

    // Output compare register
    pub ocra: RegisterAddr,
    pub ocrb: RegisterAddr,
}

impl<'a> fmt::Display for Timer8bit<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "8bit timer {{
    count: {}, mode: {:?}, tcnt: {},
    tccr: {}, ocra: {}, ocrb: {}
}}",
            self.count,
            self.mode,
            self.avr.get_register(self.tcnt),
            self.avr.get_register(self.tccr),
            self.avr.get_register(self.ocra),
            self.avr.get_register(self.ocrb),
        )
    }
}

impl<'a> Timer8bit<'a> {
    pub fn new(
        avr: &'a dyn AVR,
        tcnt: RegisterAddr,
        tccr: RegisterAddr,
        ocra: RegisterAddr,
        ocrb: RegisterAddr,
    ) -> Timer8bit<'a> {
        Timer8bit {
            count: 0,
            mode: Mode::Normal,
            avr: avr,
            tcnt: tcnt,
            tccr: tccr,
            ocra: ocra,
            ocrb: ocrb,
        }
    }

    pub fn input_clk(&mut self) {
        self.count += 1;
    }

    pub fn count(&self) {}

    pub fn clear(&self) {}
}

pub struct Timer16bit<'a> {
    pub count: u16,
    pub avr: &'a dyn AVR,
    pub mode: Mode,
    // Timer counter
    pub tcnt: RegisterWordAddr,
    // Timer counter control register
    pub tccra: RegisterAddr,
    pub tccrb: RegisterAddr,
    pub tccrc: RegisterAddr,
    // Output compare ragister
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
        ocra: RegisterWordAddr,
        ocrb: RegisterWordAddr,
    ) -> Timer16bit<'a> {
        Timer16bit {
            count: 0,
            avr: avr,
            mode: Mode::Normal,
            tcnt: tcnt,
            tccra: tccra,
            tccrb: tccrb,
            tccrc: tccrc,
            ocra: ocra,
            ocrb: ocrb,
        }
    }

    pub fn input_clk(&mut self) {
        self.count += 1;
        if !self.is_on() {
            return;
        }

        match self.mode() {
            Mode::Normal => match self.prescale() {
                Some(n) => {
                    if self.count > n {
                        self.count = 0;
                        let tcnt = self.avr.get_word(self.tcnt);
                        self.avr.set_word(self.tcnt, tcnt + 1);
                    }
                }
                None => (),
            },
            Mode::CTC => (),
            Mode::FastPWM => (),
            Mode::PhaseCorrectPWM => (),
        };
    }

    pub fn count(&self) {}

    pub fn clear(&self) {}

    pub fn is_on(&self) -> bool {
        self.avr.get_register(self.tccrb) & 0b111 != 0
    }

    pub fn prescale(&self) -> Option<u16> {
        match self.avr.get_register(self.tccrb) & 0b111 {
            0 => None,    // Timer counter 停止
            1 => Some(1), // prescale なし
            2 => Some(8),
            3 => Some(64),
            4 => Some(256),
            5 => Some(1024),
            _ => None, // clock edge の設定
        }
    }

    pub fn mode(&self) -> Mode {
        match (
            (self.avr.get_register(self.tccrb) & 0b11000) >> 3,
            self.avr.get_register(self.tccra) & 0b11,
        ) {
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
}

impl<'a> fmt::Display for Timer16bit<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "16bit timer {{                     power: {}
    count: {}, mode: {:?}, tcnt: {}, prescale: {:?},
    tccra: {}, tccrb: {}, tccrc: {},
    ocra:  {}, ocrb: {},
}}",
            if self.is_on() { "ON" } else { "OFF" },
            self.count,
            self.mode(),
            self.avr.get_word(self.tcnt),
            self.prescale(),
            self.avr.get_register(self.tccra),
            self.avr.get_register(self.tccrb),
            self.avr.get_register(self.tccrc),
            self.avr.get_word(self.ocra),
            self.avr.get_word(self.ocrb),
        )
    }
}

#[derive(Debug)]
pub enum Mode {
    Normal,
    CTC,
    FastPWM,
    PhaseCorrectPWM,
}
