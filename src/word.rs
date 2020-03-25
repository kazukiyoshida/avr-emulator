use std::iter::IntoIterator;

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Word(pub u16);

impl Word {
    // r, d
    pub fn operand55(&self) -> (usize, usize) {
        (
            operand(self.0, 0b0000001000001111) as usize,
            operand(self.0, 0b0000000111110000) as usize,
        )
    }

    // d, r
    // operand is 1 left shifted
    pub fn operand44(&self) -> (usize, usize) {
        (
            (operand(self.0, 0b0000000011110000) * 2) as usize,
            (operand(self.0, 0b0000000000001111) * 2) as usize,
        )
    }

    // I/O Register starts from 0x20(0d32), so there is offset.
    pub fn operand65(&self) -> (usize, usize) {
        (
            (operand(self.0, 0b0000_0110_0000_1111) + 0x20) as usize,
            operand(self.0, 0b0000_0001_1111_0000) as usize,
        )
    }

    // d_addr = {24, 26, 28, 30}
    pub fn operand62(&self) -> (u8, usize) {
        (
            operand(self.0, 0b0000_0000_1100_1111) as u8,
            (operand(self.0, 0b0000_0000_0011_0000) * 2 + 24) as usize,
        )
    }

    // there is a 16 addr offset
    pub fn operand84(&self) -> (u8, usize) {
        (
            operand(self.0, 0b0000111100001111) as u8,
            (operand(self.0, 0b0000000011110000) + 16) as usize,
        )
    }

    pub fn operand53(&self) -> (u8, u8) {
        (
            operand(self.0, 0b0000000011111000) as u8,
            operand(self.0, 0b0000000000000111) as u8,
        )
    }

    pub fn operand7(&self) -> u8 {
        operand(self.0, 0b0000001111111000) as u8
    }

    pub fn operand5(&self) -> usize {
        operand(self.0, 0b0000000111110000) as usize
    }

    pub fn operand10(&self) -> u16 {
        operand(self.0, 0b0000_0011_1111_1111)
    }

    pub fn operand12(&self) -> u16 {
        operand(self.0, 0b0000_1111_1111_1111)
    }

    pub fn operand22(&self, w: Word) -> u32 {
        ((operand(self.0, 0b0000_0001_1111_0001) as u32) << 16) | w.0 as u32
    }
}

#[test]
fn test_word() {
    let w = Word(0b1001_0100_0000_1110);
    assert_eq!(w.operand22(Word(0b0000_0001_1100_1100)), 0b111001100);
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
            return None;
        }
        let bit = (self.word & (1 << self.seeker)) >> self.seeker;
        self.seeker += 1;
        Some(bit == 1)
    }
}

pub fn operand(word: u16, mask: u16) -> u16 {
    let mut k = 0;
    Word(word)
        .into_iter()
        .zip(Word(mask).into_iter())
        .fold(0, |mut s, (word_bit, mask_bit)| {
            if mask_bit && word_bit {
                s = s | (1 << k)
            }
            if mask_bit {
                k += 1;
            }
            s
        })
}

#[test]
fn test_operand() {
    assert_eq!(
        operand(0b1111_1111_1111_1111, 0b0000_1111_0000_1111),
        0b0000_0000_1111_1111
    );
    assert_eq!(
        operand(0b0000_0000_0000_0000, 0b0000_1111_0000_1111),
        0b0000_0000_0000_0000
    );
    assert_eq!(
        operand(0b1111_1001_1111_0110, 0b0000_1111_0000_1101),
        0b0000_0000_0100_1010
    );
}
