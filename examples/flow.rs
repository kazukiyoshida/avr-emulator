use avr_emulator::arch::atmega328p::*;
use avr_emulator::avrmcu::*;
use std::fs;
use std::{thread, time};

pub const SAMPLE_FILE_NAME: &str = "hex/atmel_studio/led_flashing_fast/led_flashing.hex";

fn main() {
    let hex = fs::read_to_string(SAMPLE_FILE_NAME).unwrap();
    let mut avr = ATmega328P::new(Package::PDIP28);
    avr.program(hex);
    avr.initialize();
    let mut pin18 = false;
    loop {
        avr.next();
        let next_pin18 = avr.get_pins()[18];
        if pin18 != next_pin18 {
            pin18 = next_pin18;
            println!("{}", pin18);
        }
    }
}
