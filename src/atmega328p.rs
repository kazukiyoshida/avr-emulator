
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
}

impl ATmega328P {
    fn new() -> ATmega328P {
        ATmega328P{
            flash_memory: FlashMemory::new(),
            sram: SRAM::new(),
            eeprom: EEPROM::new(),
            pc: 0,
        }
    }
}


// メモリサイズ
pub const GENERAL_PURPOSE_REGISTER_SIZE: usize = 32;
pub const FLASH_MEMORY_SIZE: usize = 0x8000; // = 0d32768 = 32 KiB
pub const SRAM_SIZE: usize         = 0x800;  // = 0d2048  = 2 KiB
pub const EEPROM_SIZE: usize       = 0x400;  // = 0d1024  = 1 KiB

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

