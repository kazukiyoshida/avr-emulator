use avr_emulator::arch::atmega328p::*;
use avr_emulator::avrmcu::*;
use std::fmt;
use std::fs;

pub const SAMPLE_FILE_NAME: &str = "hex/atmel_studio/led_flashing_fast/led_flashing.hex";

fn screenshot<T: fmt::Display>(avr: &T) {
    println!("\x1B[2J{}", avr);
    std::io::stdin().read_line(&mut String::new()).ok();
}

fn main() {
    let hex = fs::read_to_string(SAMPLE_FILE_NAME).unwrap();
    let mut avr = ATmega328P::new(Package::PDIP28);
    avr.program(hex);
    avr.initialize();
    screenshot(&avr);

    for count in 0..10 {
        avr.next();
        screenshot(&avr);
    }
}
