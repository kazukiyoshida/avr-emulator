use super::instruction::*;
use super::util::bit::*;
use std::fmt;

thread_local! {
    #[rustfmt::skip]
    pub static OPCODE_TREE: Node = {
        let mut t: Node = Default::default();
        t.add((0b0000_1100_0000_0000, 0b1111_1100_0000_0000), Instr::ADD, &add);
        t.add((0b0001_1100_0000_0000, 0b1111_1100_0000_0000), Instr::ADC, &adc);
        t.add((0b1001_0110_0000_0000, 0b1111_1111_0000_0000), Instr::ADIW, &adiw);
        t.add((0b0001_1000_0000_0000, 0b1111_1100_0000_0000), Instr::SUB, &sub);
        t.add((0b0000_1000_0000_0000, 0b1111_1100_0000_0000), Instr::SBC, &sbc);
        t.add((0b0101_0000_0000_0000, 0b1111_0000_0000_0000), Instr::SUBI, &subi);
        t.add((0b0100_0000_0000_0000, 0b1111_0000_0000_0000), Instr::SBCI, &sbci);
        t.add((0b1001_0111_0000_0000, 0b1111_1111_0000_0000), Instr::SBIW, &sbiw);
        t.add((0b1001_0100_0000_1010, 0b1111_1110_0000_1111), Instr::DEC, &dec);
        t.add((0b1001_0100_0000_0000, 0b1111_1110_0000_1111), Instr::COM, &com);
        t.add((0b1110_0000_0000_0000, 0b1111_0000_0000_0000), Instr::LDI, &ldi);
        t.add((0b1001_0000_0000_1100, 0b1111_1110_0000_1111), Instr::LD1, &ld1);
        t.add((0b1001_0000_0000_1101, 0b1111_1110_0000_1111), Instr::LD2, &ld2);
        t.add((0b1001_0000_0000_1110, 0b1111_1110_0000_1111), Instr::LD3, &ld3);
        t.add((0b1000_0000_0000_1000, 0b1111_1110_0000_1111), Instr::LDDY1, &lddy1);
        t.add((0b1001_0000_0000_1001, 0b1111_1110_0000_1111), Instr::LDDY2, &lddy2);
        t.add((0b1001_0000_0000_1010, 0b1111_1110_0000_1111), Instr::LDDY3, &lddy3);
        t.add((0b1000_0000_0000_0000, 0b1111_1110_0000_1111), Instr::LDDZ1, &lddz1);
        t.add((0b1001_0000_0000_0001, 0b1111_1110_0000_1111), Instr::LDDZ2, &lddz2);
        t.add((0b1001_0000_0000_0010, 0b1111_1110_0000_1111), Instr::LDDZ3, &lddz3);
        t.add((0b1001_0000_0000_0000, 0b1111_1110_0000_1111), Instr::LDS, &lds);
        t.add((0b1011_1000_0000_0000, 0b1111_1000_0000_0000), Instr::OUT, &out);
        t.add((0b1011_0000_0000_0000, 0b1111_1000_0000_0000), Instr::IN, &in_instr);
        t.add((0b0000_0000_0000_0000, 0b1111_1111_1111_1111), Instr::NOP, &nop);
        t.add((0b1001_0100_0000_1110, 0b1111_1110_0000_1110), Instr::CALL, &call);
        t.add((0b1101_0000_0000_0000, 0b1111_0000_0000_0000), Instr::RCALL, &rcall);
        // t.add((0b0001_1100_0000_0000, 0b1111_1100_0000_0000), Instr::ROL, &rol);
        // t.add((0b0000_1100_0000_0000, 0b1111_1100_0000_0000), Instr::LSL, &lsl);
        t.add((0b1001_0100_0000_1100, 0b1111_1110_0000_1110), Instr::JMP, &jmp);
        t.add((0b1100_0000_0000_0000, 0b1111_0000_0000_0000), Instr::RJMP, &rjmp);
        t.add((0b0110_0000_0000_0000, 0b1111_0000_0000_0000), Instr::ORI, &ori);
        t.add((0b0010_0000_0000_0000, 0b1111_1100_0000_0000), Instr::AND, &and);
        t.add((0b0111_0000_0000_0000, 0b1111_0000_0000_0000), Instr::ANDI, &andi);
        t.add((0b0010_1000_0000_0000, 0b1111_1100_0000_0000), Instr::OR, &or);
        t.add((0b0010_0100_0000_0000, 0b1111_1100_0000_0000), Instr::EOR, &eor);
        t.add((0b1001_0010_0000_0000, 0b1111_1110_0000_1111), Instr::STS, &sts);
        t.add((0b1001_0010_0000_1100, 0b1111_1110_0000_1111), Instr::ST1, &st1);
        t.add((0b1001_0010_0000_1101, 0b1111_1110_0000_1111), Instr::ST2, &st2);
        t.add((0b1001_0010_0000_1110, 0b1111_1110_0000_1111), Instr::ST3, &st3);
        t.add((0b1000_0010_0000_1000, 0b1111_1110_0000_1111), Instr::STY1, &sty1);
        t.add((0b1001_0010_0000_1001, 0b1111_1110_0000_1111), Instr::STY2, &sty2);
        t.add((0b1001_0010_0000_1010, 0b1111_1110_0000_1111), Instr::STY3, &sty3);
        t.add((0b1000_0010_0000_0000, 0b1111_1110_0000_1111), Instr::STZ1, &stz1);
        t.add((0b1001_0010_0000_0001, 0b1111_1110_0000_1111), Instr::STZ2, &stz2);
        t.add((0b1001_0010_0000_0010, 0b1111_1110_0000_1111), Instr::STZ3, &stz3);
        t.add((0b1001_0101_1100_1000, 0b1111_1111_1111_1111), Instr::LPM1, &lpm1);
        t.add((0b1001_0000_0000_0100, 0b1111_1110_0000_1111), Instr::LPM2, &lpm2);
        t.add((0b1001_0000_0000_0101, 0b1111_1110_0000_1111), Instr::LPM3, &lpm3);
        t.add((0b0001_0100_0000_0000, 0b1111_1100_0000_0000), Instr::CP, &cp);
        t.add((0b0011_0000_0000_0000, 0b1111_0000_0000_0000), Instr::CPI, &cpi);
        t.add((0b0000_0100_0000_0000, 0b1111_1100_0000_0000), Instr::CPC, &cpc);
        t.add((0b0001_0000_0000_0000, 0b1111_1100_0000_0000), Instr::CPSE, &cpse);
        t.add((0b1111_0000_0000_0001, 0b1111_1100_0000_0111), Instr::BREQ, &breq);
        t.add((0b1111_0100_0000_0001, 0b1111_1100_0000_0111), Instr::BRNE, &brne);
        t.add((0b1111_0000_0000_0000, 0b1111_1100_0000_0111), Instr::BRCS, &brcs);
        t.add((0b1001_1011_0000_0000, 0b1111_1111_0000_0000), Instr::SBIS, &sbis);
        t.add((0b1001_0100_0111_1000, 0b1111_1111_1111_1111), Instr::SEI, &sei);
        t.add((0b1001_0100_1111_1000, 0b1111_1111_1111_1111), Instr::CLI, &cli);
        t.add((0b1001_0101_0000_1000, 0b1111_1111_1111_1111), Instr::RET, &ret);
        t.add((0b1001_0010_0000_1111, 0b1111_1110_0000_1111), Instr::PUSH, &push);
        t.add((0b1001_0000_0000_1111, 0b1111_1110_0000_1111), Instr::POP, &pop);
        t.add((0b0010_1100_0000_0000, 0b1111_1100_0000_0000), Instr::MOV, &mov);
        t.add((0b0000_0001_0000_0000, 0b1111_1111_0000_0000), Instr::MOVW, &movw);
        t
    };
}

