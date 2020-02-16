use std::process;
use super::avr::*;
use super::utils::*;
use super::word::*;
use itertools::izip;
use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub enum Instr {
    ADD,
    ADC,
    SUB,
    SBC,
    SUBI,
    SBCI,
    SBIW,
    DEC,
    COM,
    LD1,
    LD2,
    LD3,
    LDI,
    LDDY1,
    LDDY2,
    LDDY3,
    LDDZ1,
    LDDZ2,
    LDDZ3,
    LDS,
    OUT,
    IN,
    NOP,
    CALL,
    RCALL,
    ROL,
    LSL,
    JMP,
    RJMP,
    AND,
    ANDI,
    OR,
    EOR,
    ORI,
    STS,
    ST1,
    ST2,
    ST3,
    STY1,
    STY2,
    STY3,
    STZ1,
    STZ2,
    STZ3,
    LPM1,
    LPM2,
    LPM3,
    CP,
    CPI,
    CPC,
    CPSE,
    BREQ,
    BRNE,
    BRCS,
    SBIS,
    SEI,
    CLI,
    RET,
    PUSH,
    POP,
    MOV,
    MOVW,
}

pub struct Opcode(pub u16, pub u16);

lazy_static! {
    static ref INSTRUCTION_32_BIT: Vec<&'static Instr> = vec![
        &Instr::CALL,
        &Instr::JMP,
        &Instr::LDS,
        &Instr::STS,
    ];

    static ref OPCODE_MAP: HashMap<Instr, Opcode> = {
        let mut m = HashMap::new();
        m.insert(
            Instr::ADD,
            Opcode(0b0000_1100_0000_0000, 0b1111_1100_0000_0000),
        );
        m.insert(
            Instr::ADC,
            Opcode(0b0001_1100_0000_0000, 0b1111_1100_0000_0000),
        );
        m.insert(
            Instr::SUB,
            Opcode(0b0001_1000_0000_0000, 0b1111_1100_0000_0000),
        );
        m.insert(
            Instr::SBC,
            Opcode(0b0000_1000_0000_0000, 0b1111_1100_0000_0000),
        );
        m.insert(
            Instr::SUBI,
            Opcode(0b0101_0000_0000_0000, 0b1111_0000_0000_0000),
        );
        m.insert(
            Instr::SBCI,
            Opcode(0b0100_0000_0000_0000, 0b1111_0000_0000_0000),
        );
        m.insert(
            Instr::SBIW,
            Opcode(0b1001_0111_0000_0000, 0b1111_1111_0000_0000),
        );
        m.insert(
            Instr::DEC,
            Opcode(0b1001_0100_0000_1010, 0b1111_1110_0000_1111),
        );
        m.insert(
            Instr::COM,
            Opcode(0b1001_0100_0000_0000, 0b1111_1110_0000_1111),
        );
        m.insert(
            Instr::LDI,
            Opcode(0b1110_0000_0000_0000, 0b1111_0000_0000_0000),
        );
        m.insert(
            Instr::LD1,
            Opcode(0b1001_0000_0000_1100, 0b1111_1110_0000_1111),
        );
        m.insert(
            Instr::LD2,
            Opcode(0b1001_0000_0000_1101, 0b1111_1110_0000_1111),
        );
        m.insert(
            Instr::LD3,
            Opcode(0b1001_0000_0000_1110, 0b1111_1110_0000_1111),
        );
        m.insert(
            Instr::LDDY1,
            Opcode(0b1000_0000_0000_1000, 0b1111_1110_0000_1111),
        );
        m.insert(
            Instr::LDDY2,
            Opcode(0b1001_0000_0000_1001, 0b1111_1110_0000_1111),
        );
        m.insert(
            Instr::LDDY3,
            Opcode(0b1001_0000_0000_1010, 0b1111_1110_0000_1111),
        );
        m.insert(
            Instr::LDDZ1,
            Opcode(0b1000_0000_0000_0000, 0b1111_1110_0000_1111),
        );
        m.insert(
            Instr::LDDZ2,
            Opcode(0b1001_0000_0000_0001, 0b1111_1110_0000_1111),
        );
        m.insert(
            Instr::LDDZ3,
            Opcode(0b1001_0000_0000_0010, 0b1111_1110_0000_1111),
        );
        m.insert(
            Instr::LDS,
            Opcode(0b1001_0000_0000_0000, 0b1111_1110_0000_1111),
        );
        m.insert(
            Instr::OUT,
            Opcode(0b1011_1000_0000_0000, 0b1111_1000_0000_0000),
        );
        m.insert(
            Instr::IN,
            Opcode(0b1011_0000_0000_0000, 0b1111_1000_0000_0000),
        );
        m.insert(
            Instr::NOP,
            Opcode(0b0000_0000_0000_0000, 0b1111_1111_1111_1111),
        );
        m.insert(
            Instr::CALL,
            Opcode(0b1001_0100_0000_1110, 0b1111_1110_0000_1110),
        );
        m.insert(
            Instr::RCALL,
            Opcode(0b1101_0000_0000_0000, 0b1111_0000_0000_0000),
        );
        m.insert(
            Instr::ROL,
            Opcode(0b0001_1100_0000_0000, 0b1111_1100_0000_0000),
        );
        m.insert(
            Instr::LSL,
            Opcode(0b0000_1100_0000_0000, 0b1111_1100_0000_0000),
        );
        m.insert(
            Instr::JMP,
            Opcode(0b1001_0100_0000_1100, 0b1111_1110_0000_1110),
        );
        m.insert(
            Instr::RJMP,
            Opcode(0b1100_0000_0000_0000, 0b1111_0000_0000_0000),
        );
        m.insert(
            Instr::ORI,
            Opcode(0b0110_0000_0000_0000, 0b1111_0000_0000_0000),
        );
        m.insert(
            Instr::AND,
            Opcode(0b0010_0000_0000_0000, 0b1111_1100_0000_0000),
        );
        m.insert(
            Instr::ANDI,
            Opcode(0b0111_0000_0000_0000, 0b1111_0000_0000_0000),
        );
        m.insert(
            Instr::OR,
            Opcode(0b0010_1000_0000_0000, 0b1111_1100_0000_0000),
        );
        m.insert(
            Instr::EOR,
            Opcode(0b0010_0100_0000_0000, 0b1111_1100_0000_0000),
        );
        m.insert(
            Instr::STS,
            Opcode(0b1001_0010_0000_0000, 0b1111_1110_0000_1111),
        );
        m.insert(
            Instr::ST1,
            Opcode(0b1001_0010_0000_1100, 0b1111_1110_0000_1111),
        );
        m.insert(
            Instr::ST2,
            Opcode(0b1001_0010_0000_1101, 0b1111_1110_0000_1111),
        );
        m.insert(
            Instr::ST3,
            Opcode(0b1001_0010_0000_1110, 0b1111_1110_0000_1111),
        );
        m.insert(
            Instr::STY1,
            Opcode(0b1000_0010_0000_1000, 0b1111_1110_0000_1111),
        );
        m.insert(
            Instr::STY2,
            Opcode(0b1001_0010_0000_1001, 0b1111_1110_0000_1111),
        );
        m.insert(
            Instr::STY3,
            Opcode(0b1001_0010_0000_1010, 0b1111_1110_0000_1111),
        );
        m.insert(
            Instr::STZ1,
            Opcode(0b1000_0010_0000_0000, 0b1111_1110_0000_1111),
        );
        m.insert(
            Instr::STZ2,
            Opcode(0b1001_0010_0000_0001, 0b1111_1110_0000_1111),
        );
        m.insert(
            Instr::STZ3,
            Opcode(0b1001_0010_0000_0010, 0b1111_1110_0000_1111),
        );
        m.insert(
            Instr::LPM1,
            Opcode(0b1001_0101_1100_1000, 0b1111_1111_1111_1111),
        );
        m.insert(
            Instr::LPM2,
            Opcode(0b1001_0000_0000_0100, 0b1111_1110_0000_1111),
        );
        m.insert(
            Instr::LPM3,
            Opcode(0b1001_0000_0000_0101, 0b1111_1110_0000_1111),
        );
        m.insert(
            Instr::CP,
            Opcode(0b0001_0100_0000_0000, 0b1111_1100_0000_0000),
        );
        m.insert(
            Instr::CPI,
            Opcode(0b0011_0000_0000_0000, 0b1111_0000_0000_0000),
        );
        m.insert(
            Instr::CPC,
            Opcode(0b0000_0100_0000_0000, 0b1111_1100_0000_0000),
        );
        m.insert(
            Instr::CPSE,
            Opcode(0b0001_0000_0000_0000, 0b1111_1100_0000_0000),
        );
        m.insert(
            Instr::BREQ,
            Opcode(0b1111_0000_0000_0001, 0b1111_1100_0000_0111),
        );
        m.insert(
            Instr::BRNE,
            Opcode(0b1111_0100_0000_0001, 0b1111_1100_0000_0111),
        );
        m.insert(
            Instr::BRCS,
            Opcode(0b1111_0000_0000_0000, 0b1111_1100_0000_0111),
        );
        m.insert(
            Instr::SBIS,
            Opcode(0b1001_1011_0000_0000, 0b1111_1111_0000_0000),
        );
        m.insert(
            Instr::SEI,
            Opcode(0b1001_0100_0111_1000, 0b1111_1111_1111_1111),
        );
        m.insert(
            Instr::CLI,
            Opcode(0b1001_0100_1111_1000, 0b1111_1111_1111_1111),
        );
        m.insert(
            Instr::RET,
            Opcode(0b1001_0101_0000_1000, 0b1111_1111_1111_1111),
        );
        m.insert(
            Instr::PUSH,
            Opcode(0b1001_0010_0000_1111, 0b1111_1110_0000_1111),
        );
        m.insert(
            Instr::POP,
            Opcode(0b1001_0000_0000_1111, 0b1111_1110_0000_1111),
        );
        m.insert(
            Instr::MOV,
            Opcode(0b0010_1100_0000_0000, 0b1111_1100_0000_0000),
        );
        m.insert(
            Instr::MOVW,
            Opcode(0b0000_0001_0000_0000, 0b1111_1111_0000_0000),
        );
        m
    };
}

