use std::collections::HashMap;
use itertools::izip;
use lazy_static::lazy_static;
use super::avr::*;
use super::utils::*;


#[derive(Eq, PartialEq, Hash, Debug)]
pub enum Instr {
    ADD, ADC, SUB, SBC, SUBI, SBCI, DEC,
    LD1, LD2, LD3, LDI, LDS, OUT, IN,
    NOP,
    CALL, RCALL,
    JMP, RJMP,
    AND, ANDI, EOR, ORI,
    STS, ST1, ST2, ST3,
    LPM1, LPM2, LPM3,
    CPI, CPC, CPSE,
    BREQ, BRNE, BRCS, SBIS,
    SEI, CLI,
    RET, PUSH,
    MOV, MOVW,
}

pub struct Opcode(pub u16, pub u16);

lazy_static! {
    static ref OPCODE_MAP: HashMap<Instr, Opcode> = {
        let mut m = HashMap::new();
        m.insert(Instr::ADD,   Opcode(0b0000_1100_0000_0000, 0b1111_1100_0000_0000));
        m.insert(Instr::ADC,   Opcode(0b0001_1100_0000_0000, 0b1111_1100_0000_0000));
        m.insert(Instr::SUB,   Opcode(0b0001_1000_0000_0000, 0b1111_1100_0000_0000));
        m.insert(Instr::SBC,   Opcode(0b0000_1000_0000_0000, 0b1111_1100_0000_0000));
        m.insert(Instr::SUBI,  Opcode(0b0101_0000_0000_0000, 0b1111_0000_0000_0000));
        m.insert(Instr::SBCI,  Opcode(0b0100_0000_0000_0000, 0b1111_0000_0000_0000));
        m.insert(Instr::DEC,   Opcode(0b1001_0100_0000_1010, 0b1111_1110_0000_1111));
        m.insert(Instr::LDI,   Opcode(0b1110_0000_0000_0000, 0b1111_0000_0000_0000));
        m.insert(Instr::LD1,   Opcode(0b1001_0000_0000_1100, 0b1111_1110_0000_1111));
        m.insert(Instr::LD2,   Opcode(0b1001_0000_0000_1101, 0b1111_1110_0000_1111));
        m.insert(Instr::LD3,   Opcode(0b1001_0000_0000_1110, 0b1111_1110_0000_1111));
        m.insert(Instr::LDS,   Opcode(0b1001_0000_0000_0000, 0b1111_1110_0000_1111));
        m.insert(Instr::OUT,   Opcode(0b1011_1000_0000_0000, 0b1111_1000_0000_0000));
        m.insert(Instr::IN,    Opcode(0b1011_0000_0000_0000, 0b1111_1000_0000_0000));
        m.insert(Instr::NOP,   Opcode(0b0000_0000_0000_0000, 0b1111_1111_1111_1111));
        m.insert(Instr::CALL,  Opcode(0b1001_0100_0000_1110, 0b1111_1110_0000_1110));
        m.insert(Instr::RCALL, Opcode(0b1101_0000_0000_0000, 0b1111_0000_0000_0000));
        m.insert(Instr::JMP,   Opcode(0b1001_0100_0000_1100, 0b1111_1110_0000_1110));
        m.insert(Instr::RJMP,  Opcode(0b1100_0000_0000_0000, 0b1111_0000_0000_0000));
        m.insert(Instr::ORI,   Opcode(0b0110_0000_0000_0000, 0b1111_0000_0000_0000));
        m.insert(Instr::AND,   Opcode(0b0010_0000_0000_0000, 0b1111_1100_0000_0000));
        m.insert(Instr::ANDI,  Opcode(0b0111_0000_0000_0000, 0b1111_0000_0000_0000));
        m.insert(Instr::EOR,   Opcode(0b0010_0100_0000_0000, 0b1111_1100_0000_0000));
        m.insert(Instr::STS,   Opcode(0b1001_0010_0000_0000, 0b1111_1110_0000_1111));
        m.insert(Instr::ST1,   Opcode(0b1001_0010_0000_1100, 0b1111_1110_0000_1111));
        m.insert(Instr::ST2,   Opcode(0b1001_0010_0000_1101, 0b1111_1110_0000_1111));
        m.insert(Instr::ST3,   Opcode(0b1001_0010_0000_1110, 0b1111_1110_0000_1111));
        m.insert(Instr::LPM1,  Opcode(0b1001_0101_1100_1000, 0b1111_1111_1111_1111));
        m.insert(Instr::LPM2,  Opcode(0b1001_0000_0000_0100, 0b1111_1110_0000_1111));
        m.insert(Instr::LPM3,  Opcode(0b1001_0000_0000_0101, 0b1111_1110_0000_1111));
        m.insert(Instr::CPI,   Opcode(0b0011_0000_0000_0000, 0b1111_0000_0000_0000));
        m.insert(Instr::CPC,   Opcode(0b0000_0100_0000_0000, 0b1111_1100_0000_0000));
        m.insert(Instr::CPSE,  Opcode(0b0001_0000_0000_0000, 0b1111_1100_0000_0000));
        m.insert(Instr::BREQ,  Opcode(0b1111_0000_0000_0001, 0b1111_1100_0000_0111));
        m.insert(Instr::BRNE,  Opcode(0b1111_0100_0000_0001, 0b1111_1100_0000_0111));
        m.insert(Instr::BRCS,  Opcode(0b1111_0000_0000_0000, 0b1111_1100_0000_0111));
        m.insert(Instr::SBIS,  Opcode(0b1001_1011_0000_0000, 0b1111_1111_0000_0000));
        m.insert(Instr::SEI,   Opcode(0b1001_0100_0111_1000, 0b1111_1111_1111_1111));
        m.insert(Instr::CLI,   Opcode(0b1001_0100_1111_1000, 0b1111_1111_1111_1111));
        m.insert(Instr::RET,   Opcode(0b1001_0101_0000_1000, 0b1111_1111_1111_1111));
        m.insert(Instr::PUSH,  Opcode(0b1001_0010_0000_1111, 0b1111_1110_0000_1111));
        m.insert(Instr::MOV,   Opcode(0b0010_1100_0000_0000, 0b1111_1100_0000_0000));
        m.insert(Instr::MOVW,  Opcode(0b0000_0001_0000_0000, 0b1111_1111_0000_0000));
        m
    };
}

