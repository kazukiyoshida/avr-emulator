use super::util::bit::*;
use std::fmt;

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

    pub fn push_pc_stack(&mut self, pc: usize) {
        let w = (pc & 0xffff) as u16;
        self.push_stack(high_byte(w));
        self.push_stack(low_byte(w));
    }

    pub fn pop_pc_stack(&mut self) -> u16 {
        let l = self.pop_stack();
        let h = self.pop_stack();
        concat(h, l)
    }

    pub fn set_status_by_arithmetic_instruction(&mut self, d: u8, r: u8, res: u8) {
        self.set_bit(self.bit_map.h, has_borrow_from_bit3(d, r, res));
        self.set_bit(self.bit_map.v, has_2complement_overflow(d, r, res));
        self.set_bit(self.bit_map.n, msb(res));
        self.set_bit(self.bit_map.z, res == 0);
        self.set_bit(self.bit_map.s, self.signed_test());
    }

    pub fn set_status_by_arithmetic_instruction2(&mut self, d: u8, k: u8, res: u8) {
        self.set_bit(self.bit_map.h, has_borrow_from_bit3_k(d, k, res));
        self.set_bit(self.bit_map.v, has_2complement_overflow_2(d, k, res));
        self.set_bit(self.bit_map.n, msb(res));
        self.set_bit(self.bit_map.z, res == 0);
        self.set_bit(self.bit_map.s, self.signed_test());
    }

    pub fn set_status_by_bit_instruction(&mut self, res: u8) {
        self.set_bit(self.bit_map.v, false);
        self.set_bit(self.bit_map.n, msb(res));
        self.set_bit(self.bit_map.z, res == 0);
        self.set_bit(self.bit_map.s, self.signed_test());
    }

    pub fn signed_test(&self) -> bool {
        self.get_bit(self.bit_map.v) ^ self.get_bit(self.bit_map.n)
    }
}

impl fmt::Display for SRAM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut sum = String::from("");
        for i in 0..20 {
            let i = i * 8;
            let s = format!(
                "{:#06x} | {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x}",
                i,
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
