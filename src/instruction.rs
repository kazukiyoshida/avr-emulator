use std::collections::HashMap;
use itertools::izip;
use lazy_static::lazy_static;
use super::avr::*;
use super::utils::*;


#[derive(Eq, PartialEq, Hash, Debug)]
pub enum Instr {
    ADC,         //
    LDI, OUT,    //
    NOP,         //
    RCALL,       //
}

pub struct Opcode(pub u16, pub u16);

lazy_static! {
    static ref OPCODE_MAP: HashMap<Instr, Opcode> = {
        let mut m = HashMap::new();
        m.insert(Instr::ADC,   Opcode(0b0001_1100_0000_0000, 0b1111_1100_0000_0000));
        m.insert(Instr::LDI,   Opcode(0b1110_0000_0000_0000, 0b1111_0000_0000_0000));
        m.insert(Instr::OUT,   Opcode(0b1011_1000_0000_0000, 0b1111_1000_0000_0000));
        m.insert(Instr::NOP,   Opcode(0b0000_0000_0000_0000, 0b1111_1111_1111_1111));
        m.insert(Instr::RCALL, Opcode(0b1101_0000_0000_0000, 0b1111_0000_0000_0000));
        m
    };
}

pub fn is_decoded(word: Word, code: &Opcode) -> bool {
    for (w, c, m) in izip!(word, Word(code.0), Word(code.1)) {
        if m && ( w != c ) {
            return false
        }
    }
    true
}

#[test]
pub fn test_is_decoded() {
    assert_eq!(
        true,
        is_decoded(
            Word(0b0001_1100_0000_0000),
            &Opcode(0b0001_1100_0000_0000, 0b1111_1100_0000_0000)
        )
    );
    assert_eq!(
        false,
        is_decoded(
            Word(0b0011_1100_0000_0000),
            &Opcode(0b0001_1100_0000_0000,0b1111_1100_0000_0000)
        )
    );
}

pub fn decode_instr(w: Word) -> Option<&'static Instr> {
    for (instr, code) in OPCODE_MAP.iter() {
        if is_decoded(w, code) {
            return Some(instr)
        }
    }
    None
}

#[test]
pub fn test_decode_instr() {
    assert_eq!(Some(&Instr::ADC), decode_instr(Word(0b0001_1100_0111_0101)));
    assert_eq!(Some(&Instr::OUT), decode_instr(Word(0b1011_1110_0111_0101)));
    assert_eq!(Some(&Instr::NOP), decode_instr(Word(0b0000_0000_0000_0000)));
    assert_eq!(None,              decode_instr(Word(0b1111_1111_1111_1111)));
}

pub fn exec<T: AVR>(i: &Instr, avr: &mut T, w: Word) {
    match i {
        &Instr::ADC   => adc(avr, w),
        &Instr::LDI   => ldi(avr, w),
        &Instr::OUT   => out(avr, w),
        &Instr::NOP   => nop(avr, w),
        &Instr::RCALL => rcall(avr, w),
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

