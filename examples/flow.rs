use avr_emulator::atmega328p::*;
use avr_emulator::avr::*;
use std::{thread, time};

pub const SAMPLE_FILE_NAME: &str = "hex/atmel_studio/led_flashing_fast/led_flashing.hex";

fn main() {
    let ds = time::Duration::from_millis(1);
    let mut s = 0;

    let avr = ATmega328P::new();

    let mut timer0 = avr.new_timer0();
    let mut timer1 = avr.new_timer1();
    let mut timer2 = avr.new_timer2();

    let mut portb = avr.new_portb();
    let mut portc = avr.new_portc();
    let mut portd = avr.new_portd();

    avr.load_hex(SAMPLE_FILE_NAME);
    avr.initialize_sram();

    loop {
        s += 1;

        if avr.pc() == 0x115 {
            println!(
                "cycle = {:8}, PC : 0x115  ||  0x12 = {:4x}    0x16 = {:4x}    0x17 = {:4x}    0x18 = {:4x}",
                avr.cycle(),
                avr.get_register(0x12),
                avr.get_register(0x16),
                avr.get_register(0x17),
                avr.get_register(0x18),
            );
        }

        if avr.pc() == 0x74 {
            println!("|||||||||||||||||||||||||||| digital Write  HIGH |||||||||||||||||||||||||||||| cycle = {}", avr.cycle());
        }

        if avr.pc() == 0x7e {
            println!("|||||||||||||||||||||||||||| digital Write  LOW  |||||||||||||||||||||||||||||| cycle = {}", avr.cycle());
        }

        avr.execute();
        timer0.clk_io();
        timer1.clk_io();
        timer2.clk_io();
        portb.clk_io();
        portc.clk_io();
        portd.clk_io();

        // if s % 100 == 0 {
        //     println!("cycle = {:10}    PORTB = {}", avr.cycle(), portb,);
        // }

        thread::sleep(ds);
    }
}
