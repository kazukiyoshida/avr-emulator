use super::flash_memory::*;
use super::sram::*;

#[rustfmt::skip]
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum Instr {
    ADD, ADC, ADIW, SUB, SBC, SUBI, SBCI, SBIW, DEC, COM, LD1, LD2, LD3, LDI,
    LDDY1, LDDY2, LDDY3, LDDZ1, LDDZ2, LDDZ3, LDS, OUT, IN, NOP, CALL, RCALL,
    ROL, LSL, JMP, RJMP, AND, ANDI, OR, EOR, ORI, STS, ST1, ST2, ST3, STY1,
    STY2, STY3, STZ1, STZ2, STZ3, LPM1, LPM2, LPM3, CP, CPI, CPC, CPSE, BREQ,
    BRNE, BRCS, SBIS, SEI, CLI, RET, PUSH, POP, MOV, MOVW,
}

#[rustfmt::skip]
pub const INSTRUCTION_32_BIT: [Instr; 4] = [
    Instr::CALL, Instr::JMP, Instr::LDS, Instr::STS,
];

pub type InstrFunc = &'static dyn Fn(&mut SRAM, &FlashMemory, usize, u64) -> (usize, u64);

pub fn add(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    let (r_addr, d_addr) = flash_memory.word(pc).operand55();
    let (r, d) = sram.gets(r_addr, d_addr);
    let res = r.wrapping_add(d);
    sram.set(d_addr, res);
    // avr.set_status_by_arithmetic_instruction(d, r, res);
    // avr.set_bit(avr.b().c, has_borrow_from_msb(r, d, res));
    (pc + 1, cycle + 1)
}
