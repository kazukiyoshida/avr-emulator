extern crate avr_emulator;
use avr_emulator::core::Core;

fn main() {
    let mut core = Core::new();
    println!("{:?}", core);
    core.next();
}
