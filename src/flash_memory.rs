use super::util::bit::*;
use super::word::*;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct FlashMemory {
    data: Vec<u16>,
}

impl FlashMemory {
    pub fn new(size: usize) -> FlashMemory {
        FlashMemory {
            data: vec![0; size],
        }
    }

    pub fn get(&self, a: usize) -> u16 {
        self.data[a]
    }

    pub fn set(&mut self, a: usize, v: u16) {
        self.data[a] = v;
    }

    pub fn word(&self, pc: usize) -> Word {
        Word(self.get(pc))
    }

    pub fn double_word(&self, pc: usize) -> (Word, Word) {
        (self.word(pc), self.word(pc + 1))
    }

    pub fn z_program_memory(&self, z_addr: u16) -> u8 {
        if z_addr % 2 == 0 {
            let addr = z_addr / 2;
            low_byte(self.get(addr as usize))
        } else {
            let addr = (z_addr - 1) / 2;
            high_byte(self.get(addr as usize))
        }
    }

    // Example intel Hex file's line
    // :100000 | 00 | 0C945C000C946E000C946E000C946E00 | CA
    pub fn load_hex(&mut self, filepath: &str) {
        let f = File::open(filepath).expect("file not found");
        let f = BufReader::new(f);

        let mut memory_addr = 0;

        for line in f.lines() {
            let line = line.unwrap();

            let record_type = &line[7..9];
            if record_type != "00" {
                continue;
            }

            let data = &line[9..line.len() - 2];
            // TODO: need refactoring
            for list in data.chars().collect::<Vec<char>>().chunks(4) {
                if list.len() != 4 {
                    continue;
                }
                let a = list[0].to_digit(16).unwrap();
                let b = list[1].to_digit(16).unwrap();
                let c = list[2].to_digit(16).unwrap();
                let d = list[3].to_digit(16).unwrap();
                self.set(memory_addr, (a << 12 | b << 8 | c << 4 | d) as u16);
                memory_addr += 1;
            }
        }
    }

    pub fn load_hex_from_string(&mut self, hex: String) {
        let hex_lines: Vec<&str> = hex.split("\n").collect();
        let mut memory_addr = 0;
        for line in hex_lines {
            if line.len() < 11 {
                continue;
            }

            let record_type = &line[7..9];
            if record_type != "00" {
                continue;
            }

            // TODO: need refactoring
            let data = &line[9..line.len() - 2];
            for list in data.chars().collect::<Vec<char>>().chunks(4) {
                if list.len() != 4 {
                    continue;
                }
                let a = list[0].to_digit(16).unwrap();
                let b = list[1].to_digit(16).unwrap();
                let c = list[2].to_digit(16).unwrap();
                let d = list[3].to_digit(16).unwrap();
                self.set(memory_addr, (a << 12 | b << 8 | c << 4 | d) as u16);
                memory_addr += 1;
            }
        }
    }
}

impl fmt::Display for FlashMemory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut sum = String::from("");
        for i in 0..20 {
            let i = i * 8;
            let s = format!(
                "{:#06x} | {:04x} {:04x} {:04x} {:04x} {:04x} {:04x} {:04x} {:04x}",
                i * 2,
                self.get(i + 0),
                self.get(i + 1),
                self.get(i + 2),
                self.get(i + 3),
                self.get(i + 4),
                self.get(i + 5),
                self.get(i + 6),
                self.get(i + 7),
            );
            sum = format!("{}\n{}", sum, s);
        }
        write!(f, "{}", sum)
    }
}