// WIP: Decode のアルゴリズムは Tree を使って高速化可能
// cf. https://www.avrfreaks.net/comment/128264#comment-128264
pub fn is_decoded(word: Word, code: &Opcode) -> bool {
    for (word_bit, code_bit, mask_bit) in izip!(word, Word(code.0), Word(code.1)) {
        if mask_bit && (word_bit != code_bit) {
            return false;
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
            &Opcode(0b0001_1100_0000_0000, 0b1111_1100_0000_0000)
        )
    );
}

pub fn decode_instr(w: Word) -> Option<&'static Instr> {
    for (instr, code) in OPCODE_MAP.iter() {
        if is_decoded(w, code) {
            return Some(instr);
        }
    }
    None
}

#[test]
pub fn test_decode_instr() {
    assert_eq!(Some(&Instr::ADC), decode_instr(Word(0b0001_1100_0111_0101)));
    assert_eq!(Some(&Instr::OUT), decode_instr(Word(0b1011_1110_0111_0101)));
    assert_eq!(Some(&Instr::NOP), decode_instr(Word(0b0000_0000_0000_0000)));
    assert_eq!(None, decode_instr(Word(0b1111_1111_1111_1111)));
}

// WIP
pub fn exec<T: AVR>(avr: &mut T, i: &Instr) {
    match i {
        &Instr::ADD => add(avr),
        &Instr::ADC => adc(avr),
        &Instr::SUB => sub(avr),
        &Instr::SBC => sbc(avr),
        &Instr::SUBI => subi(avr),
        &Instr::SBCI => sbci(avr),
        &Instr::DEC => dec(avr),
        &Instr::COM => com(avr),
        &Instr::LDI => ldi(avr),
        &Instr::LD1 => ld1(avr),
        &Instr::LD2 => ld2(avr),
        &Instr::LD3 => ld3(avr),
        &Instr::LDS => lds(avr),
        &Instr::LDDY1 => lddy1(avr),
        &Instr::LDDY2 => lddy2(avr),
        &Instr::LDDY3 => lddy3(avr),
        &Instr::LDDZ1 => lddz1(avr),
        &Instr::LDDZ2 => lddz2(avr),
        &Instr::LDDZ3 => lddz3(avr),
        &Instr::OUT => out(avr),
        &Instr::IN => in_instr(avr),
        &Instr::NOP => nop(avr),
        &Instr::CALL => call(avr),
        &Instr::RCALL => rcall(avr),
        &Instr::ROL => rol(avr),
        &Instr::LSL => lsl(avr),
        &Instr::JMP => jmp(avr),
        &Instr::RJMP => rjmp(avr),
        &Instr::ORI => ori(avr),
        &Instr::OR => or(avr),
        &Instr::EOR => eor(avr),
        &Instr::AND => and(avr),
        &Instr::ANDI => andi(avr),
        &Instr::STS => sts(avr),
        &Instr::ST1 => st1(avr),
        &Instr::ST2 => st2(avr),
        &Instr::ST3 => st3(avr),
        &Instr::STY1 => sty1(avr),
        &Instr::STY2 => sty2(avr),
        &Instr::STY3 => sty3(avr),
        &Instr::STZ1 => stz1(avr),
        &Instr::STZ2 => stz2(avr),
        &Instr::STZ3 => stz3(avr),
        &Instr::LPM1 => lpm1(avr),
        &Instr::LPM2 => lpm2(avr),
        &Instr::LPM3 => lpm3(avr),
        &Instr::CP => cp(avr),
        &Instr::CPI => cpi(avr),
        &Instr::CPC => cpc(avr),
        &Instr::CPSE => cpse(avr),
        &Instr::BREQ => breq(avr),
        &Instr::BRNE => brne(avr),
        &Instr::BRCS => brcs(avr),
        &Instr::SBIS => sbis(avr),
        &Instr::SBIW => sbiw(avr),
        &Instr::SEI => sei(avr),
        &Instr::CLI => cli(avr),
        &Instr::RET => ret(avr),
        &Instr::PUSH => push(avr),
        &Instr::POP => pop(avr),
        &Instr::MOV => mov(avr),
        &Instr::MOVW => movw(avr),
    };
}

