use super::avr::*;
use super::atmega328p::*;
use super::instruction::*;
use super::utils::*;
use super::word::*;

type Tree = Option<Box<Node>>;
type InstrFunc = &'static Fn(&mut AVR);
type Opcode = (u16, u16);

pub struct Node {
    pub depth: u8,
    pub on: Tree,
    pub off: Tree,
    pub undef: Tree,
    pub f: Option<InstrFunc>,
}

impl Node {
    pub fn new() -> Node {
        Node {
            depth: 0,
            on: None,
            off: None,
            undef: None,
            f: None,
        }
    }

    pub fn add(&mut self, opcode: Opcode, f: &'static Fn(&mut AVR)) {
        self.insert(0, opcode, f);
    }

    fn insert(
        &mut self,
        depth: u8,
        opcode: Opcode,
        f: &'static Fn(&mut AVR)
    ) {
        if depth >= 15 {
            self.f = Some(f);
            return
        }

        let should_eval =  nth_bit_from_left_u16(opcode.1, depth);
        let is_on =  nth_bit_from_left_u16(opcode.0, depth);

        match (should_eval, is_on) {
            (true, true) => {
                match &mut self.on {
                    Some(n) => n.insert(depth+1, opcode, f),
                    None => {
                        let mut n = Node::new();
                        n.depth = depth + 1;
                        n.insert(depth + 1, opcode, f);
                        self.on = Some(Box::new(n));
                    },
                }
            },
            (true, false) => {
                match &mut self.off {
                    Some(n) => n.insert(depth+1, opcode, f),
                    None => {
                        let mut n = Node::new();
                        n.depth = depth + 1;
                        n.insert(depth + 1, opcode, f);
                        self.off = Some(Box::new(n));
                    },
                }
            },
            (false, _) => {
                match &mut self.undef {
                    Some(n) => n.insert(depth+1, opcode, f),
                    None => {
                        let mut n = Node::new();
                        n.depth = depth + 1;
                        n.insert(depth + 1, opcode, f);
                        self.undef = Some(Box::new(n));
                    },
                }
            },
        }
    }

    pub fn find(&self, word: Word) -> InstrFunc {
        let w = word.0;
        self.find_recursive(w, 0).unwrap()
    }

    fn find_recursive(&self, w: u16, depth: u8) -> Option<InstrFunc> {
        if depth >= 15 {
            return self.f
        }

        if nth_bit_from_left_u16(w, depth) {
            match &self.on {
                Some(n) => n.find_recursive(w, depth+1),
                None => {
                    match &self.undef {
                        Some(n) => n.find_recursive(w, depth+1),
                        None => panic!("there is no on & undef"),
                    }
                },
            }
        } else {
            match &self.off {
                Some(n) => n.find_recursive(w, depth+1),
                None => {
                    match &self.undef {
                        Some(n) => n.find_recursive(w, depth+1),
                        None => panic!("there is no off & undef"),
                    }
                },
            }
        }
    }
}

