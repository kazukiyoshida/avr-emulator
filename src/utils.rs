use super::avr::*;

pub fn operand55(w: Word) -> (u8, u8) {
    (operand(w.0, 0b0000001000001111) as u8,
     operand(w.0, 0b0000000111110000) as u8)
}

pub fn operand65(w: Word) -> (u8, u8) {
    (operand(w.0, 0b0000_0110_0000_1111) as u8,
     operand(w.0, 0b0000_0001_1111_0000) as u8)
}

pub fn operand84(w: Word) -> (u8, u8) {
    (operand(w.0, 0b0000111100001111) as u8,
     operand(w.0, 0b0000000011110000) as u8)
}

pub fn operand12(w: Word) -> u16 {
    operand(w.0, 0b0000_1111_1111_1111)
}

fn operand(word: u16, mask: u16) -> u16 {
    let mut k = 0;
    Word(word)
        .into_iter()
        .zip(Word(mask).into_iter())
        .fold(0, |mut s, (word_bit, mask_bit)| {
            if mask_bit && word_bit { s = s | ( 1 << k ) }
            if mask_bit             { k += 1; }
            s
    })
}

#[test]
fn test_operand() {
    assert_eq!(
        operand(
            0b1111_1111_1111_1111,
            0b0000_1111_0000_1111),
            0b0000_0000_1111_1111
    );
    assert_eq!(
        operand(
            0b0000_0000_0000_0000,
            0b0000_1111_0000_1111),
            0b0000_0000_0000_0000
    );
    assert_eq!(
        operand(
            0b1111_1001_1111_0110,
            0b0000_1111_0000_1101),
            0b0000_0000_0100_1010
    );
}
