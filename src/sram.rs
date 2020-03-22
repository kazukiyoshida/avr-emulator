use super::util::bit::*;

macro_rules! define_stationary_struct {
    ($structName: ident, $type: ty, $( $key: ident ),* ) => {
        #[derive(Debug)]
        pub struct $structName {
            $( pub $key: $type, )*
        }
    };
}

pub type RegisterBitAddr = (usize, u8);
#[rustfmt::skip]
define_stationary_struct!(
    RegisterBitMap,
    RegisterBitAddr,
    c, z, n, v, s, h, t, i,
    tov0, ocf0a, ocf0b, // Timer 0
    tov1, ocf1a, ocf1b, // Timer 1
    tov2, ocf2a, ocf2b  // Timer 2
);

pub type RegisterAddr = usize;
#[rustfmt::skip]
define_stationary_struct!(
    RegisterMap,
    RegisterAddr,
    sreg, sph, spl, portd, ddrd, pind, ucsr0a, ucsr0b, ucsr0c,
    portc, ddrc, pinc, portb, ddrb, pinb, ramend, mcusr, twsr, twar, twdr,
    // TODO: This may not compatible with archs except atmega328p.
    tcnt0, tccr0a, tccr0b,         ocr0a, ocr0b, timsk0, tifr0, // Timer 0 (8-bit)
           tccr1a, tccr1b, tccr1c,               timsk1, tifr1, // Timer 1 (16-bit)
    tcnt2, tccr2a, tccr2b,         ocr2a, ocr2b, timsk2, tifr2  // Timer 2 (8-bit)
);

pub type RegisterWordAddr = (usize, usize);
#[rustfmt::skip]
define_stationary_struct!(
    RegisterWordMap,
    RegisterWordAddr,
    sp, x, y, z,
    tcnt1, ocr1a, ocr1b, icr1 // timer 1 (16-bit)
);

pub struct SRAM {
    data: Vec<u8>,
    pub map: &'static RegisterMap,
    pub word_map: &'static RegisterWordMap,
    pub bit_map: &'static RegisterBitMap,
}

impl SRAM {
    pub fn new(
        size: usize,
        map: &'static RegisterMap,
        word_map: &'static RegisterWordMap,
        bit_map: &'static RegisterBitMap,
    ) -> SRAM {
        SRAM {
            data: vec![0; size],
            map: map,
            word_map: word_map,
            bit_map: bit_map,
        }
    }

    pub fn get(&self, a: usize) -> u8 {
        self.data[a]
    }

    pub fn gets(&self, a: usize, b: usize) -> (u8, u8) {
        (self.data[a], self.data[b])
    }

    pub fn set(&mut self, a: usize, v: u8) {
        self.data[a] = v;
    }

    pub fn get_bit(&self, addr: RegisterBitAddr) -> bool {
        bit(self.data[addr.0], addr.1)
    }

    pub fn set_bit(&self, addr: RegisterBitAddr, v: bool) {
        // WIP
    }

    pub fn get_word(&self, addr: RegisterWordAddr) -> u16 {
        concat(self.get(addr.0), self.get(addr.1))
    }

    pub fn set_word(&self, addr: RegisterWordAddr, v: u16) {
        // WIP
    }

    pub fn sp(&self) -> u16 {
        self.get_word(self.word_map.sp)
    }

    pub fn push_stack(&mut self, v: u8) {
        self.set(self.sp() as usize, v);
        let new_sp = self.sp() - 1;
        self.set(self.map.sph, high_byte(new_sp));
        self.set(self.map.spl, low_byte(new_sp));
    }

    pub fn pop_stack(&mut self) -> u8 {
        let v = self.get((self.sp() + 1u16) as usize);
        let new_sp = self.sp() + 1;
        self.set(self.map.sph, high_byte(new_sp));
        self.set(self.map.spl, low_byte(new_sp));
        v
    }
}