// WIP: Decode のアルゴリズムは Tree を使って高速化可能
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

pub trait AVRInstruction: AVR {
    fn exec(&mut self, i: &Instr) {
        println!("||| ^-- instruction : {:?}", i);
        match i {
            &Instr::ADD   => self.add(),
            &Instr::ADC   => self.adc(),
            &Instr::SUB   => self.sub(),
            &Instr::SBC   => self.sbc(),
            &Instr::SUBI  => self.subi(),
            &Instr::SBCI  => self.sbci(),
            &Instr::DEC   => self.dec(),
            &Instr::LDI   => self.ldi(),
            &Instr::LD1   => self.ld1(),
            &Instr::LD2   => self.ld2(),
            &Instr::LD3   => self.ld3(),
            &Instr::LDS   => self.lds(),
            &Instr::OUT   => self.out(),
            &Instr::IN    => self.in_instr(),
            &Instr::NOP   => self.nop(),
            &Instr::CALL  => self.call(),
            &Instr::RCALL => self.rcall(),
            &Instr::JMP   => self.jmp(),
            &Instr::RJMP  => self.rjmp(),
            &Instr::ORI   => self.ori(),
            &Instr::EOR   => self.eor(),
            &Instr::AND   => self.and(),
            &Instr::ANDI  => self.andi(),
            &Instr::STS   => self.sts(),
            &Instr::ST1   => self.st1(),
            &Instr::ST2   => self.st2(),
            &Instr::ST3   => self.st3(),
            &Instr::LPM1  => self.lpm1(),
            &Instr::LPM2  => self.lpm2(),
            &Instr::LPM3  => self.lpm3(),
            &Instr::CPI   => self.cpi(),
            &Instr::CPC   => self.cpc(),
            &Instr::CPSE  => self.cpse(),
            &Instr::BREQ  => self.breq(),
            &Instr::BRNE  => self.brne(),
            &Instr::BRCS  => self.brcs(),
            &Instr::SBIS  => self.sbis(),
            &Instr::SEI   => self.sei(),
            &Instr::CLI   => self.cli(),
            &Instr::RET   => self.ret(),
            &Instr::PUSH  => self.push(),
            &Instr::MOV   => self.mov(),
            &Instr::MOVW  => self.movw(),
        };
    }

