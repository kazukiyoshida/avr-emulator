extern crate avr_emulator;

use std::process;
use avr_emulator::instruction::*;
use avr_emulator::atmega328p::*;
use avr_emulator::avr::*;
use avr_emulator::logger::*;

pub const SAMPLE_FILE_NAME: &str = "../sample/avr_studio/led_flashing/led_flashing.hex";

fn main() {
    let mut avr = ATmega328P::new();
    avr.load_hex(SAMPLE_FILE_NAME);

    let mut logger = Logger::new();

    print!("\x1B[2J");
    println!(">>> Flash Memory \n");
    println!("{}", logger.memory_status(&avr.flash_memory, 4, 0, 20));

    std::io::stdin().read_line(&mut String::new()).ok();

    for _ in 0..30 {
        let w = avr.word();
        match decode_instr(w) {
            Some(i) => {
                logger.append(&avr);

                print!("\x1B[2J");
                println!("{}", logger);

                avr.exec(i);
            },
            None => process::exit(1),
        }
        std::io::stdin().read_line(&mut String::new()).ok();
    }
}
