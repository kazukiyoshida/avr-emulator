use avr_emulator::atmega328p::*;
use avr_emulator::avr::*;
use avr_emulator::logger::*;
use std::collections::VecDeque;
use std::env;
use std::{thread, time};

pub const SAMPLE_FILE_NAME: &str = "hex/avr_studio/led_flashing_fast/led_flashing.hex";

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
        avr.execute();
        timer0.input_clk();
        timer1.input_clk();
        timer2.input_clk();

        if s % 100 == 0 {
            println!(
                "cycle = {:10}    tcnt1 = {:6}   PORTB = {:08b}",
                avr.cycle(),
                avr.get_word(timer1.tcnt),
                portb.avr.get_register(portb.ddrx)
            );
        }

        thread::sleep(ds);
    }
}
