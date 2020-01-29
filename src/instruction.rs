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
    JMP,
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
        m.insert(Instr::JMP,   Opcode(0b1001_0100_0000_1100, 0b1111_1110_0000_1110));
        m
    };
}

pub fn is_decoded(word: Word, code: &Opcode) -> bool {
    for (word_bit, code_bit, mask_bit) in izip!(word, Word(code.0), Word(code.1)) {
        if mask_bit && ( word_bit != code_bit ) {
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

pub fn exec<T: AVR>(i: &Instr, avr: &mut T) {
    println!("||| ^-- instruction : {:?}", i);
    match i {
        &Instr::ADC   => adc(avr),
        &Instr::LDI   => ldi(avr),
        &Instr::OUT   => out(avr),
        &Instr::NOP   => nop(avr),
        &Instr::RCALL => rcall(avr),
        &Instr::JMP   => jmp(avr),
    };
}

pub fn adc<T: AVR>(avr: &mut T) {
    let w = avr.word();
    let (r_addr, d_addr) = operand55(w);
    let r = avr.gprg(r_addr as usize);
    let d = avr.gprg(d_addr as usize);
    avr.set_gprg(r_addr as usize, r+d);
}

pub fn ldi<T: AVR>(avr: &mut T) {
    let w = avr.word();
    let (k, d_addr) = operand84(w);
    avr.set_gprg(d_addr as usize + 16, k);
}

pub fn out<T: AVR>(avr: &mut T) {
    let w = avr.word();
    let (a, r) = operand65(w);
    avr.set_gprg(a as usize, r);
}

pub fn nop<T: AVR>(_: &mut T) {
}

pub fn rcall<T: AVR>(avr: &mut T) {
    let w = avr.word();
    let k = operand12(w);
    let pc = avr.pc();
    avr.set_pc(pc+k+1);
}

pub fn jmp<T: AVR>(avr: &mut T) {
    let (w1, w2) = avr.double_word();
    let k = operand22(w1, w2);
    avr.set_pc(k)
}

