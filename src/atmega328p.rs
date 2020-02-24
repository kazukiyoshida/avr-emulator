use super::avr::*;
use super::memory::*;

pub const FLASH_MEMORY_SIZE: usize = 0x8000;
pub const SRAM_SIZE: usize = 0x900;
pub const EEPROM_SIZE: usize = 0x400;

pub const REGISTER_MAP: RegisterMap = RegisterMap {
    sreg: 0x5f,
    sph: 0x5e,
    spl: 0x5d,
    ocr0b: 0x48,
    ocr0a: 0x47,
    tcnt0: 0x46,
    tccr0b: 0x45,
    tccr0a: 0x44,
    portd: 0x2b,
    ddrd: 0x2a,
    pind: 0x29,
    portc: 0x28,
    ddrc: 0x27,
    pinc: 0x26,
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
};

pub struct ATmega328P {
    pub flash_memory: FlashMemory,
    pub sram: SRAM,
    pub eeprom: EEPROM,
    pub pc: u32,
    pub cycle: u64,
}

impl AVR for ATmega328P {
    fn execute(&mut self) {
        let (_, instr_func) = self.decode_instr(self.word());
        instr_func(self);
    }

    fn flash_memory(&self) -> &dyn Memory<u16> {
        &self.flash_memory
    }

    fn flash_memory_mut(&mut self) -> &mut dyn Memory<u16> {
        &mut self.flash_memory
    }

    fn sram(&self) -> &dyn Memory<u8> {
        &self.sram
    }

    fn sram_mut(&mut self) -> &mut dyn Memory<u8> {
        &mut self.sram
    }

    fn pc(&self) -> u32 {
        self.pc
    }

    fn set_pc(&mut self, v: u32) {
        self.pc = v;
    }

    fn cycle(&self) -> u64 {
        self.cycle
    }

    fn cycle_increment(&mut self, v: u64) {
        self.cycle += v;
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
            flash_memory: FlashMemory::new(),
            sram: SRAM::new(),
            eeprom: EEPROM::new(),
            pc: 0,
            cycle: 0,
        }
    }

    pub fn initialize_sram(&mut self) {
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
