// use avr_emulator::arch::atmega328p::*;
// use avr_emulator::avrmcu::*;
// use std::fs;
//
// pub const SAMPLE_FILE_NAME: &str = "hex/atmel_studio/led_flashing_fast/led_flashing.hex";
//
fn main() {}
//     let hex = fs::read_to_string(SAMPLE_FILE_NAME).unwrap();
//     let mut avr = ATmega328P::new();
//     avr.program(hex);
//     avr.initialize();
//     for count in 0..10 {
//         println!("\x1B[2J{}", avr);
//         avr.next();
//         std::io::stdin().read_line(&mut String::new()).ok();
//     }
// }
