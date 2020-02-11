use std::iter::IntoIterator;
use super::utils::*;
use super::word::*;

pub trait AVR {
    // Program Counter 
    // AVR has 16 or 22 bit program counter.
    // ATmega328p is 16bit PC machine. We need to implement 22bit models soon.
    fn pc(&self) -> u32;
    fn set_pc(&mut self, v: u32);
    fn pc_increment(&mut self) {
        self.set_pc(self.pc() + 1);
    }
    fn pc_double_increment(&mut self) {
        self.set_pc(self.pc() + 2);
    }

    // Stack Pointer
    fn sp(&self) -> u16;
    fn push_stack(&mut self, v: u8);
    fn pop_stack(&mut self) -> u8;

    fn push_pc_stack(&mut self, v: u32) {
        // WIP: ATmega328p is 16bit Program Counter machine...
        let w = ( v & 0xffff ) as u16;
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

    fn xyz_reg_addresses(&self, x: XYZReg) -> (usize, usize);

    fn xyz_reg(&self, x: XYZReg) -> u16 {
        let (h, l) = self.xyz_reg_addresses(x);
        concat(self.gprg(h), self.gprg(l))
    }

    fn set_xyz_reg(&mut self, x: XYZReg, v: u16) {
        let (h_addr, l_addr) = self.xyz_reg_addresses(x);
        self.set_gprg(h_addr, high_bit(v));
        self.set_gprg(l_addr, low_bit(v));
    }

    // Fetch 1 word from Program Memory.
    // Program Memory has ~0x8000 address, this is coverd by u16(~0xffff).
    fn fetch(&self, p: u32) -> u16;

    fn word(&self) -> Word {
        Word(self.fetch(self.pc()))
    }
    fn double_word(&self) -> (Word, Word) {
        (Word(self.fetch(self.pc())),
         Word(self.fetch(self.pc()+1)))
    }

    fn status(&self, s: Sreg) -> bool;
    fn set_status(&mut self, s: Sreg, v: bool);

    fn signed_test(&mut self) {
        let s = self.status(Sreg::V) ^ self.status(Sreg::N);
        self.set_status(Sreg::S, s);
    }
}

pub enum Sreg { I, T, H, S, V, N, Z, C }
pub enum XYZReg { X, Y, Z }

pub trait Memory<T> {
    fn get(&self, a: usize) -> T;
    fn set(&mut self, a: usize, v: T);
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Word(pub u16);

impl Word {
    pub fn high(&self) -> u8 {
        high_bit(self.0)
    }
    pub fn low(&self) -> u8 {
        low_bit(self.0)
    }
}

pub struct WordIter {
    seeker: u8,
    word: u16,
}

impl IntoIterator for Word {
    type Item = bool;
    type IntoIter = WordIter;
    fn into_iter(self) -> Self::IntoIter {
        WordIter {
            seeker: 0,
            word: self.0,
        }
    }
}

impl Iterator for WordIter {
    type Item = bool;

    // Seek each bit from right to left.
    fn next(&mut self) -> Option<bool> {
        if self.seeker >= 16 {
            return None
        }
        let bit = ( self.word & ( 1 << self.seeker )) >> self.seeker;
        self.seeker += 1;
        Some(bit == 1)
    }
}

#[test]
fn test_u8_word() {
    let w = Word(0b00001111_11110000);
    assert_eq!(0b00001111, w.high());
    assert_eq!(0b11110000, w.low());
}
