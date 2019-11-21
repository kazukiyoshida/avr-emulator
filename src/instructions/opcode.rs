use super::arithmetic;
use super::super::core::Core;

#[derive(Debug)]
pub struct Opcode(&'static str);

const AVR_OPCODES: [( Opcode, fn(&mut Core) -> ()); 5] = [
    (Opcode("000111rdddddrrrr"), arithmetic::adc),
    (Opcode("000111rdddddrrrr"), arithmetic::add),
    (Opcode("10010110KKddKKKK"), arithmetic::adiw),
    (Opcode("001000rdddddrrrr"), arithmetic::and),
    (Opcode("0111KKKKddddKKKK"), arithmetic::andi),
];

impl Opcode {
    fn contains(&self, word: u16) -> bool {
        let opcode_chars = self.0.chars();
        let s = format!("{:016b}", word);
        let word_chars = s.chars();

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
