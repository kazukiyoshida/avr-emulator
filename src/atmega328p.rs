use std::fs::File;
use std::io::{BufRead, BufReader};
use super::avr::*;
use super::instruction::*;
use super::utils::*;


pub const GENERAL_PURPOSE_REGISTER_SIZE: usize = 32;
pub const FLASH_MEMORY_SIZE: usize = 0x8000; // = 0d32768 = 32 KiB. 16bit(~0xffff)で表現可能.
pub const SRAM_SIZE: usize = 0xffff;          // = 0d2048  = 2 KiB
pub const EEPROM_SIZE: usize = 0x400;        // = 0d1024  = 1 KiB
pub const STATUS_REGISTER: usize = 0x5f;
pub const STACK_POINTER_H: usize = 0x5e;
pub const STACK_POINTER_L: usize = 0x5d;
pub const RAMEND: u16 = 0x08ff;


pub struct ATmega328P {
    pub flash_memory: FlashMemory,
    pub sram: SRAM,
    pub eeprom: EEPROM,
    pub pc: u32,
}

impl AVR for ATmega328P {
    fn pc(&self) -> u32 {
        self.pc
    }

    fn set_pc(&mut self, v: u32) {
        self.pc = v;
    }

    fn sp(&self) -> u16 {
        concat(self.sram.get(STACK_POINTER_H), self.sram.get(STACK_POINTER_L))
    }

    fn push_stack(&mut self, v: u8) {
        self.set_gprg(self.sp() as usize, v);
        let new_sp = self.sp() - 1;
        self.sram.set(STACK_POINTER_H, high_bit(new_sp));
        self.sram.set(STACK_POINTER_L, low_bit(new_sp));
    }

    fn pop_stack(&mut self) -> u8 {
        let v = self.gprg((self.sp()+1u16) as usize);
        let new_sp = self.sp() + 1;
        self.sram.set(STACK_POINTER_H, high_bit(new_sp));
        self.sram.set(STACK_POINTER_L, low_bit(new_sp));
        v
    }

    fn gprg(&self, addr: usize) -> u8 {
        self.sram.get(addr)
    }

    fn set_gprg(&mut self, addr: usize, v: u8) {
        self.sram.set(addr, v)
    }

    fn xyz_reg_addresses(&self, x: XYZReg) -> (usize, usize) {
        match x {
            X => (27, 26),
            Y => (29, 28),
            Z => (31, 30),
        }
    }

    fn fetch(&self, p: u32) -> u16 {
        self.flash_memory.get(p as usize)
    }

    fn status(&self, s: Sreg) -> bool {
        bit(self.sram.0[STATUS_REGISTER], self.status_register_map(s))
    }

    fn set_status(&mut self, s: Sreg, v: bool) {
        let n = self.status_register_map(s);
        if v { 
            self.sram.0[STATUS_REGISTER] = self.sram.0[STATUS_REGISTER] | ( 1 << n );
        } else { 
            self.sram.0[STATUS_REGISTER] = self.sram.0[STATUS_REGISTER] ^ ( 1 << n );
        };
    }
}

impl AVRInstruction for ATmega328P {}

impl ATmega328P {
    pub fn new() -> ATmega328P {
        let mut sram = SRAM::new();
        sram.set_word(STACK_POINTER_L, RAMEND);
        ATmega328P{
            flash_memory: FlashMemory::new(),
            sram: sram,
            eeprom: EEPROM::new(),
            pc: 0,
        }
    }

    pub fn status_register_map(&self, s: Sreg) -> u8 {
        match s {
            Sreg::I => 7,
            Sreg::T => 6,
            Sreg::H => 5,
            Sreg::S => 4,
            Sreg::V => 3,
            Sreg::N => 2,
            Sreg::Z => 1,
            Sreg::C => 0,
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

