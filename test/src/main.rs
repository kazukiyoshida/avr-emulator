extern crate avr_emulator;

use std::env;
use std::process;
use avr_emulator::instruction::*;
use avr_emulator::atmega328p::*;
use avr_emulator::avr::*;
use avr_emulator::logger::*;

pub const SAMPLE_FILE_NAME: &str = "../sample/avr_studio/led_flashing_fast/led_flashing.hex";

// メモ: CliDebuggable trait を定義して、「1step実行」「早送り」「巻き戻し」
//       などの interface を定義するのが良さそう.
//       （今は取り急ぎ L チカを優先する）
fn main() {
    let args: Vec<String> = env::args().collect();

    let skip_to_cycle: u64 = match &args[1].parse().ok() {
        Some(n) => *n,
        None => 0,
    };

    let mut avr = ATmega328P::new();
    avr.load_hex(SAMPLE_FILE_NAME);

    // Registers in AtmelStudio Simulator have initial data.
    avr.set_gprg(0x12, 0x01);
    avr.set_gprg(0x1a, 0x09);
    avr.set_gprg(0x1b, 0x01);
    avr.set_gprg(0x1c, 0xff);
    avr.set_gprg(0x1d, 0x08);

    avr.set_gprg(0x54, 0x01);

    avr.set_gprg(0xb9, 0xf8);
    avr.set_gprg(0xba, 0xfe);
    avr.set_gprg(0xbb, 0xff);
    avr.set_gprg(0xc0, 0x20);
    avr.set_gprg(0xc2, 0x06);

    let mut logger = Logger::new();

    // Flash Memory Check
    // print!("\x1B[2J");
    // println!(">>> Flash Memory \n");
    // println!("{}", logger.memory_status(&avr.flash_memory, 4, 0, 20));
    // std::io::stdin().read_line(&mut String::new()).ok();

    let mut portb = 0;
    let mut ddrb = 0;
    let mut pinb = 0;

    for c in 0..1000000 {
        if c % 20000 == 0 {
            println!("c = {}", c);
        }

        if portb != avr.sram().get(0x25) {
            portb = avr.sram().get(0x25);
            println!("|||||| PORTB : {:x}", portb);
        }
        if ddrb != avr.sram().get(0x24) {
            ddrb = avr.sram().get(0x24);
            println!("|||||| DDRB : {:x}", ddrb);
        }
        if pinb != avr.sram().get(0x23) {
            pinb = avr.sram().get(0x23);
            println!("|||||| PINB : {:x}", pinb);
        }

        match decode_instr(avr.word()) {
            Some(i) => {
                if avr.cycle <= skip_to_cycle {
                    exec(&mut avr, i);
                } else {
                    logger.append(&avr);
                    print!("\x1B[2J");
                    println!("{}", logger);
                    println!("Next Instruction : {:?} ({:016b})", &i, avr.word().0);
                    exec(&mut avr, i);
                    std::io::stdin().read_line(&mut String::new()).ok();
                }
            },
            None => {
                println!("instruction decode failed: {:016b}", &avr.word().0);
                process::exit(1);
            },
        }
    }
}