    fn add(&mut self) {
        let (r_addr, d_addr) = operand55(self.word());
        let r = self.gprg(r_addr as usize);
        let d = self.gprg(d_addr as usize);
        let result = r.wrapping_add(d);
        self.set_gprg(d_addr as usize, result);

        self.set_status(Sreg::H, has_borrow_from_bit3(d, r, result));
        self.set_status(Sreg::V, has_2complement_overflow(d, r, result));
        self.set_status(Sreg::N, msb(result));
        self.set_status(Sreg::Z, result == 0);
        self.set_status(Sreg::C, has_borrow_from_msb(r, d, result));
        self.signed_test();
    
        self.pc_increment();
    }
    
    fn adc(&mut self) {
        let (r_addr, d_addr) = operand55(self.word());
        let r = self.gprg(r_addr as usize);
        let d = self.gprg(d_addr as usize);
        let c = if self.status(Sreg::C) { 1u8 } else { 0u8 };
        let result = r.wrapping_add(d).wrapping_add(c);
        self.set_gprg(d_addr as usize, result);

        self.set_status(Sreg::H, has_borrow_from_bit3(d, r, result));
        self.set_status(Sreg::V, has_2complement_overflow(d, r, result));
        self.set_status(Sreg::N, msb(result));
        self.set_status(Sreg::Z, result == 0);
        self.set_status(Sreg::C, has_borrow_from_msb(r, d, result));
        self.signed_test();
    
        self.pc_increment();
    }
    
    fn sbci(&mut self) {
        let (k, d_addr) = operand84(self.word());
        // there is a 16 addr offset
        let d = self.gprg(d_addr as usize + 16);
        let c = if self.status(Sreg::C) { 1u8 } else { 0u8 };
        let result = d.wrapping_sub(k).wrapping_sub(c);
        self.set_gprg(d_addr as usize, result);

        self.set_status(Sreg::H, has_borrow_from_bit3(d, k, result));
        self.set_status(Sreg::V, has_2complement_overflow(d, k, result));
        self.set_status(Sreg::N, msb(result));
        self.set_status(Sreg::Z, result == 0);
        self.set_status(Sreg::C, d < k);
        self.signed_test();
    
        self.pc_increment();
    }
    
    fn dec(&mut self) {
        let d_addr = operand5(self.word());
        let d = self.gprg(d_addr as usize);
        let result = d.wrapping_sub(1);

        self.set_gprg(d_addr as usize, result);

        self.set_status(Sreg::V, d == 0x80u8);
        self.set_status(Sreg::N, msb(result));
        self.set_status(Sreg::Z, result == 0);
        self.signed_test();
    
        self.pc_increment();
    }
    
    fn sub(&mut self) {
        let (r_addr, d_addr) = operand55(self.word());
        let r = self.gprg(r_addr as usize);
        let d = self.gprg(d_addr as usize);
        let result = d.wrapping_sub(r);
        self.set_gprg(d_addr as usize, result);

        self.set_status(Sreg::H, has_borrow_from_bit3(d, r, result));
        self.set_status(Sreg::V, has_2complement_overflow(d, r, result));
        self.set_status(Sreg::N, msb(result));
        self.set_status(Sreg::Z, result == 0);
        self.set_status(Sreg::C, d < r);
        self.signed_test();
    
        self.pc_increment();
    }
    