pub fn add<T: AVR>(avr: &mut T) {
    let (r_addr, d_addr) = avr.word().operand55();
    let (r, d) = avr.gprgs(r_addr, d_addr);
    let res = r.wrapping_add(d);
    avr.set_gprg(d_addr, res);
    avr.set_status_by_arithmetic_instruction(d, r, res);
    avr.set_status(Sreg::C, has_borrow_from_msb(r, d, res));
    avr.pc_increment(1);
    avr.cycle_increment(1);
}

pub fn adc<T: AVR>(avr: &mut T) {
    let (r_addr, d_addr) = avr.word().operand55();
    let (r, d) = avr.gprgs(r_addr, d_addr);
    let c = avr.status(Sreg::C) as u8;
    let res = r.wrapping_add(d).wrapping_add(c);
    avr.set_gprg(d_addr, res);
    avr.set_status_by_arithmetic_instruction(d, r, res);
    avr.set_status(Sreg::C, has_borrow_from_msb(r, d, res));
    avr.pc_increment(1);
    avr.cycle_increment(1);
}

pub fn sbci<T: AVR>(avr: &mut T) {
    let (k, d_addr) = avr.word().operand84();
    let d = avr.gprg(d_addr);
    let c = avr.status(Sreg::C) as u8;
    let res = d.wrapping_sub(k).wrapping_sub(c);
    avr.set_gprg(d_addr, res);

    avr.set_status(Sreg::H, has_borrow_from_bit3(d, k, res));
    avr.set_status(Sreg::V, has_2complement_overflow(d, k, res));
    avr.set_status(Sreg::N, msb(res));
    if res != 0 {
        avr.set_status(Sreg::Z, false);
    };
    match d.checked_sub(k).and_then(|d_k| d_k.checked_sub(c)) {
        None => avr.set_status(Sreg::C, true),
        _ => avr.set_status(Sreg::C, false),
    };
    avr.signed_test();

    avr.pc_increment(1);
    avr.cycle_increment(1);
}

