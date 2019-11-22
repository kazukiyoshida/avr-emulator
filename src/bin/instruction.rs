extern crate avr_emulator;
use avr_emulator::instructions::opcode;

fn main() {
    let (o, f) = &opcode::AVR_OPCODES[0];
    println!("{}", o.contains(0b0001110011110000));
    println!("{:?}", o);
}
