use super::instruction::*;
use super::utils::*;
use super::word::*;
use std::fmt::LowerHex;

pub trait AVR {
    fn execute(&mut self);
    fn run(&mut self, max_cycle: u64);

    fn flash_memory(&self) -> &dyn Memory<u16>;
    fn sram(&self) -> &dyn Memory<u8>;

    // Program Counter
    // AVR has 16 or 22 bit program counter.
    // ATmega328p is 16bit PC machine. We need to implement 22bit models soon.
    fn pc(&self) -> u32;

    fn set_pc(&mut self, v: u32);

    fn pc_increment(&mut self, diff: u32) {
        self.set_pc(self.pc() + diff);
    }

    // Stack Pointer
    fn sp(&self) -> u16;

    fn push_stack(&mut self, v: u8);

    fn pop_stack(&mut self) -> u8;

    fn push_pc_stack(&mut self, v: u32) {
        // WIP: ATmega328p is 16bit Program Counter machine...
        let w = (v & 0xffff) as u16;
        self.push_stack(high_bit(w));
        self.push_stack(low_bit(w));
    }

    fn pop_pc_stack(&mut self) -> u16 {
        let l = self.pop_stack();
        let h = self.pop_stack();
        concat(h, l)
    }

    // General Purpose Register
    // 先頭の 0~31 要素が汎用レジスタに当たる.
    fn gprg(&self, addr: usize) -> u8;

    fn set_gprg(&mut self, addr: usize, v: u8);

    fn gprgs(&self, addr1: usize, addr2: usize) -> (u8, u8) {
        (self.gprg(addr1), self.gprg(addr2))
    }

    fn preg_addresses(&self, x: Preg) -> (usize, usize) {
        match x {
            Preg::X => (27, 26),
            Preg::Y => (29, 28),
            Preg::Z => (31, 30),
        }
    }

    fn preg(&self, x: Preg) -> u16 {
        let (h, l) = self.preg_addresses(x);
        concat(self.gprg(h), self.gprg(l))
    }

    fn set_preg(&mut self, x: Preg, v: u16) {
        let (h_addr, l_addr) = self.preg_addresses(x);
        self.set_gprg(h_addr, high_bit(v));
        self.set_gprg(l_addr, low_bit(v));
    }

    // Cycle Counter
    fn cycle(&self) -> u64;

    fn cycle_increment(&mut self, v: u64);

    // Fetch 1 word from Program Memory.
    // Program Memory has ~0x8000 address, this is coverd by u16(~0xffff).
    fn fetch(&self, p: u32) -> u16;

    fn word(&self) -> Word {
        Word(self.fetch(self.pc()))
    }

    fn double_word(&self) -> (Word, Word) {
        (Word(self.fetch(self.pc())), Word(self.fetch(self.pc() + 1)))
    }

    fn sreg(&self) -> u8;

    fn status(&self, s: Sreg) -> bool;

    fn set_status(&mut self, s: Sreg, v: bool);

    fn set_status_by_arithmetic_instruction(&mut self, d: u8, r: u8, res: u8) {
        // WIP: Updating algorithm of status bit is not optimized
        self.set_status(Sreg::H, has_borrow_from_bit3(d, r, res));
        self.set_status(Sreg::V, has_2complement_overflow(d, r, res));
        self.set_status(Sreg::N, msb(res));
        self.set_status(Sreg::Z, res == 0);
        self.signed_test();
    }

    fn set_status_by_bit_instruction(&mut self, res: u8) {
        // WIP: Updating algorithm of status bit is not optimized
        self.set_status(Sreg::V, false);
        self.set_status(Sreg::N, msb(res));
        self.set_status(Sreg::Z, res == 0);
        self.signed_test();
    }

    fn signed_test(&mut self) {
        let s = self.status(Sreg::V) ^ self.status(Sreg::N);
        self.set_status(Sreg::S, s);
    }

    fn z_program_memory(&self) -> u8 {
        let z_addr = self.preg(Preg::Z);
        if z_addr % 2 == 0 {
            let addr = z_addr / 2;
            low_bit(self.fetch(addr as u32))
        } else {
            let addr = ( z_addr - 1 ) / 2;
            high_bit(self.fetch(addr as u32))
        }
    }
}

// Status Register
#[derive(Eq, PartialEq, Debug)]
pub enum Sreg {
    C,
    Z,
    N,
    V,
    S,
    H,
    T,
    I,
}

// Pointer Register
#[derive(Eq, PartialEq, Debug)]
pub enum Preg {
    X,
    Y,
    Z,
}

pub trait Memory<T>
where
    T: LowerHex,
{
    fn get(&self, a: usize) -> T;
    fn set(&mut self, a: usize, v: T);
}
