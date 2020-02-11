use std::fmt::{ LowerHex };
use super::utils::*;
use super::word::*;
use super::instruction::*;

pub trait AVR {
    // Program Counter
    // AVR has 16 or 22 bit program counter.
    // ATmega328p is 16bit PC machine. We need to implement 22bit models soon.
    fn pc(&self) -> u32;

    fn set_pc(&mut self, v: u32);

    fn pc_increment(&mut self) {
        self.set_pc(self.pc() + 1);
    }

    fn pc_double_increment(&mut self) {
        self.set_pc(self.pc() + 2);
    }

    // Stack Pointer
    fn sp(&self) -> u16;

    fn push_stack(&mut self, v: u8);

    fn pop_stack(&mut self) -> u8;

    fn push_pc_stack(&mut self, v: u32) {
        // WIP: ATmega328p is 16bit Program Counter machine...
        let w = ( v & 0xffff ) as u16;
        self.push_stack(high_bit(w));
        self.push_stack(low_bit(w));
    }

    fn pop_pc_stack(&mut self) -> u16 {
        let l = self.pop_stack();
        let h = self.pop_stack();
        concat(h, l)
    }

    // General Purpose Register
    // 先頭の 0~31 要素が汎用レジスタに当たる.
    fn gprg(&self, addr: usize) -> u8;

    fn set_gprg(&mut self, addr: usize, v: u8);

    fn gprgs(&self, addr1: usize, addr2: usize) -> (u8, u8) {
        (self.gprg(addr1), self.gprg(addr2))
    }

    fn preg_addresses(&self, x: Preg) -> (usize, usize) {
        match x {
            Preg::X => (27, 26),
            Preg::Y => (29, 28),
            Preg::Z => (31, 30),
        }
    }

    fn preg(&self, x: Preg) -> u16 {
        let (h, l) = self.preg_addresses(x);
        concat(self.gprg(h), self.gprg(l))
    }

    fn set_preg(&mut self, x: Preg, v: u16) {
        let (h_addr, l_addr) = self.preg_addresses(x);
        self.set_gprg(h_addr, high_bit(v));
        self.set_gprg(l_addr, low_bit(v));
    }

    // Cycle Counter
    fn cycle(&self) -> u64;

    fn cycle_increment(&mut self, v: u64);

    // Fetch 1 word from Program Memory.
    // Program Memory has ~0x8000 address, this is coverd by u16(~0xffff).
    fn fetch(&self, p: u32) -> u16;

    fn word(&self) -> Word {
        Word(self.fetch(self.pc()))
    }

    fn double_word(&self) -> (Word, Word) {
        (Word(self.fetch(self.pc())),
         Word(self.fetch(self.pc()+1)))
    }

    fn sreg(&self) -> u8;

    fn status(&self, s: Sreg) -> bool;

    fn set_status(&mut self, s: Sreg, v: bool);

    fn set_status_by_arithmetic_instruction(&mut self, d: u8, r: u8, res: u8) {
        // WIP: Updating algorithm of status bit is not optimized
        self.set_status(Sreg::H, has_borrow_from_bit3(d, r, res));
        self.set_status(Sreg::V, has_2complement_overflow(d, r, res));
        self.set_status(Sreg::N, msb(res));
        self.set_status(Sreg::Z, res == 0);
        self.signed_test();
    }

    fn set_status_by_bit_instruction(&mut self, res: u8) {
        // WIP: Updating algorithm of status bit is not optimized
        self.set_status(Sreg::V, false);
        self.set_status(Sreg::N, msb(res));
        self.set_status(Sreg::Z, res == 0);
        self.signed_test();
    }

    fn signed_test(&mut self) {
        let s = self.status(Sreg::V) ^ self.status(Sreg::N);
        self.set_status(Sreg::S, s);
    }

    fn view_processor_status(&self, instruction: &Instr) {
        print!("\x1B[2J"); // clear console
        println!(
r#"
Program Counter: {:#08x} (Hexfile = {:x})
Stack Pointer:   {:#04x}
X Register:      {:#04x}
Y Register:      {:#04x}
Z Register:      {:#04x}
Status Register: {:08b}
Cycle Counter:   {}
instruction:     {:?} ({:#04x})
"#,
            self.pc(), self.pc()*2,
            self.sp(),
            self.preg(Preg::X),
            self.preg(Preg::Y),
            self.preg(Preg::Z),
            self.sreg(),
            self.cycle(),
            instruction,
            self.word().0,
        );
    }

    fn view_registers(&self) {
        for i in 0..8 {
            let i = i * 4;
            println!(
                "R{:02} = {:#04x}, R{:02} = {:#04x}, R{:02} = {:#04x}, R{:02} = {:#04x},",
                i,   self.gprg(i),
                i+1, self.gprg(i+1),
                i+2, self.gprg(i+2),
                i+3, self.gprg(i+3)
            );
        }
    }

}

// Status Register
#[derive(Eq, PartialEq, Debug)]
pub enum Sreg { C, Z, N, V, S, H, T, I }

// Pointer Register
#[derive(Eq, PartialEq, Debug)]
pub enum Preg { X, Y, Z }

pub trait Memory<T>
where T: LowerHex
{
    fn get(&self, a: usize) -> T;
    fn set(&mut self, a: usize, v: T);

    fn view_memory(&self, unit: u8, length: usize) {
        print!("\x1B[2J"); // clear console
        for i in 0..length {
            let i = i * 8;
            if unit == 2 {
                println!(
                    "{:#06x} | {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x}",
                    i,
                    self.get(i+0), self.get(i+1), self.get(i+2), self.get(i+3),
                    self.get(i+4), self.get(i+5), self.get(i+6), self.get(i+7),
                );
            } else if unit == 4 {
                println!(
                    "{:#06x} | {:04x} {:04x} {:04x} {:04x} {:04x} {:04x} {:04x} {:04x}",
                    i*2,
                    self.get(i+0), self.get(i+1), self.get(i+2), self.get(i+3),
                    self.get(i+4), self.get(i+5), self.get(i+6), self.get(i+7),
                );
            } else {
                return
            };
        }
    }
}
