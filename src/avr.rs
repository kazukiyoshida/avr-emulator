use std::iter::IntoIterator;

pub trait AVR {
    // Program Counter
    fn pc(&self) -> u16;
    fn set_pc(&mut self, v: u16);

    // Stack Pointer
    fn sp(&self) -> u16;

    // General Purpose Register
    fn gprg(&self, addr: usize) -> u8;
    fn set_gprg(&mut self, addr: usize, v: u8);
}

pub enum Sreg { I, T, H, S, V, N, Z, C }

// 8bit, 16bit のメモリ
pub trait Memory<T> {
    fn get(&self, a: usize) -> T;
    fn set(&mut self, a: usize, v: T);
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Word(pub u16);

impl Word {
    pub fn top(&self) -> u8 {
        ( self.0 >> 8 ) as u8
    }
    pub fn low(&self) -> u8 {
        ( self.0 & 0b11111111 ) as u8
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
    let w = Word(0b01001100_11001100);
    for b in w.into_iter() {
        println!("{}", b);
    }
    let w1 = Word(0b00001111_11110000);
    println!("{:08b}", w1.top());
    println!("{:08b}", w1.low());
}
