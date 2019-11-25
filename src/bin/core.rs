extern crate avr_emulator;
use std::{thread, time};
use avr_emulator::core::Core;


fn main() {
    let ten_millis = time::Duration::from_millis(100);
    let now = time::Instant::now();

    let mut core = Core::new();

    println!("--- load hex file ---");
    core.load_hex("src/bin/sample.hex");
    println!("Mem");
    println!("{}", core.mem);

    println!("--- start ---");
    loop {
        println!("\n|||| core ||||");
        println!("Cycles: {:?}", core.cycles);
        println!("SP: {}", core.sp());
        println!("Registers: {:?}", core.regs());
        println!("StatusRegister: {:?}", core.sreg());
        core.next();

        thread::sleep(ten_millis);
    }
}
