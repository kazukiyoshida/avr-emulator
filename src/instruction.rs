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

pub struct InstructionCode(pub Instruction, pub u16, pub u16);

// Instruction Object is created here, and in application, every instruction instance is
// reffered this object.
const INSTRUCTION_CODE_MAP: [InstructionCode; 2] = [
    InstructionCode(Instruction::ADC, 0b0001_1100_0000_0000, 0b1111_1100_0000_0000),
    InstructionCode(Instruction::ADC, 0b0001_1100_0000_0000, 0b1111_1100_0000_0000),
];

pub fn is_bit_match(w: Word, code: u16, mask: u16) -> bool {
    true
}

pub fn decode_instruction(w: Word) -> Option<&'static Instruction> {
    for InstructionCode(instruction, code, mask) in &INSTRUCTION_CODE_MAP {
        if is_bit_match(w, *code, *mask) {
            return Some(instruction)
        }
    }
    None
}

pub fn exec<T: AVR>(i: Instruction, avr: &mut T, w: Word) {
    match i {
        Instruction::ADC   => adc(avr, w),
        Instruction::LDI   => ldi(avr, w),
        Instruction::OUT   => out(avr, w),
        Instruction::NOP   => nop(avr, w),
        Instruction::RCALL => rcall(avr, w),
    };
}

pub fn adc<T: AVR>(avr: &mut T, w: Word) {
    let (r_addr, d_addr) = operand55(w);
    let r = avr.gprg(r_addr as usize);
    let d = avr.gprg(d_addr as usize);
    avr.set_gprg(r_addr as usize, r+d);
}

pub fn ldi<T: AVR>(avr: &mut T, w: Word) {
    let (k, d_addr) = operand84(w);
    avr.set_gprg(d_addr as usize + 16, k);
}

pub fn out<T: AVR>(avr: &mut T, w: Word) {
    let (a, r) = operand65(w);
    avr.set_gprg(a as usize, r);
}

pub fn nop<T: AVR>(_: &mut T, _: Word) {
}

pub fn rcall<T: AVR>(avr: &mut T, w: Word) {
    let k = operand12(w);
    let pc = avr.pc();
    avr.set_pc(pc+k+1);
}