type Tree = Option<Box<Node>>;
type Opcode = (u16, u16);

#[derive(Default)]
pub struct Node {
    depth: u8,
    on: Tree,
    off: Tree,
    undef: Tree,
    instr: Option<Instr>,
    f: Option<InstrFunc>,
}

impl Node {
    fn add(&mut self, opcode: Opcode, instr: Instr, f: InstrFunc) {
        self.insert(0, opcode, instr, f);
    }

    fn insert(&mut self, depth: u8, opcode: Opcode, instr: Instr, f: InstrFunc) {
        if depth >= 16 {
            self.instr = Some(instr);
            self.f = Some(f);
            return;
        }

        let is_eval = nth_bit_from_left_u16(opcode.1, depth);
        let is_on = nth_bit_from_left_u16(opcode.0, depth);

        match (is_eval, is_on) {
            (true, true) => match &mut self.on {
                Some(n) => n.insert(depth + 1, opcode, instr, f),
                None => {
                    let mut n: Node = Default::default();
                    n.depth = depth + 1;
                    n.insert(depth + 1, opcode, instr, f);
                    self.on = Some(Box::new(n));
                }
            },
            (true, false) => match &mut self.off {
                Some(n) => n.insert(depth + 1, opcode, instr, f),
                None => {
                    let mut n: Node = Default::default();
                    n.depth = depth + 1;
                    n.insert(depth + 1, opcode, instr, f);
                    self.off = Some(Box::new(n));
                }
            },
            (false, _) => match &mut self.undef {
                Some(n) => n.insert(depth + 1, opcode, instr, f),
                None => {
                    let mut n: Node = Default::default();
                    n.depth = depth + 1;
                    n.insert(depth + 1, opcode, instr, f);
                    self.undef = Some(Box::new(n));
                }
            },
        }
    }