pub fn dec<T: AVR>(avr: &mut T) {
    let d_addr = avr.word().operand5();
    let d = avr.gprg(d_addr);
    let result = d.wrapping_sub(1);
    avr.set_gprg(d_addr, result);

    avr.set_status(Sreg::V, d == 0x80u8);
    avr.set_status(Sreg::N, msb(result));
    avr.set_status(Sreg::Z, result == 0);
    avr.signed_test();

    avr.pc_increment(1);
    avr.cycle_increment(1);
}

pub fn com<T: AVR>(avr: &mut T) {
    let d_addr = avr.word().operand5();
    let d = avr.gprg(d_addr);
    let res = 0xff - d;
    avr.set_gprg(d_addr, res);
    avr.set_status_by_bit_instruction(res);
    avr.set_status(Sreg::C, false);
    avr.pc_increment(1);
    avr.cycle_increment(1);
}

pub fn sub<T: AVR>(avr: &mut T) {
    let (r_addr, d_addr) = avr.word().operand55();
    let (r, d) = avr.gprgs(r_addr, d_addr);
    let res = d.wrapping_sub(r);
    avr.set_gprg(d_addr, res);
    avr.set_status_by_arithmetic_instruction(d, r, res);
    avr.set_status(Sreg::C, d < r);
    avr.pc_increment(1);
    avr.cycle_increment(1);
}

pub fn sbc<T: AVR>(avr: &mut T) {
    let (r_addr, d_addr) = avr.word().operand55();
    let (r, d) = avr.gprgs(r_addr, d_addr);
    let c = avr.status(Sreg::C) as u8;
    let res = d.wrapping_add(r).wrapping_add(c);
    avr.set_gprg(d_addr, res);
    avr.set_status_by_arithmetic_instruction(d, r, res);
    avr.set_status(Sreg::C, d < (r + 1));
    avr.pc_increment(1);
    avr.cycle_increment(1);
}

pub fn subi<T: AVR>(avr: &mut T) {
    let (k, d_addr) = avr.word().operand84();
    let d = avr.gprg(d_addr);
    let res = d.wrapping_sub(k);
    avr.set_gprg(d_addr, res);
    avr.set_status_by_arithmetic_instruction(d, k, res);
    avr.set_status(Sreg::C, d < k);
    avr.pc_increment(1);
    avr.cycle_increment(1);
}

pub fn ld1<T: AVR>(avr: &mut T) {
    let d_addr = avr.word().operand5();
    let x_addr = avr.preg(Preg::X);
    avr.set_gprg(d_addr, avr.gprg(x_addr as usize));
    avr.pc_increment(1);
    avr.cycle_increment(1);
}

pub fn ld2<T: AVR>(avr: &mut T) {
    let d_addr = avr.word().operand5();
    let x_addr = avr.preg(Preg::X);
    let x = avr.gprg(x_addr as usize);
    avr.set_gprg(d_addr, x);
    avr.set_preg(Preg::X, x_addr + 1);
    avr.pc_increment(1);
    avr.cycle_increment(2);
}

