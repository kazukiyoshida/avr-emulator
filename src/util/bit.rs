pub fn msb(b: u8) -> bool {
    b >> 7 == 1
}

pub fn lsb(b: u8) -> bool {
    b & 1 == 1
}

pub fn msb_u16(b: u16) -> bool {
    b >> 15 == 1
}

pub fn lsb_u16(b: u16) -> bool {
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
    let (a3, b3, r3) = (bit(a, 3), bit(b, 3), bit(r, 3));
    a3 & b3 | b3 & !r3 | !r3 & a3
}

pub fn has_borrow_from_bit3_k(a: u8, k: u8, r: u8) -> bool {
    let (a3, k3, r3) = (bit(a, 3), bit(k, 3), bit(r, 3));
    !a3 & k3 | k3 & r3 | r3 & !a3
}

pub fn has_borrow_from_msb(a: u8, b: u8, r: u8) -> bool {
    let (a7, b7, r7) = (bit(a, 7), bit(b, 7), bit(r, 7));
    a7 & b7 | b7 & !r7 | !r7 & a7
}

pub fn has_2complement_overflow(a: u8, b: u8, r: u8) -> bool {
    let (a7, b7, r7) = (bit(a, 7), bit(b, 7), bit(r, 7));
    a7 & b7 & !r7 | !a7 & !b7 & r7
}

pub fn has_2complement_overflow_2(a: u8, b: u8, r: u8) -> bool {
    let (a7, b7, r7) = (bit(a, 7), bit(b, 7), bit(r, 7));
    a7 & !b7 & !r7 | !a7 & b7 & r7
}

pub fn bit(a: u8, n: u8) -> bool {
    ((a & 1 << n) >> n) == 1
}

pub fn nth_bit_from_left_u16(a: u16, n: u8) -> bool {
    let index = 15 - n;
    ((a & 1 << index) >> index) == 1
}

#[test]
fn test_nth_bit_from_left_u16() {
    assert_eq!(true, nth_bit_from_left_u16(0b1111_1111_0000_0000, 0));
    assert_eq!(true, nth_bit_from_left_u16(0b1111_1111_0000_0000, 7));
    assert_eq!(false, nth_bit_from_left_u16(0b1111_1111_0000_0000, 8));
    assert_eq!(false, nth_bit_from_left_u16(0b1111_1111_0000_0000, 15));
}

pub fn high_byte(w: u16) -> u8 {
    (w >> 8) as u8
}

pub fn low_byte(w: u16) -> u8 {
    (w & 0b11111111) as u8
}

pub fn concat(a: u8, b: u8) -> u16 {
    (a as u16) << 8 | b as u16
}

// Parameter k is represented in two's complement form.
pub fn add_in_twos_complement_form(s: u8, k: u8) -> u8 {
    ((s + k) & 0b1111111) + 1u8
}

// This calculate relative destination, - 63 < destination < pc + 64.
// cf. http://kccn.konan-u.ac.jp/information/cs/cyber03/cy3_hum.htm
pub fn add_7bits_in_twos_complement_form(pc: u32, k: u8) -> u32 {
    // 0xFF.. is number lower 7 bits is 0, others is 1
    pc & 0xFFFFFF80_u32 | (pc + k as u32) & 0b111_1111_u32
}

// This calculate relative destination, - 2048 < k in two's complement < +2047.
pub fn add_12bits_in_twos_complement_form(pc: u32, k: u16) -> u32 {
    // 0xFF.. is number lower 12 bits is 0, others is 1
    pc & 0xFFFFF000_u32 | (pc + k as u32) & 0b1111_1111_1111_u32
}

#[test]
fn test_add_in_twos_complement_form() {
    // 100 + 3
    assert_eq!(103, add_7bits_in_twos_complement_form(100u32, 0b11_u8));
    // 100 - 4
    assert_eq!(96, add_7bits_in_twos_complement_form(100u32, 0b111_1100_u8));
    // 511 - 4
    assert_eq!(
        511 - 4,
        add_7bits_in_twos_complement_form(0b1_1111_1111_u32, 0b111_1100_u8)
    );
    // 0x105 - 0x6
    // assert_eq!(
    //     0x105 - 0x6,
    //     add_7bits_in_twos_complement_form(0x105_u32, 0x7a_u8)
    // );

    // 100 + 3
    assert_eq!(103, add_12bits_in_twos_complement_form(100u32, 0b11_u16));
    // 100 - 4
    assert_eq!(
        96,
        add_12bits_in_twos_complement_form(100u32, 0b1111_1111_1100_u16)
    );
    // 16383 - 4
    assert_eq!(
        16383 - 4,
        add_12bits_in_twos_complement_form(0b11_1111_1111_1111_u32, 0b1111_1111_1100_u16)
    );
}

#[test]
fn test_bit() {
    assert!(!bit(0b1100, 0));
    assert!(!bit(0b1100, 1));
    assert!(bit(0b1100, 2));
    assert!(bit(0b1100, 3));
    assert_eq!(high_byte(0b1100_0011_0011_1100), 0b1100_0011);
    assert_eq!(low_byte(0b1100_0011_0011_1100), 0b0011_1100);
    assert_eq!(concat(0b1100_0011, 0b01111_0000), 0b1100_0011_1111_0000);
}
