use super::super::flash_memory::*;
use super::super::io_port::*;
use super::super::sram::*;
use super::super::timer16bit::*;
use super::super::timer8bit::*;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

const FLASH_MEMORY_SIZE: usize = 0x8000;
const SRAM_SIZE: usize = 0x900;

const REGISTER_MAP: RegisterMap = RegisterMap {
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

const REGISTER_BIT_MAP: RegisterBitMap = RegisterBitMap {
    c: (REGISTER_MAP.sreg, 0),
    z: (REGISTER_MAP.sreg, 1),
    n: (REGISTER_MAP.sreg, 2),
    v: (REGISTER_MAP.sreg, 3),
    s: (REGISTER_MAP.sreg, 4),
    h: (REGISTER_MAP.sreg, 5),
    t: (REGISTER_MAP.sreg, 6),
    i: (REGISTER_MAP.sreg, 7),

    // Timer 0
    tov0: (REGISTER_MAP.tifr0, 0),
    ocf0a: (REGISTER_MAP.tifr0, 1),
    ocf0b: (REGISTER_MAP.tifr0, 2),

    // Timer 1
    tov1: (REGISTER_MAP.tifr1, 0),
    ocf1a: (REGISTER_MAP.tifr1, 1),
    ocf1b: (REGISTER_MAP.tifr1, 2),

    // Timer 2
    tov2: (REGISTER_MAP.tifr2, 0),
    ocf2a: (REGISTER_MAP.tifr2, 1),
    ocf2b: (REGISTER_MAP.tifr2, 2),
};

const REGISTER_WORD_MAP: RegisterWordMap = RegisterWordMap {
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
    pc: Cell<u32>,
    cycle: Cell<u64>,
    sram: Rc<RefCell<SRAM>>,
    flash_memory: Rc<RefCell<FlashMemory>>,
    timer0: Timer8bit,
    timer1: Timer16bit,
    timer2: Timer8bit,
    portb: IOPort,
    portc: IOPort,
    portd: IOPort,
}

impl ATmega328P {
    pub fn new() -> ATmega328P {
        let sram = Rc::new(RefCell::new(SRAM::new(
            SRAM_SIZE,
            &REGISTER_MAP,
            &REGISTER_WORD_MAP,
            &REGISTER_BIT_MAP,
        )));

        let flash_memory = Rc::new(RefCell::new(FlashMemory::new(FLASH_MEMORY_SIZE)));

        let timer0 = Timer8bit::new(
            Timer8bitType::A,
            Rc::clone(&sram),
            sram.borrow().map.tcnt0,
            sram.borrow().map.tccr0a,
            sram.borrow().map.tccr0b,
            sram.borrow().map.ocr0a,
            sram.borrow().map.ocr0b,
            sram.borrow().bit_map.tov0,
            sram.borrow().bit_map.ocf0a,
            sram.borrow().bit_map.ocf0b,
        );

        let timer1 = Timer16bit::new(
            Rc::clone(&sram),
            sram.borrow().word_map.tcnt1,
            sram.borrow().map.tccr1a,
            sram.borrow().map.tccr1b,
            sram.borrow().map.tccr1c,
            sram.borrow().word_map.icr1,
            sram.borrow().word_map.ocr1a,
            sram.borrow().word_map.ocr1b,
            sram.borrow().bit_map.tov1,
            sram.borrow().bit_map.ocf1a,
            sram.borrow().bit_map.ocf1b,
        );

        let timer2 = Timer8bit::new(
            Timer8bitType::B,
            Rc::clone(&sram),
            sram.borrow().map.tcnt2,
            sram.borrow().map.tccr2a,
            sram.borrow().map.tccr2b,
            sram.borrow().map.ocr2a,
            sram.borrow().map.ocr2b,
            sram.borrow().bit_map.tov2,
            sram.borrow().bit_map.ocf2a,
            sram.borrow().bit_map.ocf2b,
        );

        let portb = IOPort::new(
            Rc::clone(&sram),
            sram.borrow().map.portb,
            sram.borrow().map.ddrb,
            sram.borrow().map.pinb,
        );

        let portc = IOPort::new(
            Rc::clone(&sram),
            sram.borrow().map.portc,
            sram.borrow().map.ddrc,
            sram.borrow().map.pinc,
        );

        let portd = IOPort::new(
            Rc::clone(&sram),
            sram.borrow().map.portd,
            sram.borrow().map.ddrd,
            sram.borrow().map.pind,
        );

        ATmega328P {
            pc: Cell::new(0),
            cycle: Cell::new(0),
            sram: sram,
            flash_memory: flash_memory,
            timer0: timer0,
            timer1: timer1,
            timer2: timer2,
            portb: portb,
            portc: portc,
            portd: portd,
        }
    }

    pub fn initialize_sram(&self) {
        let mut sram = self.sram.borrow_mut();
        sram.set_word(REGISTER_WORD_MAP.sp, REGISTER_MAP.ramend as u16);
        sram.set(0x12, 0x01);
        sram.set(0x16, 0x01);
        sram.set(0x18, 0x87);
        sram.set(0x1a, 0x09);
        sram.set(0x1b, 0x01);
        sram.set(0x1c, 0xff);
        sram.set(0x1d, 0x08);
        sram.set(0x1e, 0x7a);
        sram.set(REGISTER_MAP.mcusr, 0x01);
        sram.set(REGISTER_MAP.twsr, 0xf8);
        sram.set(REGISTER_MAP.twar, 0xfe);
        sram.set(REGISTER_MAP.twdr, 0xff);
        sram.set(REGISTER_MAP.ucsr0a, 0x20);
        sram.set(REGISTER_MAP.ucsr0c, 0x06);
    }
}