    fn sbc(&mut self) {
        let (r_addr, d_addr) = operand55(self.word());
        let r = self.gprg(r_addr as usize);
        let d = self.gprg(d_addr as usize);
        let c = if self.status(Sreg::C) { 1u8 } else { 0u8 };
        let result = d.wrapping_add(r).wrapping_add(c);
        self.set_gprg(d_addr as usize, result);

        self.set_status(Sreg::H, has_borrow_from_bit3(d, r, result));
        self.set_status(Sreg::V, has_2complement_overflow(d, r, result));
        self.set_status(Sreg::N, msb(result));
        self.set_status(Sreg::Z, result == 0);
        self.set_status(Sreg::C, d < (r+1));
        self.signed_test();
    
        self.pc_increment();
    }
    
    fn subi(&mut self) {
        let (k, d_addr) = operand84(self.word());
        // there is a 16 addr offset
        let d = self.gprg(d_addr as usize + 16);
        let result = d.wrapping_sub(k);
        self.set_gprg(d_addr as usize, result);

        self.set_status(Sreg::H, has_borrow_from_bit3(d, k, result));
        self.set_status(Sreg::V, has_2complement_overflow(d, k, result));
        self.set_status(Sreg::N, msb(result));
        self.set_status(Sreg::Z, result == 0);
        self.set_status(Sreg::C, d < k);
        self.signed_test();
    
        self.pc_increment();
    }
    
    fn ld1(&mut self) {
        let d_addr = operand5(self.word());
        let x_addr = self.xyz_reg(XYZReg::X);
        let x = self.gprg(x_addr as usize);
        self.set_gprg(d_addr as usize, x);
        self.pc_increment();
    }
    
    fn ld2(&mut self) {
        let d_addr = operand5(self.word());
        let x_addr = self.xyz_reg(XYZReg::X);
        let x = self.gprg(x_addr as usize);
        self.set_gprg(d_addr as usize, x);
        self.set_xyz_reg(XYZReg::X, x_addr + 1u16);
        self.pc_increment();
    }
    
    fn ld3(&mut self) {
        let d_addr = operand5(self.word());
        let x_addr = self.xyz_reg(XYZReg::X) - 1u16;
        self.set_xyz_reg(XYZReg::X, x_addr);
        let x = self.gprg(x_addr as usize);
        self.set_gprg(d_addr as usize, x);
        self.pc_increment();
    }
    
    fn ldi(&mut self) {
        let (k, d_addr) = operand84(self.word());
        // there is a 16 addr offset
        self.set_gprg(d_addr as usize + 16, k);
        self.pc_increment();
    }
    
    fn lds(&mut self) {
        let (w1, k) = self.double_word();
        let d_addr = operand5(w1);
        self.set_gprg(d_addr as usize, k.0 as u8);
        self.pc_double_increment();
    }
    
    fn out(&mut self) {
        let (a_addr, r_addr) = operand65(self.word());
        let r = self.gprg(r_addr as usize);
        // I/O Register starts from 0x20(0d32), so there is offset.
        self.set_gprg(( a_addr + 0x20 ) as usize, r);
        self.pc_increment();
    }
    
    fn in_instr(&mut self) {
        let (a_addr, d_addr) = operand65(self.word());
        // I/O Register starts from 0x20(0d32), so there is offset.
        let a = self.gprg(( a_addr + 0x20 ) as usize);
        self.set_gprg(d_addr as usize, a);
        self.pc_increment();
    }
    
    fn nop(&mut self) {
        self.pc_increment();
    }
    
    fn call(&mut self) {
        // Push current pc to stack
        // WIP: ATmega328p is 16bit Program Counter machine...
        //      if pc is 16 bit, then sp-2. if pc is 22 bit then sp-3.
        self.push_pc_stack(self.pc()+2); 

        // Update pc by immediate
        let (w1, w2) = self.double_word();
        self.set_pc(operand22(w1, w2));
    }
    
    // WIP
    fn rcall(&mut self) {
        let k = operand12(self.word()) as u32;
        let pc = self.pc();
        self.set_pc(pc+k+1);
    }
    
    fn jmp(&mut self) {
        let (w1, w2) = self.double_word();
        let k = operand22(w1, w2);
        self.set_pc(k)
    }
    
    fn rjmp(&mut self) {
        let k = operand12(self.word());
        let pc = self.pc();
        let result = add_12bits_in_twos_complement_form(pc, k) + 1u32;
        self.set_pc(result);
    }
    
