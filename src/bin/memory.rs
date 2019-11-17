extern crate avr_emulator;
use avr_emulator::memory::{ProgramMemory, DataMemory, StatusRegister, Registers, IORegisters};

fn main() {
    println!("{:?}", ProgramMemory::new(8));
    println!("{:?}", DataMemory::new(8));
    println!("{:?}", StatusRegister::default());
    println!("{:?}", Registers::new());
    println!("{:?}", IORegisters::new());
}