    pub fn find(&self, word: u16) -> (Instr, InstrFunc) {
        self.find_recursive(word, 0)
            .unwrap_or_else(|| panic!("there is no instruction, w: {:016b}", word))
    }

    fn find_recursive(&self, w: u16, depth: u8) -> Option<(Instr, InstrFunc)> {
        if depth >= 16 {
            return Some((
                self.instr.expect("instr not found"),
                self.f.expect("instr function not found"),
            ));
        }

        return if nth_bit_from_left_u16(w, depth) {
            self.on
                .as_ref()
                .and_then(|n| n.find_recursive(w, depth + 1))
                .or(self
                    .undef
                    .as_ref()
                    .and_then(|n| n.find_recursive(w, depth + 1)))
        } else {
            self.off
                .as_ref()
                .and_then(|n| n.find_recursive(w, depth + 1))
                .or(self
                    .undef
                    .as_ref()
                    .and_then(|n| n.find_recursive(w, depth + 1)))
        };
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let instr = match &self.instr {
            Some(instr) => format!(", \"instr\": \"{:?}\"", instr),
            None => "".to_string(),
        };
        let on = match &self.on {
            Some(n) => format!(", \"on\": {}", n),
            None => "".to_string(),
        };
        let off = match &self.off {
            Some(n) => format!(", \"off\": {}", n),
            None => "".to_string(),
        };
        let undef = match &self.undef {
            Some(n) => format!(", \"undef\": {}", n),
            None => "".to_string(),
        };
        write!(
            f,
            "{{\"depth\" :{}{}{}{}{}}}",
            self.depth, instr, on, off, undef
        )
    }
}

#[test]
fn test_node() {
    &OPCODE_TREE.with(|f| {
        assert_eq!(Instr::ADD, f.find(0b0000_1100_0000_0000).0);
        assert_eq!(Instr::ADC, f.find(0b0001_1100_0000_0000).0);
        assert_eq!(Instr::JMP, f.find(0b1001_0100_0000_1100).0);
        assert_eq!(Instr::SEI, f.find(0b1001_0100_0111_1000).0);
        assert_eq!(Instr::STS, f.find(0b1001_0010_0000_0000).0);
    });
}

#[test]
#[should_panic]
fn test_node_panic() {
    &OPCODE_TREE.with(|f| {
        let _xxx = f.find(0b1111_1100_0000_0000);
    });
}
