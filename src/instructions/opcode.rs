use super::super::core::Core;
use super::{
    arithmetic,
    branch,
    data_transfer,
};

#[derive(Debug)]
struct Opcode(&'static str);

const AVR_OPCODES: [( Opcode, fn(&mut Core) -> ()); 10] = [
    (Opcode("000111rdddddrrrr"), arithmetic::adc),
    (Opcode("000011rdddddrrrr"), arithmetic::add),
    (Opcode("10010110KKddKKKK"), arithmetic::adiw),
    (Opcode("001000rdddddrrrr"), arithmetic::and),
    (Opcode("0111KKKKddddKKKK"), arithmetic::andi),
    (Opcode("1001010ddddd1010"), arithmetic::dec),
    (Opcode("1110KKKKddddKKKK"), data_transfer::ldi),
    (Opcode("10111AArrrrrAAAA"), data_transfer::out),
    (Opcode("1101kkkkkkkkkkkk"), branch::rcall),
    (Opcode("111101kkkkkkk001"), branch::brne),
];

impl Opcode {
    fn contains(&self, word: u16) -> bool {
        let s = format!("{:016b}", word);
        let word_chars = s.chars();
        let opcode_chars = self.0.chars();

        for (w, o) in word_chars.zip(opcode_chars) {
            let w_bit = w.to_digit(2).unwrap();
            match o.to_digit(2) {
                Some(o_bit) if o_bit != w_bit => return false,
                _ => (),
            }
        }
        true
    }
}

pub fn decode(word: u16) -> Option<fn(&mut Core) -> ()> {
    for (o, f) in AVR_OPCODES.iter() {
        if o.contains(word) { return Some(*f) }
    }
    None
}