    fn sts(&mut self) {
        let (w1, k) = self.double_word();
        let d_addr = operand5(w1);
        let d = self.gprg(d_addr as usize);
        self.set_gprg(k.0 as usize, d);
        self.pc_double_increment();
    }
    
    fn lpm1(&mut self) {
        if ( self.xyz_reg(XYZReg::Z) & 1 ) == 1 {
            self.set_gprg(0, high_bit(self.xyz_reg(XYZReg::Z)));
        } else {
            self.set_gprg(0, low_bit(self.xyz_reg(XYZReg::Z)));
        }
        self.pc_increment();
    }
    
    fn lpm2(&mut self) {
        let d_addr = operand5(self.word());
        if ( self.xyz_reg(XYZReg::Z) & 1 ) == 1 {
            self.set_gprg(d_addr as usize, high_bit(self.xyz_reg(XYZReg::Z)));
        } else {
            self.set_gprg(d_addr as usize, low_bit(self.xyz_reg(XYZReg::Z)));
        }
        self.pc_increment();
    }
    
    fn lpm3(&mut self) {
        let d_addr = operand5(self.word());
        if ( self.xyz_reg(XYZReg::Z) & 1 ) == 1 {
            self.set_gprg(d_addr as usize, high_bit(self.xyz_reg(XYZReg::Z)));
        } else {
            self.set_gprg(d_addr as usize, low_bit(self.xyz_reg(XYZReg::Z)));
        }
        self.set_xyz_reg(XYZReg::Z, self.xyz_reg(XYZReg::Z)+1);
        self.pc_increment();
    }
    
    // WIP
    fn st1(&mut self) {
        // self.pc_increment();
    }
    
    // WIP
    fn st2(&mut self) {
        // self.pc_increment();
    }
    
    // WIP
    fn st3(&mut self) {
        // self.pc_increment();
    }
    
    // WIP
    fn cpi(&mut self) {
        let (k, d_addr) = operand84(self.word());
        // there is a 16 addr offset
        let d = self.gprg(d_addr as usize + 16);
        let result = d.wrapping_sub(k);
    
        self.set_status(Sreg::H, has_borrow_from_bit3(d, k, result));
        self.set_status(Sreg::V, has_2complement_overflow(d, k, result));
        self.set_status(Sreg::N, msb(result));
        self.set_status(Sreg::Z, result == 0);
        self.set_status(Sreg::C, d < k);
        self.signed_test();
    
        self.pc_increment();
    }
    
    fn cpc(&mut self) {
        let (r_addr, d_addr) = operand55(self.word());
        let r = self.gprg(r_addr as usize);
        let d = self.gprg(d_addr as usize);
        let c = if self.status(Sreg::C) { 1u8 } else { 0u8 };
        let result = d.wrapping_sub(r).wrapping_sub(c);
    
        self.set_status(Sreg::H, has_borrow_from_bit3(d, r, result));
        self.set_status(Sreg::V, has_2complement_overflow(d, r, result));
        self.set_status(Sreg::N, msb(result));
        if result == 0 {
            self.set_status(Sreg::Z, false);
        }
        self.set_status(Sreg::C, d < r+c);
        self.signed_test();
    
        self.pc_increment();
    }
    
    fn cpse(&mut self) {
        let (r_addr, d_addr) = operand55(self.word());
        let r = self.gprg(r_addr as usize);
        let d = self.gprg(d_addr as usize);
        if r == d {
            // WIP: ATmega328p is 16bit Program Counter machine...
            self.set_pc(self.pc()+2);
        } else {
            self.pc_increment();
        }
    }
    
    fn ori(&mut self) {
        let (k, d_addr) = operand84(self.word());
        // there is a 16 addr offset
        let d = self.gprg(d_addr as usize + 16);
        let result = d | k;
        self.set_gprg(d_addr as usize, result);
    
        self.set_status(Sreg::V, false);
        self.set_status(Sreg::N, msb(result));
        self.set_status(Sreg::Z, result == 0);
        self.signed_test();
        self.pc_increment();
    }
    
