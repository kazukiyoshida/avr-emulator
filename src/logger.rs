use super::avr::*;
use super::memory::*;
use colored::*;
use difference::{Changeset, Difference};
use std::cell::RefCell;
use std::fmt;
use std::fmt::LowerHex;

pub struct Log {
    pub processor: String,
    pub registers: String,
    pub sram: String,
    pub stack: String,
}

impl fmt::Display for Log {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}\n{}\n{}\n..{}",
            self.processor, self.registers, self.sram, self.stack
        )
    }
}

impl Log {
    pub fn new(processor: String, registers: String, sram: String, stack: String) -> Log {
        Log {
            processor: processor,
            registers: registers,
            sram: sram,
            stack: stack,
        }
    }

    pub fn all(&self) -> String {
        format!(
            "{}\n{}\n{}\n{}",
            self.processor, self.registers, self.sram, self.stack
        )
    }

    pub fn diff(&self, log: &Log) -> String {
        let Changeset { diffs, .. } = Changeset::new(&self.all(), &log.all(), "");
        let mut s = String::from("");
        for c in &diffs {
            match *c {
                Difference::Same(ref z) => s = format!("{}{}", &s, &z),
                Difference::Rem(ref z) => s = format!("{}{}", &s, &z.red()),
                _ => (),
            }
        }
        s
    }
}

pub struct Logger {
    logs: Vec<Log>,
    current_index: Option<usize>,
}

impl fmt::Display for Logger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (self.current_log(), self.prev_log()) {
            (Some(now), Some(old)) => write!(f, "{}", now.diff(old)),
            (Some(now), None) => write!(f, "{}", now),
            (None, _) => write!(f, "log is empty"),
        }
    }
}

impl Logger {
    pub fn new() -> Logger {
        Logger {
            logs: vec![],
            current_index: None,
        }
    }

    pub fn current_log(&self) -> Option<&Log> {
        match self.current_index {
            Some(i) => Some(&self.logs[i]),
            None => None,
        }
    }

    pub fn prev_log(&self) -> Option<&Log> {
        match self.current_index {
            Some(i) => {
                if i == 0 {
                    return None;
                };
                return Some(&self.logs[i - 1]);
            }
            None => None,
        }
    }

    pub fn append<T: AVR>(&mut self, avr: &T) {
        let log = Log::new(
            self.processor_status(avr),
            self.registers_status(avr),
            self.memory_status(avr.sram(), 2, 0, 25),
            self.memory_status(avr.sram(), 2, 286, 288), // 0x8ff / 8 = 0d288
        );
        self.logs.push(log);
        match self.current_index {
            Some(i) => self.current_index = Some(i + 1),
            None => self.current_index = Some(0),
        }
    }

    pub fn processor_status<T: AVR>(&self, avr: &T) -> String {
        format!(
            r#"
Program Counter: {:#08x} (Hexfile = {:x})
Stack Pointer:   {:#04x}
X Register:      {:#04x}
Y Register:      {:#04x}
Z Register:      {:#04x}
Status Register: {:08b}
Cycle Counter:   {}"#,
            avr.pc(),
            avr.pc() * 2,
            avr.sp(),
            avr.get_word(avr.w().x),
            avr.get_word(avr.w().y),
            avr.get_word(avr.w().z),
            avr.get_register(avr.r().sreg),
            avr.cycle(),
        )
    }

    pub fn registers_status<T: AVR>(&self, avr: &T) -> String {
        let mut sum = String::from("");
        for i in 0..8 {
            let i = i * 4;
            let s = format!(
                "R{:02} = {:#04x}, R{:02} = {:#04x}, R{:02} = {:#04x}, R{:02} = {:#04x},",
                i,
                avr.get_register(i),
                i + 1,
                avr.get_register(i + 1),
                i + 2,
                avr.get_register(i + 2),
                i + 3,
                avr.get_register(i + 3)
            );
            sum = format!("{}\n{}", sum, s);
        }
        sum
    }

    pub fn memory_status<T>(
        &self,
        mem: &RefCell<dyn Memory<T>>,
        unit: u8,
        from: usize,
        to: usize,
    ) -> String
    where
        T: LowerHex,
    {
        let mut sum = String::from("");
        for i in from..to {
            let s;
            let i = i * 8;
            if unit == 2 {
                s = format!(
                    "{:#06x} | {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x}",
                    i,
                    mem.borrow().get(i + 0),
                    mem.borrow().get(i + 1),
                    mem.borrow().get(i + 2),
                    mem.borrow().get(i + 3),
                    mem.borrow().get(i + 4),
                    mem.borrow().get(i + 5),
                    mem.borrow().get(i + 6),
                    mem.borrow().get(i + 7),
                )
            } else if unit == 4 {
                s = format!(
                    "{:#06x} | {:04x} {:04x} {:04x} {:04x} {:04x} {:04x} {:04x} {:04x}",
                    i * 2,
                    mem.borrow().get(i + 0),
                    mem.borrow().get(i + 1),
                    mem.borrow().get(i + 2),
                    mem.borrow().get(i + 3),
                    mem.borrow().get(i + 4),
                    mem.borrow().get(i + 5),
                    mem.borrow().get(i + 6),
                    mem.borrow().get(i + 7),
                )
            } else {
                s = String::from("")
            };
            sum = format!("{}\n{}", sum, s);
        }
        sum
    }
}
