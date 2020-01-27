
pub fn operand55(word: u16) -> (u8, u8) {
    (operand(word, 0b0000001000001111) as u8,
     operand(word, 0b0000000111110000) as u8)
}

pub fn operand65(word: u16) -> (u8, u8) {
    (operand(word, 0b0000_0110_0000_1111) as u8,
     operand(word, 0b0000_0001_1111_0000) as u8)
}

pub fn operand84(word: u16) -> (u8, u8) {
    (operand(word, 0b0000111100001111) as u8,
     operand(word, 0b0000000011110000) as u8)
}

pub fn operand12(word: u16) -> u16 {
    operand(word, 0b0000_1111_1111_1111)
}

fn operand(word: u16, mask: u16) -> u16 {
    let mut m = 0;
    let mut b: u16 = 0;
    for n in 0..16 {
        let isOpen = (( mask & ( 1 << n ) ) >> n) == 1;
        let isOn = (( word & ( 1 << n ) ) >> n) == 1;
        if isOpen {
            if isOn {
                b = b | ( 1 << m )
            }
            m += 1;
        }
    }
    b
}

#[test]
fn test_operand() {
    assert_eq!(operand(0b1111_1111_1111_1111, 0b0000_1111_0000_1111), 0b0000_0000_1111_1111);
    assert_eq!(operand(0b0000_0000_0000_0000, 0b0000_1111_0000_1111), 0b0000_0000_0000_0000);
    assert_eq!(operand(0b1111_1001_1111_0110, 0b0000_1111_0000_1101), 0b0000_0000_0100_1010);
}
