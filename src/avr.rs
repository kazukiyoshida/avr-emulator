use super::instruction::*;
use super::memory::*;
use super::opcode_tree::*;
use super::utils::*;
use super::word::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub trait AVR {
    fn register_map(&self) -> &'static RegisterMap;
    fn register_bit_map(&self) -> &'static RegisterBitMap;
    fn register_word_map(&self) -> &'static RegisterWordMap;

    fn flash_memory(&self) -> &dyn Memory<u16>;
    fn flash_memory_mut(&mut self) -> &mut dyn Memory<u16>;
    fn sram(&self) -> &dyn Memory<u8>;
    fn sram_mut(&mut self) -> &mut dyn Memory<u8>;

    fn pc(&self) -> u32;
    fn set_pc(&mut self, v: u32);

    fn cycle(&self) -> u64;
    fn cycle_increment(&mut self, v: u64);

    fn execute(&mut self);

    fn decode_instr(&mut self, word: Word) -> (Instr, InstrFunc) {
        OPCODE_TREE.with(|tree| tree.find(word.0))
    }

    fn run(&mut self, max_cycle: u64) {
        while self.cycle() < max_cycle {
            self.execute();
        }
    }

    fn get_register(&self, addr: RegisterAddr) -> u8 {
        self.sram().get(addr)
    }

    fn get_registers(&self, addr1: RegisterAddr, addr2: RegisterAddr) -> (u8, u8) {
        (self.sram().get(addr1), self.sram().get(addr2))
    }

    fn set_register(&mut self, addr: RegisterAddr, v: u8) {
        self.sram_mut().set(addr, v);
    }

    fn get_bit(&self, addr: RegisterBitAddr) -> bool {
        (self.sram().get(addr.0) & (1 << addr.1)) >> addr.1 == 1
    }

    fn set_bit(&mut self, addr: RegisterBitAddr, v: bool) {
        let old = self.sram().get(addr.0);
        if v {
            self.sram_mut().set(addr.0, old | (1 << addr.1));
        } else {
            self.sram_mut().set(addr.0, old & !(1 << addr.1));
        }
    }

    fn get_word(&self, addr: RegisterWordAddr) -> u16 {
        concat(self.get_register(addr.0), self.get_register(addr.1))
    }

    fn set_word(&mut self, addr: RegisterWordAddr, v: u16) {
        self.set_register(addr.0, high_byte(v));
        self.set_register(addr.1, low_byte(v));
    }

    // alias
    fn r(&self) -> &'static RegisterMap {
        self.register_map()
    }

    // alias
    fn b(&self) -> &'static RegisterBitMap {
        self.register_bit_map()
    }

    // alias, wip
    fn w(&self) -> &'static RegisterWordMap {
        self.register_word_map()
    }

    fn pc_increment(&mut self, diff: u32) {
        self.set_pc(self.pc() + diff);
    }

    fn sp(&self) -> u16 {
        self.get_word(self.w().sp)
    }

    fn push_stack(&mut self, v: u8) {
        self.set_register(self.sp() as usize, v);
        let new_sp = self.sp() - 1;
        let r = self.r();
        self.sram_mut().set(r.sph, high_byte(new_sp));
        self.sram_mut().set(r.spl, low_byte(new_sp));
    }

    fn pop_stack(&mut self) -> u8 {
        let v = self.get_register((self.sp() + 1u16) as usize);
        let new_sp = self.sp() + 1;
        let r = self.r();
        self.sram_mut().set(r.sph, high_byte(new_sp));
        self.sram_mut().set(r.spl, low_byte(new_sp));
        v
    }

    fn push_pc_stack(&mut self, v: u32) {
        let w = (v & 0xffff) as u16;
        self.push_stack(high_byte(w));
        self.push_stack(low_byte(w));
    }

    fn pop_pc_stack(&mut self) -> u16 {
        let l = self.pop_stack();
        let h = self.pop_stack();
        concat(h, l)
    }

    fn fetch(&self, p: u32) -> u16 {
        self.flash_memory().get(p as usize)
    }

    fn word(&self) -> Word {
        Word(self.fetch(self.pc()))
    }

    fn double_word(&self) -> (Word, Word) {
        (Word(self.fetch(self.pc())), Word(self.fetch(self.pc() + 1)))
    }

    // WIP: Updating algorithm of status bit is not optimized
    fn set_status_by_arithmetic_instruction(&mut self, d: u8, r: u8, res: u8) {
        self.set_bit(self.b().h, has_borrow_from_bit3(d, r, res));
        self.set_bit(self.b().v, has_2complement_overflow(d, r, res));
        self.set_bit(self.b().n, msb(res));
        self.set_bit(self.b().z, res == 0);
        self.set_bit(self.b().s, self.signed_test());
    }

    fn set_status_by_bit_instruction(&mut self, res: u8) {
        self.set_bit(self.b().v, false);
        self.set_bit(self.b().n, msb(res));
        self.set_bit(self.b().z, res == 0);
        self.set_bit(self.b().s, self.signed_test());
    }

    fn signed_test(&self) -> bool {
        self.get_bit(self.b().v) ^ self.get_bit(self.b().n)
    }

    fn z_program_memory(&self) -> u8 {
        let z_addr = self.get_word(self.w().z);
        if z_addr % 2 == 0 {
            let addr = z_addr / 2;
            low_byte(self.fetch(addr as u32))
        } else {
            let addr = (z_addr - 1) / 2;
            high_byte(self.fetch(addr as u32))
        }
    }

    fn load_hex(&mut self, filepath: &str) {
        let f = File::open(filepath).expect("file not found");
        let f = BufReader::new(f);
        let mut memory_addr = 0;
        for line in f.lines() {
            let line = line.unwrap();

            // Example intel Hex file's line
            // :060040004A95E9F708955E
            let record_type = &line[7..9];
            let data = &line[9..line.len() - 2];

            if record_type != "00" {
                continue;
            }

            for list in data.chars().collect::<Vec<char>>().chunks(4) {
                let a = list[0].to_digit(16).unwrap();
                let b = list[1].to_digit(16).unwrap();
                let c = list[2].to_digit(16).unwrap();
                let d = list[3].to_digit(16).unwrap();
                self.flash_memory_mut()
                    .set(memory_addr, (a << 12 | b << 8 | c << 4 | d) as u16);
                memory_addr += 1;
            }
        }
    }
}

macro_rules! define_stationary_struct {
    ($structName: ident, $type: ty, $( $key: ident ),* ) => {
        #[derive(Debug)]
        pub struct $structName {
            $( pub $key: $type, )*
        }
    };
}

type RegisterBitAddr = (usize, u8);
#[rustfmt::skip]
define_stationary_struct!(
    RegisterBitMap,
    RegisterBitAddr,
    c, z, n, v, s, h, t, i
);

type RegisterAddr = usize;
#[rustfmt::skip]
define_stationary_struct!(
    RegisterMap,
    RegisterAddr,
    sreg, sph, spl, ocr0b, ocr0a, tcnt0, tccr0b, tccr0a, portd, ddrd, pind,
    portc, ddrc, pinc, portb, ddrb, pinb, ramend, mcusr, twsr, twar, twdr,
    ucsr0a, ucsr0b, ucsr0c
);

type RegisterWordAddr = (usize, usize);
#[rustfmt::skip]
define_stationary_struct!(
    RegisterWordMap,
    RegisterWordAddr,
    sp, x, y, z
);
