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
