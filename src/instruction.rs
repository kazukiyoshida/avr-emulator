use std::collections::HashMap;
use super::avr::*;
use super::utils::*;


#[derive(Eq, PartialEq, Hash)]
pub enum Instruction {
    ADC,
    LDI, OUT,
    NOP,
    RCALL,
}

// (code, mask) // if ( word & mask ) == code then ..
// pub type instruction_code = (u16, u16);
// ADC = ( 0b0001_1100_0000_0000, 0b1111_1100_0000_0000 )

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

