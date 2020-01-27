use std::collections::HashMap;
use super::avr::*;


#[derive(Eq, PartialEq, Hash)]
pub enum Instruction {
    ADC,
    LDI, OUT,
    NOP,
    RCALL,
}

pub fn decode_instruction(w: u16) -> Option<Instruction> {
    Some(Instruction::ADC)
}

pub fn exec<T: AVR>(i: Instruction, avr: &mut T, w: u16) {
    match i {
        Instruction::ADC   => adc(avr, w),
        Instruction::LDI   => ldi(avr, w),
        Instruction::OUT   => out(avr, w),
        Instruction::NOP   => nop(avr, w),
        Instruction::RCALL => rcall(avr, w),
    };
}

pub fn adc<T: AVR>(avr: &mut T, word: u16) {
    let (r_addr, d_addr) = operand55(word);
    let r = avr.gprg(r_addr as usize);
    let d = avr.gprg(d_addr as usize);
    avr.set_gprg(r_addr as usize, r+d);
}

pub fn ldi<T: AVR>(avr: &mut T, word: u16) {
    let (k, d_addr) = operand84(word);
    avr.set_gprg(d_addr as usize + 16, k);
}

pub fn out<T: AVR>(avr: &mut T, word: u16) {
    let (a, r) = operand65(word);
    avr.set_gprg(a as usize, r);
}

pub fn nop<T: AVR>(_: &mut T, _: u16) {
}

pub fn rcall<T: AVR>(avr: &mut T, word: u16) {
    let k = operand12(word);
    let pc = avr.pc();
    avr.set_pc(pc+k+1);
}

pub fn operand55(word: u16) -> (u8, u8) {
    (operand(word, 0b0000001000001111) as u8,
     operand(word, 0b0000000111110000) as u8)
}

pub fn operand65(word: u16) -> (u8, u8) {
    (operand(word, 0b0000_0110_0000_1111) as u8,
     operand(word, 0b0000_0001_1111_0000) as u8)
}

pub fn operand84(word: u16) -> (u8, u8) {
    (operand(word, 0b0000111100001111) as u8,
     operand(word, 0b0000000011110000) as u8)
}

pub fn operand12(word: u16) -> u16 {
    operand(word, 0b0000_1111_1111_1111)
}

fn operand(word: u16, mask: u16) -> u16 {
    let mut m = 0;
    let mut b: u16 = 0;
    for n in 0..16 {
        let isOpen = (( mask & ( 1 << n ) ) >> n) == 1;
        let isOn = (( word & ( 1 << n ) ) >> n) == 1;
        if isOpen {
            if isOn {
                b = b | ( 1 << m )
            }
            m += 1;
        }
    }
    b
}

#[test]
fn test_operand() {
    assert_eq!(operand(0b1111_1111_1111_1111, 0b0000_1111_0000_1111), 0b0000_0000_1111_1111);
    assert_eq!(operand(0b0000_0000_0000_0000, 0b0000_1111_0000_1111), 0b0000_0000_0000_0000);
    assert_eq!(operand(0b1111_1001_1111_0110, 0b0000_1111_0000_1101), 0b0000_0000_0100_1010);
}
