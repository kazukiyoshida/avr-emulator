extern crate avr_emulator;

use std::process;
use avr_emulator::instruction::*;
use avr_emulator::atmega328p::*;
use avr_emulator::avr::*;

pub const SAMPLE_FILE_NAME: &str = "../sample/avr_studio/led_flashing/led_flashing.hex";

fn main() {
    let mut avr = ATmega328P::new();
    avr.load_hex(SAMPLE_FILE_NAME);

    avr.flash_memory.view_memory(4u8, 20);
    std::io::stdin().read_line(&mut String::new()).ok();

    for _ in 0..10 {
        let w = avr.word();
        match decode_instr(w) {
            Some(i) => {
                avr.view_processor_status(i);
                avr.view_registers();
                avr.exec(i);
            },
            None => process::exit(1),
        }
        std::io::stdin().read_line(&mut String::new()).ok();
    }
}