thread_local! {
    pub static OPCODE_TREE: Node = {
        let mut t = Node::new();
        t.add((0b0000_1100_0000_0000, 0b1111_1100_0000_0000), &add);
        t.add((0b0001_1100_0000_0000, 0b1111_1100_0000_0000), &adc);
        t.add((0b1001_0110_0000_0000, 0b1111_1111_0000_0000), &adiw);
        t.add((0b0001_1000_0000_0000, 0b1111_1100_0000_0000), &sub);
        t.add((0b0000_1000_0000_0000, 0b1111_1100_0000_0000), &sbc);
        t.add((0b0101_0000_0000_0000, 0b1111_0000_0000_0000), &subi);
        t.add((0b0100_0000_0000_0000, 0b1111_0000_0000_0000), &sbci);
        t.add((0b1001_0111_0000_0000, 0b1111_1111_0000_0000), &sbiw);
        t.add((0b1001_0100_0000_1010, 0b1111_1110_0000_1111), &dec);
        t.add((0b1001_0100_0000_0000, 0b1111_1110_0000_1111), &com);
        t.add((0b1110_0000_0000_0000, 0b1111_0000_0000_0000), &ldi);
        t.add((0b1001_0000_0000_1100, 0b1111_1110_0000_1111), &ld1);
        t.add((0b1001_0000_0000_1101, 0b1111_1110_0000_1111), &ld2);
        t.add((0b1001_0000_0000_1110, 0b1111_1110_0000_1111), &ld3);
        t.add((0b1000_0000_0000_1000, 0b1111_1110_0000_1111), &lddy1);
        t.add((0b1001_0000_0000_1001, 0b1111_1110_0000_1111), &lddy2);
        t.add((0b1001_0000_0000_1010, 0b1111_1110_0000_1111), &lddy3);
        t.add((0b1000_0000_0000_0000, 0b1111_1110_0000_1111), &lddz1);
        t.add((0b1001_0000_0000_0001, 0b1111_1110_0000_1111), &lddz2);
        t.add((0b1001_0000_0000_0010, 0b1111_1110_0000_1111), &lddz3);
        t.add((0b1001_0000_0000_0000, 0b1111_1110_0000_1111), &lds);
        t.add((0b1011_1000_0000_0000, 0b1111_1000_0000_0000), &out);
        t.add((0b1011_0000_0000_0000, 0b1111_1000_0000_0000), &in_instr);
        t.add((0b0000_0000_0000_0000, 0b1111_1111_1111_1111), &nop);
        t.add((0b1001_0100_0000_1110, 0b1111_1110_0000_1110), &call);
        t.add((0b1101_0000_0000_0000, 0b1111_0000_0000_0000), &rcall);
        t.add((0b0001_1100_0000_0000, 0b1111_1100_0000_0000), &rol);
        t.add((0b0000_1100_0000_0000, 0b1111_1100_0000_0000), &lsl);
        t.add((0b1001_0100_0000_1100, 0b1111_1110_0000_1110), &jmp);
        t.add((0b1100_0000_0000_0000, 0b1111_0000_0000_0000), &rjmp);
        t.add((0b0110_0000_0000_0000, 0b1111_0000_0000_0000), &ori);
        t.add((0b0010_0000_0000_0000, 0b1111_1100_0000_0000), &and);
        t.add((0b0111_0000_0000_0000, 0b1111_0000_0000_0000), &andi);
        t.add((0b0010_1000_0000_0000, 0b1111_1100_0000_0000), &or);
        t.add((0b0010_0100_0000_0000, 0b1111_1100_0000_0000), &eor);
        t.add((0b1001_0010_0000_0000, 0b1111_1110_0000_1111), &sts);
        t.add((0b1001_0010_0000_1100, 0b1111_1110_0000_1111), &st1);
        t.add((0b1001_0010_0000_1101, 0b1111_1110_0000_1111), &st2);
        t.add((0b1001_0010_0000_1110, 0b1111_1110_0000_1111), &st3);
        t.add((0b1000_0010_0000_1000, 0b1111_1110_0000_1111), &sty1);
        t.add((0b1001_0010_0000_1001, 0b1111_1110_0000_1111), &sty2);
        t.add((0b1001_0010_0000_1010, 0b1111_1110_0000_1111), &sty3);
        t.add((0b1000_0010_0000_0000, 0b1111_1110_0000_1111), &stz1);
        t.add((0b1001_0010_0000_0001, 0b1111_1110_0000_1111), &stz2);
        t.add((0b1001_0010_0000_0010, 0b1111_1110_0000_1111), &stz3);
        t.add((0b1001_0101_1100_1000, 0b1111_1111_1111_1111), &lpm1);
        t.add((0b1001_0000_0000_0100, 0b1111_1110_0000_1111), &lpm2);
        t.add((0b1001_0000_0000_0101, 0b1111_1110_0000_1111), &lpm3);
        t.add((0b0001_0100_0000_0000, 0b1111_1100_0000_0000), &cp);
        t.add((0b0011_0000_0000_0000, 0b1111_0000_0000_0000), &cpi);
        t.add((0b0000_0100_0000_0000, 0b1111_1100_0000_0000), &cpc);
        t.add((0b0001_0000_0000_0000, 0b1111_1100_0000_0000), &cpse);
        t.add((0b1111_0000_0000_0001, 0b1111_1100_0000_0111), &breq);
        t.add((0b1111_0100_0000_0001, 0b1111_1100_0000_0111), &brne);
        t.add((0b1111_0000_0000_0000, 0b1111_1100_0000_0111), &brcs);
        t.add((0b1001_1011_0000_0000, 0b1111_1111_0000_0000), &sbis);
        t.add((0b1001_0100_0111_1000, 0b1111_1111_1111_1111), &sei);
        t.add((0b1001_0100_1111_1000, 0b1111_1111_1111_1111), &cli);
        t.add((0b1001_0101_0000_1000, 0b1111_1111_1111_1111), &ret);
        t.add((0b1001_0010_0000_1111, 0b1111_1110_0000_1111), &push);
        t.add((0b1001_0000_0000_1111, 0b1111_1110_0000_1111), &pop);
        t.add((0b0010_1100_0000_0000, 0b1111_1100_0000_0000), &mov);
        t.add((0b0000_0001_0000_0000, 0b1111_1111_0000_0000), &movw);
        t
    };
}

#[test]
fn test_node() {
    &OPCODE_TREE.with(|f| {
        let f1 = f.find(Word(0b0000_1100_0000_0000));
        let f2 = f.find(Word(0b0001_1100_0000_0000));
        // let f3 = f.find(Word(0b1111_1100_0000_0000)); // panic
    });
}