pub fn ld3<T: AVR>(avr: &mut T) {
    let d_addr = avr.word().operand5();
    let x_addr = avr.preg(Preg::X) - 1;
    avr.set_preg(Preg::X, x_addr);
    let x = avr.gprg(x_addr as usize);
    avr.set_gprg(d_addr, x);
    avr.pc_increment(1);
    avr.cycle_increment(3);
}

pub fn ldi<T: AVR>(avr: &mut T) {
    let (k, d_addr) = avr.word().operand84();
    avr.set_gprg(d_addr, k);
    avr.pc_increment(1);
    avr.cycle_increment(1);
}

pub fn lds<T: AVR>(avr: &mut T) {
    let (w, k_addr) = avr.double_word();
    let d_addr = w.operand5();
    let k = avr.gprg(k_addr.0 as usize);
    avr.set_gprg(d_addr, k);
    avr.pc_increment(2);
    avr.cycle_increment(2);
}

pub fn lddy1<T: AVR>(avr: &mut T) {
    let d_addr = avr.word().operand5();
    let y_addr = avr.preg(Preg::Y);
    avr.set_gprg(d_addr, avr.gprg(y_addr as usize));
    avr.pc_increment(1);
    avr.cycle_increment(2); // 1 cycles in Manual
}

pub fn lddy2<T: AVR>(avr: &mut T) {
    let d_addr = avr.word().operand5();
    let y_addr = avr.preg(Preg::Y);
    avr.set_gprg(d_addr, avr.gprg(y_addr as usize));
    avr.set_preg(Preg::Y, y_addr + 1);
    avr.pc_increment(1);
    avr.cycle_increment(2);
}

pub fn lddy3<T: AVR>(avr: &mut T) {
    let d_addr = avr.word().operand5();
    let y_addr = avr.preg(Preg::Y) - 1;
    avr.set_preg(Preg::Y, y_addr);
    avr.set_gprg(d_addr, avr.gprg(y_addr as usize));
    avr.pc_increment(1);
    avr.cycle_increment(2);
}
pub fn lddz1<T: AVR>(avr: &mut T) {
    let d_addr = avr.word().operand5();
    let z_addr = avr.preg(Preg::Z);
    avr.set_gprg(d_addr, avr.gprg(z_addr as usize));
    avr.pc_increment(1);
    avr.cycle_increment(2); // 1 cycles in Manual
}

pub fn lddz2<T: AVR>(avr: &mut T) {
    let d_addr = avr.word().operand5();
    let z_addr = avr.preg(Preg::Z);
    avr.set_gprg(d_addr, avr.gprg(z_addr as usize));
    avr.set_preg(Preg::Z, z_addr + 1);
    avr.pc_increment(1);
    avr.cycle_increment(2);
}

pub fn lddz3<T: AVR>(avr: &mut T) {
    let d_addr = avr.word().operand5();
    let z_addr = avr.preg(Preg::Z) - 1;
    avr.set_preg(Preg::Z, z_addr);
    avr.set_gprg(d_addr, avr.gprg(z_addr as usize));
    avr.pc_increment(1);
    avr.cycle_increment(2);
}

pub fn out<T: AVR>(avr: &mut T) {
    let (a_addr, r_addr) = avr.word().operand65();
    let r = avr.gprg(r_addr);
    avr.set_gprg(a_addr, r);
    avr.pc_increment(1);
    avr.cycle_increment(1);
}

pub fn in_instr<T: AVR>(avr: &mut T) {
    let (a_addr, d_addr) = avr.word().operand65();
    let a = avr.gprg(a_addr);
    avr.set_gprg(d_addr, a);
    avr.pc_increment(1);
    avr.cycle_increment(1);
}

pub fn nop<T: AVR>(avr: &mut T) {
    avr.pc_increment(1);
    avr.cycle_increment(1);
}

pub fn call<T: AVR>(avr: &mut T) {
    // Push current pc to stack
    // WIP: ATmega328p is 16bit Program Counter machine...
    //      if pc is 16 bit, then sp-2. if pc is 22 bit then sp-3.
    avr.push_pc_stack(avr.pc() + 2);

    // Update pc by immediate
    let (w1, w2) = avr.double_word();
    avr.set_pc(w1.operand22(w2));

    avr.cycle_increment(4);
}

pub fn rol<T: AVR>(avr: &mut T) {
    let d_addr = avr.word().operand10() as usize;
    let d_old = avr.gprg(d_addr);
    let c = avr.status(Sreg::C) as u8;
    let d_new = ( d_old << 1 ) | c;
    avr.set_gprg(d_addr, d_new);

    avr.set_status(Sreg::H, ( d_old & 0b0000_1000 )>>3 == 1);
    avr.set_status(Sreg::N, msb(d_new));
    avr.set_status(Sreg::Z, d_new == 0);
    avr.set_status(Sreg::C, msb(d_old));
    avr.set_status(Sreg::V, avr.status(Sreg::N) ^ avr.status(Sreg::C));
    avr.signed_test();

    avr.pc_increment(1);
    avr.cycle_increment(3);
}

