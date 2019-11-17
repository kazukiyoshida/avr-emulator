use super::arithmetic;

fn decode(opcode: u16) -> (function) {
    match addressing_mode {
      "000111rdddddrrrr" => arithmetic::adc,
      "000111rdddddrrrr" => arithmetic::add,
      "10010110KKddKKKK" => arithmetic::adiw,
      "001000rdddddrrrr" => arithmetic::and,
      "0111KKKKddddKKKK" => arithmetic::andi,
      _ => nop,
    }
}

