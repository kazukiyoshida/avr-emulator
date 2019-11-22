extern crate avr_emulator;
use avr_emulator::core::Core;

fn main() {
    let mut core = Core::new();

    // 実験用にメモリ0番地にプログラムをセット
    // adc:  0b0001110011110000
    // add:  0b0000110011110000
    // andi: 0b0111110011110000
    core.mem.set(0, 0b0000110011110000);
    core.regs.data[0] = 0b1000;
    core.regs.data[15] = 0b11;

    println!("|||| core |||| {:?}", core);
    core.next();
    println!("|||| core |||| {:?}", core);
}
