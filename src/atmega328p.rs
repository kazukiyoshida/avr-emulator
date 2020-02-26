use super::avr::*;
use super::io_port::*;
use super::memory::*;
use super::timer::*;
use std::cell::{Cell, RefCell};

pub const FLASH_MEMORY_SIZE: usize = 0x8000;
pub const SRAM_SIZE: usize = 0x900;
pub const EEPROM_SIZE: usize = 0x400;

pub const REGISTER_MAP: RegisterMap = RegisterMap {
    sreg: 0x5f,
    sph: 0x5e,
    spl: 0x5d,

    // Timer 0 (8-bit)
    tcnt0: 0x46,
    tccr0a: 0x44,
    tccr0b: 0x45,
    ocr0a: 0x47,
    ocr0b: 0x48,
    timsk0: 0x6e,
    tifr0: 0x35,

    // Timer 1 (16-bit)
    tccr1a: 0x80,
    tccr1b: 0x81,
    tccr1c: 0x82,
    timsk1: 0x6f,
    tifr1: 0x36,

    // Timer 2 (8-bit)
    tcnt2: 0xb2,
    tccr2a: 0xb0,
    tccr2b: 0xb1,
    ocr2a: 0xb3,
    ocr2b: 0xb4,
    timsk2: 0x70,
    tifr2: 0x37,

    // PORT D
    portd: 0x2b,
    ddrd: 0x2a,
    pind: 0x29,

    // PORT C
    portc: 0x28,
    ddrc: 0x27,
    pinc: 0x26,

    // PORT B
    portb: 0x25,
    ddrb: 0x24,
    pinb: 0x23,

    ramend: 0x08ff,
    mcusr: 0x54,
    twsr: 0xb9,
    twar: 0xba,
    twdr: 0xbb,
    ucsr0a: 0xc0,
    ucsr0b: 0xc1,
    ucsr0c: 0xc2,
};

pub const REGISTER_BIT_MAP: RegisterBitMap = RegisterBitMap {
    c: (REGISTER_MAP.sreg, 0),
    z: (REGISTER_MAP.sreg, 1),
    n: (REGISTER_MAP.sreg, 2),
    v: (REGISTER_MAP.sreg, 3),
    s: (REGISTER_MAP.sreg, 4),
    h: (REGISTER_MAP.sreg, 5),
    t: (REGISTER_MAP.sreg, 6),
    i: (REGISTER_MAP.sreg, 7),
};

pub const REGISTER_WORD_MAP: RegisterWordMap = RegisterWordMap {
    sp: (REGISTER_MAP.sph, REGISTER_MAP.spl),
    x: (27, 26),
    y: (29, 28),
    z: (31, 30),

    // Timer 1 (16-bit)
    tcnt1: (0x85, 0x84),
    ocr1a: (0x89, 0x88),
    ocr1b: (0x8b, 0x8a),
    icr1: (0x87, 0x86),
};

pub struct ATmega328P {
    flash_memory: RefCell<FlashMemory>,
    sram: RefCell<SRAM>,
    eeprom: RefCell<EEPROM>,
    pc: Cell<u32>,
    cycle: Cell<u64>,
}

impl AVR for ATmega328P {
    fn execute(&self) {
        let (_, instr_func) = self.decode_instr(self.word());
        instr_func(self);
    }

    fn flash_memory(&self) -> &RefCell<dyn Memory<u16>> {
        &self.flash_memory
    }

    fn sram(&self) -> &RefCell<dyn Memory<u8>> {
        &self.sram
    }

    fn pc(&self) -> u32 {
        self.pc.get()
    }

    fn set_pc(&self, v: u32) {
        self.pc.set(v);
    }

    fn cycle(&self) -> u64 {
        self.cycle.get()
    }

    fn cycle_increment(&self, dc: u64) {
        let c = self.cycle.get();
        self.cycle.set(c + dc);
    }

    fn register_map(&self) -> &'static RegisterMap {
        &REGISTER_MAP
    }

    fn register_bit_map(&self) -> &'static RegisterBitMap {
        &REGISTER_BIT_MAP
    }

    fn register_word_map(&self) -> &'static RegisterWordMap {
        &REGISTER_WORD_MAP
    }
}

