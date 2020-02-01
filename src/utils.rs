use super::avr::*;

// r, d
pub fn operand55(w: Word) -> (u8, u8) {
    (operand(w.0, 0b0000001000001111) as u8,
     operand(w.0, 0b0000000111110000) as u8)
}

// d, r
pub fn operand44(w: Word) -> (u8, u8) {
    (operand(w.0, 0b0000000011110000) as u8,
     operand(w.0, 0b0000000000001111) as u8)
}

pub fn operand65(w: Word) -> (u8, u8) {
    (operand(w.0, 0b0000_0110_0000_1111) as u8,
     operand(w.0, 0b0000_0001_1111_0000) as u8)
}

pub fn operand84(w: Word) -> (u8, u8) {
    (operand(w.0, 0b0000111100001111) as u8,
     operand(w.0, 0b0000000011110000) as u8)
}

pub fn operand53(w: Word) -> (u8, u8) {
    (operand(w.0, 0b0000000011111000) as u8,
     operand(w.0, 0b0000000000000111) as u8)
}

pub fn operand7(w: Word) -> u8 {
    operand(w.0, 0b0000001111111000) as u8
}

pub fn operand5(w: Word) -> u8 {
    operand(w.0, 0b0000000111110000) as u8
}

pub fn operand12(w: Word) -> u16 {
    operand(w.0, 0b0000_1111_1111_1111)
}

pub fn operand22(w1: Word, w2: Word) -> u32 {
    ( ( operand(w1.0, 0b0000_0001_1111_0001) as u32 ) << 16 ) | w2.0 as u32
}

#[test]
fn test_operand22() {
    assert_eq!(
        operand22(Word(0b1001_0100_0000_1110), Word(0b0000_0001_1100_1100)),
        0b111001100
    );
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

pub fn msb(b: u8) -> bool {
    b >> 7 == 1
}

pub fn lsb(b: u8) -> bool {
    b & 1 == 1
}

#[test]
pub fn test_msb_lsb() {
    assert_eq!(msb(0b11110000), true);
    assert_eq!(msb(0b01110000), false);
    assert_eq!(lsb(0b01110001), true);
    assert_eq!(lsb(0b01110000), false);
}

pub fn has_borrow_from_bit3(a: u8, b: u8, r: u8) -> bool {
    let (a3, b3, r3) = (bit(a,3), bit(b, 3), bit(r, 3));
    !a3 & b3 | b3 & r3 | r3 & !a3
}

pub fn has_borrow_from_msb(a: u8, b: u8, r: u8) -> bool {
    let (a7, b7, r7) = (bit(a, 7), bit(b, 7), bit(r, 7));
    a7 & b7 | b7 & !r7 | !r7 & a7
}

pub fn has_2complement_overflow(a: u8, b: u8, r: u8) -> bool {
    let (a7, b7, r7) = (bit(a,7), bit(b, 7), bit(r, 7));
    a7 & !b7 & !r7 | !a7 & b7 & r7
}

pub fn bit(a: u8, n: u8) -> bool {
    ( ( a & 1 << n ) >> n ) == 1
}

pub fn high_bit(w: u16) -> u8 {
    ( w >> 8 ) as u8
}

pub fn low_bit(w: u16) -> u8 {
    ( w & 0b11111111 ) as u8
}

pub fn concat(a: u8, b: u8) -> u16 {
    ( a as u16 ) << 8 | b as u16
}

// Parameter k is represented in two's complement form.
pub fn add_in_twos_complement_form(s: u8, k: u8) -> u8 {
    ( ( s + k ) & 0b1111111 ) + 1u8
}

// This calculate relative destination, - 63 < destination < pc + 64.
// cf. http://kccn.konan-u.ac.jp/information/cs/cyber03/cy3_hum.htm
pub fn add_7bits_in_twos_complement_form(pc: u32, k: u8) -> u32 {
    // 0xFF.. is number lower 7 bits is 0, others is 1
    pc & 0xFFFFFF80_u32 | ( pc + k as u32 ) & 0b111_1111_u32
}

// This calculate relative destination, - 2048 < k in two's complement < +2047.
pub fn add_12bits_in_twos_complement_form(pc: u32, k: u16) -> u32 {
    // 0xFF.. is number lower 12 bits is 0, others is 1
    pc & 0xFFFFF000_u32 | ( pc + k as u32 ) & 0b1111_1111_1111_u32
}

#[test]
fn test_add_in_twos_complement_form() {
    // 100 + 3
    assert_eq!(103, add_7bits_in_twos_complement_form(100u32, 0b11_u8));
    // 100 - 4
    assert_eq!(96, add_7bits_in_twos_complement_form(100u32, 0b111_1100_u8));
    // 511 - 4 
    assert_eq!(511-4, add_7bits_in_twos_complement_form(0b1_1111_1111_u32, 0b111_1100_u8));

    // 100 + 3
    assert_eq!(103, add_12bits_in_twos_complement_form(100u32, 0b11_u16));
    // 100 - 4
    assert_eq!(96, add_12bits_in_twos_complement_form(100u32, 0b1111_1111_1100_u16));
    // 16383 - 4
    assert_eq!(16383-4, add_12bits_in_twos_complement_form(0b11_1111_1111_1111_u32, 0b1111_1111_1100_u16));
}

#[test]
fn test_bit() {
    assert!(!bit(0b1100, 0));
    assert!(!bit(0b1100, 1));
    assert!( bit(0b1100, 2));
    assert!( bit(0b1100, 3));
    assert_eq!(high_bit(0b1100_0011_0011_1100), 0b1100_0011);
    assert_eq!( low_bit(0b1100_0011_0011_1100), 0b0011_1100);
    assert_eq!(concat(0b1100_0011, 0b01111_0000), 0b1100_0011_1111_0000);
}