pub fn lsl<T: AVR>(avr: &mut T) {
    let d_addr = avr.word().operand10() as usize;
    let d_old = avr.gprg(d_addr);
    let d_new = d_old << 1;
    avr.set_gprg(d_addr, d_new);

    avr.set_status(Sreg::H, ( d_old & 0b0000_1000 )>>3 == 1);
    avr.set_status(Sreg::N, msb(d_new));
    avr.set_status(Sreg::Z, d_new == 0);
    avr.set_status(Sreg::C, msb(d_old));
    avr.set_status(Sreg::V, avr.status(Sreg::N) ^ avr.status(Sreg::C));
    avr.signed_test();

    avr.pc_increment(1);
    avr.cycle_increment(3);
}

// WIP
pub fn rcall<T: AVR>(avr: &mut T) {
    let k = avr.word().operand12() as u32;
    let pc = avr.pc();
    avr.set_pc(pc + k + 1);
    avr.cycle_increment(3);
}

pub fn jmp<T: AVR>(avr: &mut T) {
    let (w1, w2) = avr.double_word();
    let k = w1.operand22(w2);
    avr.set_pc(k);
    avr.cycle_increment(2); // 3 cycles in Manual
}

pub fn rjmp<T: AVR>(avr: &mut T) {
    let k = avr.word().operand12();
    let pc = avr.pc();
    let result = add_12bits_in_twos_complement_form(pc, k) + 1u32;
    avr.set_pc(result);
    avr.cycle_increment(2);
}

pub fn sts<T: AVR>(avr: &mut T) {
    let (w1, k) = avr.double_word();
    let d_addr = w1.operand5();
    let d = avr.gprg(d_addr);
    avr.set_gprg(k.0 as usize, d);
    avr.pc_increment(2);
    avr.cycle_increment(2);
}

pub fn lpm1<T: AVR>(avr: &mut T) {
    avr.set_gprg(0, avr.z_program_memory());
    avr.pc_increment(1);
    avr.cycle_increment(3);
}

pub fn lpm2<T: AVR>(avr: &mut T) {
    let d_addr = avr.word().operand5();
    avr.set_gprg(d_addr, avr.z_program_memory());
    avr.pc_increment(1);
    avr.cycle_increment(3);
}

pub fn lpm3<T: AVR>(avr: &mut T) {
    let d_addr = avr.word().operand5();
    avr.set_gprg(d_addr, avr.z_program_memory());
    avr.set_preg(Preg::Z, avr.preg(Preg::Z) + 1);
    avr.pc_increment(1);
    avr.cycle_increment(3);
}

pub fn st1<T: AVR>(avr: &mut T) {
    let d_addr = avr.word().operand5();
    let x_addr = avr.preg(Preg::X);
    let d = avr.gprg(d_addr);
    avr.set_gprg(x_addr as usize, d);
    avr.pc_increment(1);
    avr.cycle_increment(2);
}

pub fn st2<T: AVR>(avr: &mut T) {
    let d_addr = avr.word().operand5();
    let x_addr = avr.preg(Preg::X);
    let d = avr.gprg(d_addr);
    avr.set_gprg(x_addr as usize, d);
    avr.set_preg(Preg::X, x_addr + 1);
    avr.pc_increment(1);
    avr.cycle_increment(2);
}

pub fn st3<T: AVR>(avr: &mut T) {
    let d_addr = avr.word().operand5();
    let x_addr = avr.preg(Preg::X) - 1;
    let d = avr.gprg(d_addr);
    avr.set_preg(Preg::X, x_addr);
    avr.set_gprg(x_addr as usize, d);
    avr.pc_increment(1);
    avr.cycle_increment(2);
}

pub fn sty1<T: AVR>(avr: &mut T) {
    let d_addr = avr.word().operand5();
    let y_addr = avr.preg(Preg::Y);
    let d = avr.gprg(d_addr);
    avr.set_gprg(y_addr as usize, d);
    avr.pc_increment(1);
    avr.cycle_increment(2);
}

pub fn sty2<T: AVR>(avr: &mut T) {
    let d_addr = avr.word().operand5();
    let y_addr = avr.preg(Preg::Y);
    let d = avr.gprg(d_addr);
    avr.set_gprg(y_addr as usize, d);
    avr.set_preg(Preg::Y, y_addr + 1);
    avr.pc_increment(1);
    avr.cycle_increment(2);
}

