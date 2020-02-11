use std::collections::HashMap;
use itertools::izip;
use lazy_static::lazy_static;
use super::avr::*;
use super::utils::*;
use super::word::*;


#[derive(Eq, PartialEq, Hash, Debug)]
pub enum Instr {
    ADD, ADC, SUB, SBC, SUBI, SBCI, SBIW, DEC, COM,
    LD1, LD2, LD3, LDI, LDS, OUT, IN,
    NOP,
    CALL, RCALL,
    JMP, RJMP,
    AND, ANDI, EOR, ORI,
    STS, ST1, ST2, ST3,
    LPM1, LPM2, LPM3,
    CP, CPI, CPC, CPSE,
    BREQ, BRNE, BRCS, SBIS,
    SEI, CLI,
    RET, PUSH, POP,
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
        m.insert(Instr::SBIW,  Opcode(0b1001_0111_0000_0000, 0b1111_1111_0000_0000));
        m.insert(Instr::DEC,   Opcode(0b1001_0100_0000_1010, 0b1111_1110_0000_1111));
        m.insert(Instr::COM,   Opcode(0b1001_0100_0000_0000, 0b1111_1110_0000_1111));
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
        m.insert(Instr::CP,    Opcode(0b0001_0100_0000_0000, 0b1111_1100_0000_0000));
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
        m.insert(Instr::POP,   Opcode(0b1001_0000_0000_1111, 0b1111_1110_0000_1111));
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
            &Instr::COM   => self.com(),
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
            &Instr::CP    => self.cp(),
            &Instr::CPI   => self.cpi(),
            &Instr::CPC   => self.cpc(),
            &Instr::CPSE  => self.cpse(),
            &Instr::BREQ  => self.breq(),
            &Instr::BRNE  => self.brne(),
            &Instr::BRCS  => self.brcs(),
            &Instr::SBIS  => self.sbis(),
            &Instr::SBIW  => self.sbiw(),
            &Instr::SEI   => self.sei(),
            &Instr::CLI   => self.cli(),
            &Instr::RET   => self.ret(),
            &Instr::PUSH  => self.push(),
            &Instr::POP   => self.pop(),
            &Instr::MOV   => self.mov(),
            &Instr::MOVW  => self.movw(),
        };
    }

    fn add(&mut self) {
        let (r_addr, d_addr) = self.word().operand55();
        let (r, d) = self.gprgs(r_addr, d_addr);
        let res = r.wrapping_add(d);
        self.set_gprg(d_addr, res);
        self.set_status_by_arithmetic_instruction(d, r, res);
        self.set_status(Sreg::C, has_borrow_from_msb(r, d, res));
        self.pc_increment();
    }

    fn adc(&mut self) {
        let (r_addr, d_addr) = self.word().operand55();
        let (r, d) = self.gprgs(r_addr, d_addr);
        let c = self.status_as_u8(Sreg::C);
        let res = r.wrapping_add(d).wrapping_add(c);
        self.set_gprg(d_addr, res);
        self.set_status_by_arithmetic_instruction(d, r, res);
        self.set_status(Sreg::C, has_borrow_from_msb(r, d, res));
        self.pc_increment();
    }

    fn sbci(&mut self) {
        let (k, d_addr) = self.word().operand84();
        let d = self.gprg(d_addr);
        let c = self.status_as_u8(Sreg::C);
        let res = d.wrapping_sub(k).wrapping_sub(c);
        self.set_gprg(d_addr, res);
        // self.set_status_by_arithmetic_instruction(d, r, res);
        self.set_status(Sreg::C, d < k);
        self.pc_increment();
    }

    fn dec(&mut self) {
        let d_addr = self.word().operand5();
        let d = self.gprg(d_addr);
        let result = d.wrapping_sub(1);
        self.set_gprg(d_addr, result);

        self.set_status(Sreg::V, d == 0x80u8);
        self.set_status(Sreg::N, msb(result));
        self.set_status(Sreg::Z, result == 0);
        self.signed_test();

        self.pc_increment();
    }

    fn com(&mut self) {
        let d_addr = self.word().operand5();
        let d = self.gprg(d_addr);
        let res = 0xff - d;
        self.set_gprg(d_addr, res);
        self.set_status_by_bit_instruction(res);
        self.set_status(Sreg::C, false);
        self.pc_increment();
    }

    fn sub(&mut self) {
        let (r_addr, d_addr) = self.word().operand55();
        let (r, d) = self.gprgs(r_addr, d_addr);
        let res = d.wrapping_sub(r);
        self.set_gprg(d_addr, res);
        self.set_status_by_arithmetic_instruction(d, r, res);
        self.set_status(Sreg::C, d < r);
        self.pc_increment();
    }

    fn sbc(&mut self) {
        let (r_addr, d_addr) = self.word().operand55();
        let (r, d) = self.gprgs(r_addr, d_addr);
        let c = self.status_as_u8(Sreg::C);
        let res = d.wrapping_add(r).wrapping_add(c);
        self.set_gprg(d_addr, res);
        self.set_status_by_arithmetic_instruction(d, r, res);
        self.set_status(Sreg::C, d < (r+1));
        self.pc_increment();
    }

    fn subi(&mut self) {
        let (k, d_addr) = self.word().operand84();
        let d = self.gprg(d_addr);
        let res = d.wrapping_sub(k);
        self.set_gprg(d_addr, res);
        // self.set_status_by_arithmetic_instruction(d, r, res);
        self.set_status(Sreg::C, d < k);
        self.pc_increment();
    }

    fn ld1(&mut self) {
        let d_addr = self.word().operand5();
        let x_addr = self.xyz_reg(XYZReg::X);
        self.set_gprg(d_addr, self.gprg(x_addr as usize));
        self.pc_increment();
    }

    fn ld2(&mut self) {
        let d_addr = self.word().operand5();
        let x_addr = self.xyz_reg(XYZReg::X);
        let x = self.gprg(x_addr as usize);
        self.set_gprg(d_addr, x);
        self.set_xyz_reg(XYZReg::X, x_addr + 1u16);
        self.pc_increment();
    }

    fn ld3(&mut self) {
        let d_addr = self.word().operand5();
        let x_addr = self.xyz_reg(XYZReg::X) - 1u16;
        self.set_xyz_reg(XYZReg::X, x_addr);
        let x = self.gprg(x_addr as usize);
        self.set_gprg(d_addr, x);
        self.pc_increment();
    }

    fn ldi(&mut self) {
        let (k, d_addr) = self.word().operand84();
        self.set_gprg(d_addr, k);
        self.pc_increment();
    }

    fn lds(&mut self) {
        let (w, k) = self.double_word();
        let d_addr = w.operand5();
        self.set_gprg(d_addr, k.0 as u8);
        self.pc_double_increment();
    }

    fn out(&mut self) {
        let (a_addr, r_addr) = self.word().operand65();
        let r = self.gprg(r_addr);
        self.set_gprg(a_addr, r);
        self.pc_increment();
    }

    fn in_instr(&mut self) {
        let (a_addr, d_addr) = self.word().operand65();
        let a = self.gprg(a_addr);
        self.set_gprg(d_addr, a);
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
        self.set_pc(w1.operand22(w2));
    }

    // WIP
    fn rcall(&mut self) {
        let k = self.word().operand12() as u32;
        let pc = self.pc();
        self.set_pc(pc+k+1);
    }

    fn jmp(&mut self) {
        let (w1, w2) = self.double_word();
        let k = w1.operand22(w2);
        self.set_pc(k)
    }

    fn rjmp(&mut self) {
        let k = self.word().operand12();
        let pc = self.pc();
        let result = add_12bits_in_twos_complement_form(pc, k) + 1u32;
        self.set_pc(result);
    }

    fn sts(&mut self) {
        let (w1, k) = self.double_word();
        let d_addr = w1.operand5();
        let d = self.gprg(d_addr);
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
        let d_addr = self.word().operand5();
        if ( self.xyz_reg(XYZReg::Z) & 1 ) == 1 {
            self.set_gprg(d_addr, high_bit(self.fetch(self.xyz_reg(XYZReg::Z) as u32)));
        } else {
            self.set_gprg(d_addr, low_bit(self.fetch(self.xyz_reg(XYZReg::Z) as u32)));
        }
        self.pc_increment();
    }

    fn lpm3(&mut self) {
        let d_addr = self.word().operand5();
        if ( self.xyz_reg(XYZReg::Z) & 1 ) == 1 {
            self.set_gprg(d_addr, high_bit(self.fetch(self.xyz_reg(XYZReg::Z) as u32)));
        } else {
            self.set_gprg(d_addr, low_bit(self.fetch(self.xyz_reg(XYZReg::Z) as u32)));
        }
        self.set_xyz_reg(XYZReg::Z, self.xyz_reg(XYZReg::Z)+1);
        self.pc_increment();
    }

    fn st1(&mut self) {
        let d_addr = self.word().operand5();
        let x_addr = self.xyz_reg(XYZReg::X);
        let d = self.gprg(d_addr);
        self.set_gprg(x_addr as usize, d);
        self.pc_increment();
    }

    fn st2(&mut self) {
        let d_addr = self.word().operand5();
        let x_addr = self.xyz_reg(XYZReg::X);
        let d = self.gprg(d_addr);
        self.set_xyz_reg(XYZReg::X, x_addr + 1);
        self.set_gprg(x_addr as usize, d);
        self.pc_increment();
    }

    fn st3(&mut self) {
        let d_addr = self.word().operand5();
        let x_addr = self.xyz_reg(XYZReg::X) - 1;
        let d = self.gprg(d_addr);
        self.set_xyz_reg(XYZReg::X, x_addr);
        self.set_gprg(x_addr as usize, d);
        self.pc_increment();
    }

    fn cp(&mut self) {
        let (r_addr, d_addr) = self.word().operand55();
        let (r, d) = self.gprgs(r_addr, d_addr);
        let res = d.wrapping_sub(r);
        self.set_status_by_arithmetic_instruction(d, r, res);
        self.set_status(Sreg::C, d < r);
        self.pc_increment();
    }

    fn cpi(&mut self) {
        let (k, d_addr) = self.word().operand84();
        let d = self.gprg(d_addr);
        let res = d.wrapping_sub(k);
        // self.set_status_by_arithmetic_instruction(d, r, res);
        self.set_status(Sreg::C, d < k);
        self.pc_increment();
    }

    fn cpc(&mut self) {
        let (r_addr, d_addr) = self.word().operand55();
        let (r, d) = self.gprgs(r_addr, d_addr);
        let c = self.status_as_u8(Sreg::C);
        let res = d.wrapping_sub(r).wrapping_sub(c);
        self.set_status(Sreg::H, has_borrow_from_bit3(d, r, res));
        self.set_status(Sreg::V, has_2complement_overflow(d, r, res));
        self.set_status(Sreg::N, msb(res));
        if res == 0 {
            self.set_status(Sreg::Z, false);
        }
        self.set_status(Sreg::C, d < r+c);
        self.signed_test();
        self.pc_increment();
    }

    fn cpse(&mut self) {
        let (r_addr, d_addr) = self.word().operand55();
        let (r, d) = self.gprgs(r_addr, d_addr);
        if r == d {
            // WIP: ATmega328p is 16bit Program Counter machine...
            self.set_pc(self.pc()+2);
        } else {
            self.pc_increment();
        }
    }

    fn ori(&mut self) {
        let (k, d_addr) = self.word().operand84();
        let d = self.gprg(d_addr);
        let res = d | k;
        self.set_gprg(d_addr, res);
        self.set_status_by_bit_instruction(res);
        self.pc_increment();
    }

    fn and(&mut self) {
        let (r_addr, d_addr) = self.word().operand55();
        let (r, d) = self.gprgs(r_addr, d_addr);
        let res = d & r;
        self.set_gprg(d_addr, res);
        self.set_status_by_bit_instruction(res);
        self.pc_increment();
    }

    fn andi(&mut self) {
        let (k, d_addr) = self.word().operand84();
        let d = self.gprg(d_addr);
        let res = d & k;
        self.set_gprg(d_addr, res);
        self.set_status_by_bit_instruction(res);
        self.pc_increment();
    }

    fn eor(&mut self) {
        let (r_addr, d_addr) = self.word().operand55();
        let (r, d) = self.gprgs(r_addr, d_addr);
        let res = d^r;
        self.set_gprg(d_addr, res);
        self.set_status_by_bit_instruction(res);
        self.pc_increment();
    }

    fn breq(&mut self) {
        if self.status(Sreg::Z) {
            let k = self.word().operand7();
            let pc = self.pc();
            let result = add_7bits_in_twos_complement_form(pc, k) + 1u32;
            self.set_pc(result);
        } else {
            self.pc_increment();
        }
    }

    fn brne(&mut self) {
        if self.status(Sreg::Z) {
            self.pc_increment();
        } else {
            let k = self.word().operand7();
            let pc = self.pc();
            let result = add_7bits_in_twos_complement_form(pc, k) + 1u32;
            self.set_pc(result as u32);
        }
    }

    fn brcs(&mut self) {
        if self.status(Sreg::C) {
            let k = self.word().operand7();
            let pc = self.pc();
            let result = add_7bits_in_twos_complement_form(pc, k) + 1u32;
            self.set_pc(result as u32);
        } else {
            self.pc_increment();
        }
    }

    fn sbis(&mut self) {
        let (a_addr, b) = self.word().operand53();
        // I/O Register starts from 0x20(0x32), so there is offset.
        let a = self.gprg(( a_addr + 0x20 ) as usize);
        if bit(a, b) {
            // WIP: ATmega328p is 16bit Program Counter machine...
            self.set_pc(self.pc()+2);
        } else {
            self.pc_increment();
        }
    }

    fn sbiw(&mut self) {
        let (k, d_addr) = self.word().operand62();
        let (dh, dl) = self.gprgs(d_addr+1, d_addr);
        let result = concat(dh, dl).wrapping_sub(k as u16);
        self.set_gprg(d_addr+1, high_bit(result));
        self.set_gprg(d_addr,   low_bit(result));

        self.set_status(Sreg::V, msb(high_bit(result)) & !msb(dh));
        self.set_status(Sreg::C, msb(high_bit(result)) & !msb(dh));
        self.set_status(Sreg::N, msb(high_bit(result)));
        self.set_status(Sreg::Z, result == 0);
        self.signed_test();
        self.pc_increment();
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
        self.set_pc(pc as u32);
    }

    fn push(&mut self) {
        let d_addr = self.word().operand5();
        let d = self.gprg(d_addr);
        self.push_stack(d);
        self.pc_increment();
    }

    fn pop(&mut self) {
        let d_addr = self.word().operand5();
        let s = self.pop_stack();
        self.set_gprg(d_addr, s);
        self.pc_increment();
    }

    fn mov(&mut self) {
        let (r_addr, d_addr) = self.word().operand55();
        let r = self.gprg(r_addr);
        self.set_gprg(d_addr, r);
        self.pc_increment();
    }

    fn movw(&mut self) {
        let (d_addr, r_addr) = self.word().operand44();
        let (rl, rh) = self.gprgs(r_addr, r_addr+1);
        self.set_gprg(d_addr, rl);
        self.set_gprg(d_addr + 1, rh);
        self.pc_increment();
    }
}
