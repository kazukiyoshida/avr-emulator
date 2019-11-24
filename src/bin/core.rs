extern crate avr_emulator;
use avr_emulator::core::Core;

fn main() {
    let mut core = Core::new();

    println!("--- load hex file ---");
    core.load_hex("src/bin/sample.hex");
    println!("Mem");
    println!("{}", core.mem);

    println!("--- start ---");
    loop {
        println!("\n|||| core ||||");
        println!("Cycles: {:?}", core.cycles);
        println!("SP: {:?}", core.sp);
        println!("Registers: {:?}", core.regs);
        println!("StatusRegister: {:?}", core.sreg);
        core.next();
    }
}