impl ATmega328P {
    pub fn new() -> ATmega328P {
        ATmega328P {
            flash_memory: RefCell::new(FlashMemory::new()),
            sram: RefCell::new(SRAM::new()),
            eeprom: RefCell::new(EEPROM::new()),
            pc: Cell::new(0),
            cycle: Cell::new(0),
        }
    }

    pub fn new_timer0(&self) -> Timer8bit {
        Timer8bit::new(
            self,
            REGISTER_MAP.tcnt0,
            REGISTER_MAP.tccr0a,
            REGISTER_MAP.ocr0a,
            REGISTER_MAP.ocr0b,
        )
    }

    pub fn new_timer1(&self) -> Timer16bit {
        Timer16bit::new(
            self,
            REGISTER_WORD_MAP.tcnt1,
            REGISTER_MAP.tccr1a,
            REGISTER_MAP.tccr1b,
            REGISTER_MAP.tccr1c,
            REGISTER_WORD_MAP.ocr1a,
            REGISTER_WORD_MAP.ocr1b,
        )
    }

    pub fn new_timer2(&self) -> Timer8bit {
        Timer8bit::new(
            self,
            REGISTER_MAP.tcnt2,
            REGISTER_MAP.tccr2a,
            REGISTER_MAP.ocr2a,
            REGISTER_MAP.ocr2b,
        )
    }

    pub fn new_portb(&self) -> IOPort {
        IOPort::new(
            self,
            REGISTER_MAP.portb,
            REGISTER_MAP.ddrb,
            REGISTER_MAP.pinb,
        )
    }

    pub fn new_portc(&self) -> IOPort {
        IOPort::new(
            self,
            REGISTER_MAP.portc,
            REGISTER_MAP.ddrc,
            REGISTER_MAP.pinc,
        )
    }

    pub fn new_portd(&self) -> IOPort {
        IOPort::new(
            self,
            REGISTER_MAP.portd,
            REGISTER_MAP.ddrd,
            REGISTER_MAP.pind,
        )
    }

    pub fn initialize_sram(&self) {
        self.set_word(REGISTER_WORD_MAP.sp, REGISTER_MAP.ramend as u16);
        self.set_register(0x12, 0x01);
        self.set_register(0x1a, 0x09);
        self.set_register(0x1b, 0x01);
        self.set_register(0x1c, 0xff);
        self.set_register(0x1d, 0x08);
        self.set_register(REGISTER_MAP.mcusr, 0x01);
        self.set_register(REGISTER_MAP.twsr, 0xf8);
        self.set_register(REGISTER_MAP.twar, 0xfe);
        self.set_register(REGISTER_MAP.twdr, 0xff);
        self.set_register(REGISTER_MAP.ucsr0a, 0x20);
        self.set_register(REGISTER_MAP.ucsr0c, 0x06);
    }
}

pub struct FlashMemory([u16; FLASH_MEMORY_SIZE]);
pub struct SRAM([u8; SRAM_SIZE]);
pub struct EEPROM([u8; EEPROM_SIZE]);

impl Memory<u16> for FlashMemory {
    fn get(&self, a: usize) -> u16 {
        self.0[a].to_be()
    }

    fn set(&mut self, a: usize, v: u16) {
        self.0[a] = v;
    }
}

impl FlashMemory {
    fn new() -> FlashMemory {
        FlashMemory([0; FLASH_MEMORY_SIZE])
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
    fn new() -> SRAM {
        SRAM([0; SRAM_SIZE])
    }
}

impl Memory<u8> for EEPROM {
    fn get(&self, a: usize) -> u8 {
        self.0[a]
    }
    fn set(&mut self, a: usize, v: u8) {
        self.0[a] = v;
    }
}

impl EEPROM {
    fn new() -> EEPROM {
        EEPROM([0; EEPROM_SIZE])
    }
}

#[test]
fn test_flash_memory() {
    let mut m = FlashMemory::new();
    m.set(0xf, 0xffff);
    assert_eq!(m.get(0xf), 0xffff);
}
