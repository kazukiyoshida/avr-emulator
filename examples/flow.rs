use avr_emulator::arch::atmega328p::*;
use avr_emulator::avrmcu::*;
use std::fs;
use std::{thread, time};

pub const SAMPLE_FILE_NAME: &str = "hex/atmel_studio/led_flashing_fast/led_flashing.hex";

fn main() {
    let ds = time::Duration::from_millis(2);

    let hex = fs::read_to_string(SAMPLE_FILE_NAME).unwrap();
    let mut avr = ATmega328P::new(Package::PDIP28);
    avr.program(hex);
    avr.initialize();

    loop {
        avr.next();
        if avr.pc == 0x74 {
            println!("|||| HIGH |||||| cycle = {}", avr.cycle);
        }
        if avr.pc == 0x7e {
            println!("|||| LOW  |||||| cycle = {}", avr.cycle);
        }
        thread::sleep(ds);
    }
}
