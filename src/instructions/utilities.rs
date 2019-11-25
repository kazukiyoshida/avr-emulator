// get_bits(0b01101, 0b01110) -> 0b110
// get_bits(0b01101, 0b00110) -> 0b10
// get_bits(0b01101, 0b00111) -> 0b101
// get_bits(0b01101, 0b01011) -> 0b101
//
fn get_bits(n: u16, mask: u16) -> u16 {
    let ns = format!("{:016b}", n);
    let ms = format!("{:016b}", mask);
    let n_chars = ns.chars();
    let m_chars = ms.chars();
    let mut res: Vec<u16> = vec![];
    for (nb, mb) in n_chars.zip(m_chars) {
        if mb.to_digit(2).unwrap() == 1 {
            res.push(nb.to_digit(2).unwrap() as u16);
        }
    }
    let mut x = 0;
    for (i, r) in res.into_iter().rev().enumerate() {
        x = x | ( r << i );
    }
    x
}

pub fn operand_r5d5(word: u16) -> (u16, u16) {
    (get_bits(word, 0b0000001000001111),
     get_bits(word, 0b0000000111110000))
}

pub fn operand_K8d4(word: u16) -> (u8, u8) {
    (get_bits(word, 0b0000111100001111) as u8, // K
     get_bits(word, 0b0000000011110000) as u8) // d
}

pub fn operand_A6r5(word: u16) -> (u8, u8) {
    (get_bits(word, 0b0000_0110_0000_1111) as u8, // A
     get_bits(word, 0b0000_0001_1111_0000) as u8) // r
}

pub fn operand_k12(word: u16) -> u16 {
    get_bits(word, 0b0000_1111_1111_1111)
}

pub fn operand_d5(word: u16) -> u16 {
    get_bits(word, 0b0000_0001_1111_0000)
}

pub fn operand_k7(word: u16) -> i8 {
    let n = get_bits(word, 0b0000_0011_1111_1000);
    -1 * ( 0b1000_0000 - n ) as i8
}

pub fn operand_to_mem_index(i: u8) -> usize {
    let n = i + 16;
    assert!(16 <= n && n <= 31);
    n as usize
}