pub fn sty3<T: AVR>(avr: &mut T) {
    let d_addr = avr.word().operand5();
    let y_addr = avr.preg(Preg::Y) - 1;
    let d = avr.gprg(d_addr);
    avr.set_preg(Preg::Y, y_addr);
    avr.set_gprg(y_addr as usize, d);
    avr.pc_increment(1);
    avr.cycle_increment(2);
}
pub fn stz1<T: AVR>(avr: &mut T) {
    let d_addr = avr.word().operand5();
    let z_addr = avr.preg(Preg::Z);
    let d = avr.gprg(d_addr);
    avr.set_gprg(z_addr as usize, d);
    avr.pc_increment(1);
    avr.cycle_increment(2);
}

pub fn stz2<T: AVR>(avr: &mut T) {
    let d_addr = avr.word().operand5();
    let z_addr = avr.preg(Preg::Z);
    let d = avr.gprg(d_addr);
    avr.set_gprg(z_addr as usize, d);
    avr.set_preg(Preg::Z, z_addr + 1);
    avr.pc_increment(1);
    avr.cycle_increment(2);
}

pub fn stz3<T: AVR>(avr: &mut T) {
    let d_addr = avr.word().operand5();
    let z_addr = avr.preg(Preg::Z) - 1;
    let d = avr.gprg(d_addr);
    avr.set_preg(Preg::Z, z_addr);
    avr.set_gprg(z_addr as usize, d);
    avr.pc_increment(1);
    avr.cycle_increment(2);
}

pub fn cp<T: AVR>(avr: &mut T) {
    let (r_addr, d_addr) = avr.word().operand55();
    let (r, d) = avr.gprgs(r_addr, d_addr);
    let res = d.wrapping_sub(r);
    avr.set_status_by_arithmetic_instruction(d, r, res);
    avr.set_status(Sreg::C, d < r);
    avr.pc_increment(1);
    avr.cycle_increment(1);
}

pub fn cpi<T: AVR>(avr: &mut T) {
    let (k, d_addr) = avr.word().operand84();
    let d = avr.gprg(d_addr);
    let res = d.wrapping_sub(k);
    avr.set_status_by_arithmetic_instruction(d, k, res);
    avr.set_status(Sreg::C, d < k);
    avr.pc_increment(1);
    avr.cycle_increment(1);
}

pub fn cpc<T: AVR>(avr: &mut T) {
    let (r_addr, d_addr) = avr.word().operand55();
    let (r, d) = avr.gprgs(r_addr, d_addr);
    let c = avr.status(Sreg::C) as u8;
    let res = d.wrapping_sub(r).wrapping_sub(c);
    avr.set_status(Sreg::H, has_borrow_from_bit3(d, r, res));
    avr.set_status(Sreg::V, has_2complement_overflow(d, r, res));
    avr.set_status(Sreg::N, msb(res));
    if res != 0 {
        avr.set_status(Sreg::Z, false);
    }
    avr.set_status(Sreg::C, d < r + c);
    avr.signed_test();
    avr.pc_increment(1);
    avr.cycle_increment(1);
}

pub fn cpse<T: AVR>(avr: &mut T) {
    let (r_addr, d_addr) = avr.word().operand55();
    let (r, d) = avr.gprgs(r_addr, d_addr);
    if r == d {
        // The skip size is diffrenet by next instruction size.
        let next_opcode = Word(avr.fetch(avr.pc() + 1));
        match decode_instr(next_opcode) {
            Some(i) => {
                if INSTRUCTION_32_BIT.contains(&i) {
                    avr.set_pc(avr.pc() + 3);
                    avr.cycle_increment(3);
                } else {
                    avr.set_pc(avr.pc() + 2);
                    avr.cycle_increment(2);
                };
            },
            None => {
                println!("instruction decode failed: {:016b}", next_opcode.0);
                process::exit(1);
            },
        };
    } else {
        avr.pc_increment(1);
        avr.cycle_increment(1);
    }
}

pub fn ori<T: AVR>(avr: &mut T) {
    let (k, d_addr) = avr.word().operand84();
    let d = avr.gprg(d_addr);
    let res = d | k;
    avr.set_gprg(d_addr, res);
    avr.set_status_by_bit_instruction(res);
    avr.pc_increment(1);
    avr.cycle_increment(1);
}

pub fn and<T: AVR>(avr: &mut T) {
    let (r_addr, d_addr) = avr.word().operand55();
    let (r, d) = avr.gprgs(r_addr, d_addr);
    let res = d & r;
    avr.set_gprg(d_addr, res);
    avr.set_status_by_bit_instruction(res);
    avr.pc_increment(1);
    avr.cycle_increment(1);
}

pub fn andi<T: AVR>(avr: &mut T) {
    let (k, d_addr) = avr.word().operand84();
    let d = avr.gprg(d_addr);
    let res = d & k;
    avr.set_gprg(d_addr, res);
    avr.set_status_by_bit_instruction(res);
    avr.pc_increment(1);
    avr.cycle_increment(1);
}

