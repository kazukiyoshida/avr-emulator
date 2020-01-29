use std::fs::File;
use std::io::{BufRead, BufReader};
use super::avr::*;
use super::instruction::*;


pub const GENERAL_PURPOSE_REGISTER_SIZE: usize = 32;
pub const FLASH_MEMORY_SIZE: usize = 0x8000; // = 0d32768 = 32 KiB
pub const SRAM_SIZE: usize         = 0x800;  // = 0d2048  = 2 KiB
pub const EEPROM_SIZE: usize       = 0x400;  // = 0d1024  = 1 KiB


pub struct ATmega328P {
    // プログラムを保持するメモリ
    pub flash_memory: FlashMemory,

    // 汎用レジスタ・IOレジスタ空間
    pub sram: SRAM,

    pub eeprom: EEPROM,

    // Program Counter
    // WIP: is this ok..?
    pub pc: u16,
}

impl AVR for ATmega328P {
    fn pc(&self) -> u16 {
        0
    }

    fn set_pc(&mut self, v: u16) {
        self.pc = v;
    }

    fn sp(&self) -> u16 {
        0
    }

    fn gprg(&self, addr: usize) -> u8 {
        self.sram.get(addr)
    }

    fn set_gprg(&mut self, addr: usize, v: u8) {
        self.sram.set(addr, v)
    }

    fn fetch(&self) -> Word {
        let b = self.flash_memory.get(self.pc() as usize);
        Word(b)
    }

    fn word(&self) -> Word {
        Word(self.flash_memory.get(self.pc() as usize))
    }

    fn double_word(&self) -> (Word, Word)  {
        (Word(self.flash_memory.get(self.pc() as usize)),
         Word(self.flash_memory.get(self.pc() as usize +1 )))
    }
}

impl ATmega328P {
    pub fn new() -> ATmega328P {
        ATmega328P{
            flash_memory: FlashMemory::new(),
            sram: SRAM::new(),
            eeprom: EEPROM::new(),
            pc: 0,
        }
    }

    pub fn load_hex(&mut self, filepath: &str) {
        let f = File::open(filepath).expect("file not found");
        let f = BufReader::new(f);
        let mut memory_addr = 0;
        for line in f.lines() {
            let line = line.unwrap();

            // Example intel Hex file's line
            // :060040004A95E9F708955E
            let record_type = &line[7..9];
            let data = &line[9..line.len() - 2];

            if record_type != "00" { continue; }

            for list in data.chars().collect::<Vec<char>>().chunks(4) {
                let a = list[0].to_digit(16).unwrap();
                let b = list[1].to_digit(16).unwrap();
                let c = list[2].to_digit(16).unwrap();
                let d = list[3].to_digit(16).unwrap();
                self.flash_memory.set(memory_addr, ( c << 12 | d << 8 | a << 4 | b ) as u16);
                memory_addr += 1;
            }
        }
    }
}

#[test]
fn test_atmega328p() {
    let mut avr = ATmega328P::new();
    avr.load_hex("sample/led_flashing/led_flashing.ino.standard.hex");
    let w = avr.fetch();
    println!("||| w = {:016b}", w.0);

    let instr = decode_instr(w);
    match instr {
        Some(i) => exec(i, &mut avr),
        None    => println!("!!!!!!!!! panic !!!!!!!"),
    }
}

pub struct FlashMemory([u16; FLASH_MEMORY_SIZE]);
pub struct SRAM([u8; SRAM_SIZE]);
pub struct EEPROM([u8; EEPROM_SIZE]);

impl Memory<u16> for FlashMemory {
    fn get(&self, a: usize) -> u16 {
        self.0[a]
    }
    fn set(&mut self, a: usize, v: u16) {
        self.0[a] = v;
    }
}

impl FlashMemory {
    fn new() -> FlashMemory {
        FlashMemory( [0; FLASH_MEMORY_SIZE] )
    }
}

impl Memory<u8> for SRAM {
    fn get(&self, a: usize) -> u8 {
        self.0[a]
    }
    fn set(&mut self, a: usize, v: u8) {
        self.0[a] = v;
    }
}

impl SRAM {
    fn new() -> SRAM { SRAM([0; SRAM_SIZE]) }

    fn get_word(&self, a: usize) -> u16 {
        0
    }

    fn set_word(&self, a: usize, v: u16) {
    }
}

impl Memory<u8> for EEPROM {
    fn get(&self, a: usize) -> u8 {
        0_u8
    }
    fn set(&mut self, a: usize, v: u8) {
        self.0[a] = v;
    }
}

impl EEPROM {
    fn new() -> EEPROM { EEPROM([0; EEPROM_SIZE]) }
}

#[test]
fn test_flash_memory() {
    let mut m = FlashMemory::new();
    m.set(0xf, 0xffff);
    assert_eq!(m.get(0xf), 0xffff);
}

