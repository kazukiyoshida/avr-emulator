use avr_emulator::atmega328p::*;
use avr_emulator::avr::*;
use avr_emulator::logger::*;
use std::collections::VecDeque;
use std::env;

pub const SAMPLE_FILE_NAME: &str = "hex/avr_studio/led_flashing_fast/led_flashing.hex";

fn main() {
    let args: Vec<String> = env::args().collect();
    let skip_to_cycle: u64 = if args.len() > 1 {
        args[1].parse().ok().unwrap_or(0)
    } else {
        0
    };

    let avr = ATmega328P::new();

    let mut timer0 = avr.new_timer0();
    let mut timer1 = avr.new_timer1();
    let mut timer2 = avr.new_timer2();

    let mut portb = avr.new_portb();
    let mut portc = avr.new_portc();
    let mut portd = avr.new_portd();

    let mut logger = Logger::new();

    avr.load_hex(SAMPLE_FILE_NAME);
    avr.initialize_sram();

    // Flash Memory Check
    print!("\x1B[2J");
    println!(">>> Flash Memory \n");
    println!("{}", logger.memory_status(avr.flash_memory(), 4, 0, 20));
    std::io::stdin().read_line(&mut String::new()).ok();

    for c in 0..1000000 {
        if c % 20000 == 0 {
            println!("c = {}", c);
        }

        if avr.cycle() < skip_to_cycle {
            avr.execute();
            timer0.clk_io();
            timer1.clk_io();
            timer2.clk_io();
            portb.clk_io();
            portc.clk_io();
            portd.clk_io();
        } else {
            let (instr, instr_func) = avr.decode_instr(avr.word());
            logger.append(&avr);

            print!("\x1B[2J");
            println!("{}", logger);
            println!(
                "\nNext Instruction : {:?} (PC: {:x} -> {:016b} = {:04x})",
                &instr,
                avr.pc(),
                avr.word().0,
                avr.word().0
            );

            println!("");
            println!("Timer 0 {}", timer0);
            println!("Timer 1 {}", timer1);
            println!("Timer 2 {}", timer2);

            println!("Port B   {}", portb);
            println!("Port C   {}", portc);
            println!("Port D   {}", portd);

            instr_func(&avr);
            timer0.clk_io();
            timer1.clk_io();
            timer2.clk_io();
            portb.clk_io();
            portc.clk_io();
            portd.clk_io();

            std::io::stdin().read_line(&mut String::new()).ok();
        }
    }
}