pub fn or<T: AVR>(avr: &mut T) {
    let (r_addr, d_addr) = avr.word().operand55();
    let (r, d) = avr.gprgs(r_addr, d_addr);
    let res = d | r;
    avr.set_gprg(d_addr, res);
    avr.set_status_by_bit_instruction(res);
    avr.pc_increment(1);
    avr.cycle_increment(1);
}

pub fn eor<T: AVR>(avr: &mut T) {
    let (r_addr, d_addr) = avr.word().operand55();
    let (r, d) = avr.gprgs(r_addr, d_addr);
    let res = d ^ r;
    avr.set_gprg(d_addr, res);
    avr.set_status_by_bit_instruction(res);
    avr.pc_increment(1);
    avr.cycle_increment(1);
}

pub fn breq<T: AVR>(avr: &mut T) {
    if avr.status(Sreg::Z) {
        let k = avr.word().operand7();
        let pc = avr.pc();
        let result = add_7bits_in_twos_complement_form(pc, k) + 1u32;
        avr.set_pc(result);
        avr.cycle_increment(2);
    } else {
        avr.pc_increment(1);
        avr.cycle_increment(1);
    }
}

pub fn brne<T: AVR>(avr: &mut T) {
    if avr.status(Sreg::Z) {
        avr.pc_increment(1);
        avr.cycle_increment(1);
    } else {
        let k = avr.word().operand7();
        let pc = avr.pc();
        let result = add_7bits_in_twos_complement_form(pc, k) + 1u32;
        avr.set_pc(result as u32);
        avr.cycle_increment(2);
    }
}

pub fn brcs<T: AVR>(avr: &mut T) {
    if avr.status(Sreg::C) {
        let k = avr.word().operand7();
        let pc = avr.pc();
        let result = add_7bits_in_twos_complement_form(pc, k) + 1u32;
        avr.set_pc(result as u32);
        avr.cycle_increment(2);
    } else {
        avr.pc_increment(1);
        avr.cycle_increment(1);
    }
}

pub fn sbis<T: AVR>(avr: &mut T) {
    let (a_addr, b) = avr.word().operand53();
    // I/O Register starts from 0x20(0x32), so there is offset.
    let a = avr.gprg((a_addr + 0x20) as usize);
    if bit(a, b) {
        // WIP: ATmega328p is 16bit Program Counter machine...
        avr.set_pc(avr.pc() + 2);
        avr.cycle_increment(2);
    } else {
        avr.pc_increment(1);
        avr.cycle_increment(1);
    }
}

pub fn sbiw<T: AVR>(avr: &mut T) {
    let (k, d_addr) = avr.word().operand62();
    let (dh, dl) = avr.gprgs(d_addr + 1, d_addr);
    let result = concat(dh, dl).wrapping_sub(k as u16);
    avr.set_gprg(d_addr + 1, high_bit(result));
    avr.set_gprg(d_addr, low_bit(result));

    avr.set_status(Sreg::V, msb(high_bit(result)) & !msb(dh));
    avr.set_status(Sreg::C, msb(high_bit(result)) & !msb(dh));
    avr.set_status(Sreg::N, msb(high_bit(result)));
    avr.set_status(Sreg::Z, result == 0);
    avr.signed_test();
    avr.pc_increment(1);
    avr.cycle_increment(2);
}

pub fn sei<T: AVR>(avr: &mut T) {
    avr.set_status(Sreg::I, true);
    avr.pc_increment(1);
    avr.cycle_increment(1);
}

pub fn cli<T: AVR>(avr: &mut T) {
    avr.set_status(Sreg::I, false);
    avr.pc_increment(1);
    avr.cycle_increment(1);
}

pub fn ret<T: AVR>(avr: &mut T) {
    // WIP: ATmega328p is 16bit Program Counter machine...
    let pc = avr.pop_pc_stack();
    avr.set_pc(pc as u32);
    avr.cycle_increment(4);
}

pub fn push<T: AVR>(avr: &mut T) {
    let d_addr = avr.word().operand5();
    let d = avr.gprg(d_addr);
    avr.push_stack(d);
    avr.pc_increment(1);
    avr.cycle_increment(2);
}

pub fn pop<T: AVR>(avr: &mut T) {
    let d_addr = avr.word().operand5();
    let s = avr.pop_stack();
    avr.set_gprg(d_addr, s);
    avr.pc_increment(1);
    avr.cycle_increment(2);
}

pub fn mov<T: AVR>(avr: &mut T) {
    let (r_addr, d_addr) = avr.word().operand55();
    let r = avr.gprg(r_addr);
    avr.set_gprg(d_addr, r);
    avr.pc_increment(1);
    avr.cycle_increment(1);
}

pub fn movw<T: AVR>(avr: &mut T) {
    let (d_addr, r_addr) = avr.word().operand44();
    let (rl, rh) = avr.gprgs(r_addr, r_addr + 1);
    avr.set_gprg(d_addr, rl);
    avr.set_gprg(d_addr + 1, rh);
    avr.pc_increment(1);
    avr.cycle_increment(1);
}