    fn and(&mut self) {
        let (r_addr, d_addr) = operand55(self.word());
        let r = self.gprg(r_addr as usize);
        let d = self.gprg(d_addr as usize);
        let result = d & r;
        self.set_gprg(d_addr as usize, result);
    
        self.set_status(Sreg::V, false);
        self.set_status(Sreg::N, msb(result));
        self.set_status(Sreg::Z, result == 0);
        self.signed_test();
        self.pc_increment();
    }
    
    fn andi(&mut self) {
        let (k, d_addr) = operand84(self.word());
        // there is a 16 addr offset
        let d = self.gprg(( d_addr + 16 ) as usize);
        let result = d & k;
        self.set_gprg(d_addr as usize, result);
    
        self.set_status(Sreg::V, false);
        self.set_status(Sreg::N, msb(result));
        self.set_status(Sreg::Z, result == 0);
        self.signed_test();
        self.pc_increment();
    }
    
    fn eor(&mut self) {
        let (r_addr, d_addr) = operand55(self.word());
        let r = self.gprg(r_addr as usize);
        let d = self.gprg(d_addr as usize);
    
        let result = d^r;
        self.set_gprg(d_addr as usize, result);
    
        self.set_status(Sreg::V, false);
        self.set_status(Sreg::N, msb(result));
        self.set_status(Sreg::Z, result == 0);
        self.signed_test();
        self.pc_increment();
    }

    fn breq(&mut self) {
        if self.status(Sreg::Z) {
            let k = operand7(self.word()) as u32;
            let pc = self.pc();
            self.set_pc(pc+k+1);
        } else {
            self.pc_increment();
        }
    }

    fn brne(&mut self) {
        if self.status(Sreg::Z) {
            self.pc_increment();
        } else {
            let k = operand7(self.word());
            let pc = self.pc();
            let result = add_7bits_in_twos_complement_form(pc, k) + 1u32;
            self.set_pc(result as u32);
        }
    }
    
    fn brcs(&mut self) {
        if self.status(Sreg::C) {
            let k = operand7(self.word());
            let pc = self.pc();
            let result = add_7bits_in_twos_complement_form(pc, k) + 1u32;
            self.set_pc(result as u32);
        } else {
            self.pc_increment();
        }
    }
    
    // WIP
    fn sbis(&mut self) {
        let (a_addr, b) = operand53(self.word());
        // I/O Register starts from 0x20(0x32), so there is offset.
        let a = self.gprg(( a_addr + 0x20 ) as usize);
        println!("||||||||| a_addr = {}", a_addr+0x20);
        println!("||||||||| a = {}", a);
        println!("||||||||| b = {}", b);
        if bit(a, b) {
            // WIP: ATmega328p is 16bit Program Counter machine...
            self.set_pc(self.pc()+2);
        } else {
            self.pc_increment();
        }
    }
    
    fn sei(&mut self) {
        self.set_status(Sreg::I, true);
        self.pc_increment();
    }

    fn cli(&mut self) {
        self.set_status(Sreg::I, false);
        self.pc_increment();
    }

    fn ret(&mut self) {
        // WIP: ATmega328p is 16bit Program Counter machine...
        let pc = self.pop_pc_stack();
        println!("|||||||||| pc = {:x}", pc);
        self.set_pc(pc as u32);
    }

    fn push(&mut self) {
        let d_addr = operand5(self.word());
        let d = self.gprg(d_addr as usize);
        self.push_stack(d);
        println!("|||||||||| push, R{} = {:x}", d_addr, d);
        self.pc_increment();
    }

    fn mov(&mut self) {
        let (r_addr, d_addr) = operand55(self.word());
        let r = self.gprg(r_addr as usize);
        self.set_gprg(d_addr as usize, r);
        self.pc_increment();
    }

    fn movw(&mut self) {
        let (d_addr, r_addr) = operand55(self.word());
        let rl = self.gprg(r_addr as usize);
        let rh = self.gprg(( r_addr+1 ) as usize);
        self.set_gprg(d_addr as usize, rl);
        self.set_gprg(( d_addr + 1 ) as usize, rh);
        self.pc_increment();
    }
}