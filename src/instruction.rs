use super::flash_memory::*;
use super::opcode_tree::*;
use super::sram::*;
use super::util::bit::*;

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
    sram.set_status_by_arithmetic_instruction(d, r, res);
    sram.set_bit(sram.bit_map.c, has_borrow_from_msb(r, d, res));
    (pc + 1, cycle + 1)
}

pub fn adc(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    let (r_addr, d_addr) = flash_memory.word(pc).operand55();
    let (r, d) = sram.gets(r_addr, d_addr);
    let c = sram.get_bit(sram.bit_map.c) as u8;
    let res = r.wrapping_add(d).wrapping_add(c);
    sram.set(d_addr, res);
    sram.set_status_by_arithmetic_instruction(d, r, res);
    sram.set_bit(sram.bit_map.c, has_borrow_from_msb(r, d, res));
    (pc + 1, cycle + 1)
}

pub fn adiw(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    let (k, d_addr) = flash_memory.word(pc).operand62();
    let (dh, dl) = sram.gets(d_addr + 1, d_addr);
    let res = concat(dh, dl).wrapping_add(k as u16);
    sram.set(d_addr, high_byte(res));
    sram.set(d_addr + 1, low_byte(res));

    sram.set_bit(sram.bit_map.v, !msb(dh) & msb(high_byte(res)));
    sram.set_bit(sram.bit_map.n, msb(high_byte(res)));
    sram.set_bit(sram.bit_map.z, res == 0);
    sram.set_bit(sram.bit_map.c, !msb(high_byte(res)) & msb(dh));
    sram.set_bit(sram.bit_map.s, sram.signed_test());

    (pc + 1, cycle + 1)
}

pub fn sbci(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    let (k, d_addr) = flash_memory.word(pc).operand84();
    let d = sram.get(d_addr);
    let c = sram.get_bit(sram.bit_map.c) as u8;
    let res = d.wrapping_sub(k).wrapping_sub(c);
    sram.set(d_addr, res);

    sram.set_bit(sram.bit_map.h, has_borrow_from_bit3_k(d, k, res));
    sram.set_bit(sram.bit_map.v, has_2complement_overflow_2(d, k, res));
    sram.set_bit(sram.bit_map.n, msb(res));
    if res != 0 {
        sram.set_bit(sram.bit_map.z, false);
    };
    match d.checked_sub(k).and_then(|d_k| d_k.checked_sub(c)) {
        None => sram.set_bit(sram.bit_map.c, true),
        _ => sram.set_bit(sram.bit_map.c, false),
    };
    sram.set_bit(sram.bit_map.s, sram.signed_test());

    (pc + 1, cycle + 1)
}

pub fn dec(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    let d_addr = flash_memory.word(pc).operand5();
    let d = sram.get(d_addr);
    let result = d.wrapping_sub(1);
    sram.set(d_addr, result);

    sram.set_bit(sram.bit_map.v, d == 0x80u8);
    sram.set_bit(sram.bit_map.n, msb(result));
    sram.set_bit(sram.bit_map.z, result == 0);
    sram.set_bit(sram.bit_map.s, sram.signed_test());

    (pc + 1, cycle + 1)
}

pub fn com(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    let d_addr = flash_memory.word(pc).operand5();
    let d = sram.get(d_addr);
    let res = 0xff - d;
    sram.set(d_addr, res);
    sram.set_status_by_bit_instruction(res);
    sram.set_bit(sram.bit_map.c, false);
    (pc + 1, cycle + 1)
}

pub fn sub(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    let (r_addr, d_addr) = flash_memory.word(pc).operand55();
    let (r, d) = sram.gets(r_addr, d_addr);
    let res = d.wrapping_sub(r);
    sram.set(d_addr, res);
    sram.set_status_by_arithmetic_instruction2(d, r, res);
    sram.set_bit(sram.bit_map.c, d < r);
    (pc + 1, cycle + 1)
}

pub fn sbc(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    let (r_addr, d_addr) = flash_memory.word(pc).operand55();
    let (r, d) = sram.gets(r_addr, d_addr);
    let c = sram.get_bit(sram.bit_map.c) as u8;
    let res = d.wrapping_sub(r).wrapping_sub(c);
    sram.set(d_addr, res);

    sram.set_bit(sram.bit_map.h, has_borrow_from_bit3_k(d, r, res));
    sram.set_bit(sram.bit_map.v, has_2complement_overflow(d, r, res));
    sram.set_bit(sram.bit_map.n, msb(res));
    if res != 0 {
        sram.set_bit(sram.bit_map.z, false);
    }
    sram.set_bit(sram.bit_map.s, sram.signed_test());
    sram.set_bit(sram.bit_map.c, d < (r.wrapping_add(c)));

    (pc + 1, cycle + 1)
}

pub fn subi(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    let (k, d_addr) = flash_memory.word(pc).operand84();
    let d = sram.get(d_addr);
    let res = d.wrapping_sub(k);
    sram.set(d_addr, res);
    sram.set_status_by_arithmetic_instruction2(d, k, res);
    sram.set_bit(sram.bit_map.c, d < k);
    (pc + 1, cycle + 1)
}

pub fn ld1(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    let d_addr = flash_memory.word(pc).operand5();
    let x_addr = sram.get_word(sram.word_map.x);
    sram.set(d_addr, sram.get(x_addr as usize));
    (pc + 1, cycle + 2) // 割り込みの有無が影響する
}

pub fn ld2(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    let d_addr = flash_memory.word(pc).operand5();
    let x_addr = sram.get_word(sram.word_map.x);
    let x = sram.get(x_addr as usize);
    sram.set(d_addr, x);
    sram.set_word(sram.word_map.x, x_addr + 1);
    (pc + 1, cycle + 2)
}

pub fn ld3(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    let d_addr = flash_memory.word(pc).operand5();
    let x_addr = sram.get_word(sram.word_map.x) - 1;
    sram.set_word(sram.word_map.x, x_addr);
    let x = sram.get(x_addr as usize);
    sram.set(d_addr, x);
    (pc + 1, cycle + 3)
}

pub fn ldi(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    let (k, d_addr) = flash_memory.word(pc).operand84();
    sram.set(d_addr, k);
    (pc + 1, cycle + 1)
}

pub fn lds(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    let (w, k_addr) = flash_memory.double_word(pc);
    let d_addr = w.operand5();
    let k = sram.get(k_addr.0 as usize);
    sram.set(d_addr, k);
    (pc + 2, cycle + 2)
}

pub fn lddy1(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    let d_addr = flash_memory.word(pc).operand5();
    let y_addr = sram.get_word(sram.word_map.y);
    sram.set(d_addr, sram.get(y_addr as usize));
    (pc + 1, cycle + 2) // 1 cycles in Manual
}

pub fn lddy2(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    let d_addr = flash_memory.word(pc).operand5();
    let y_addr = sram.get_word(sram.word_map.y);
    sram.set(d_addr, sram.get(y_addr as usize));
    sram.set_word(sram.word_map.y, y_addr + 1);
    (pc + 1, cycle + 2)
}

pub fn lddy3(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    let d_addr = flash_memory.word(pc).operand5();
    let y_addr = sram.get_word(sram.word_map.y) - 1;
    sram.set_word(sram.word_map.y, y_addr);
    sram.set(d_addr, sram.get(y_addr as usize));
    (pc + 1, cycle + 2)
}

pub fn lddz1(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    let d_addr = flash_memory.word(pc).operand5();
    let z_addr = sram.get_word(sram.word_map.z);
    sram.set(d_addr, sram.get(z_addr as usize));
    (pc + 1, cycle + 2) // 1 cycles in Manual
}

pub fn lddz2(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    let d_addr = flash_memory.word(pc).operand5();
    let z_addr = sram.get_word(sram.word_map.z);
    sram.set(d_addr, sram.get(z_addr as usize));
    sram.set_word(sram.word_map.z, z_addr + 1);
    (pc + 1, cycle + 2)
}

pub fn lddz3(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    let d_addr = flash_memory.word(pc).operand5();
    let z_addr = sram.get_word(sram.word_map.z) - 1;
    sram.set_word(sram.word_map.z, z_addr);
    sram.set(d_addr, sram.get(z_addr as usize));
    (pc + 1, cycle + 2)
}

pub fn out(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    let (a_addr, r_addr) = flash_memory.word(pc).operand65();
    let r = sram.get(r_addr);
    sram.set(a_addr, r);
    (pc + 1, cycle + 1)
}

pub fn in_instr(
    sram: &mut SRAM,
    flash_memory: &FlashMemory,
    pc: usize,
    cycle: u64,
) -> (usize, u64) {
    let (a_addr, d_addr) = flash_memory.word(pc).operand65();
    let a = sram.get(a_addr);
    if a_addr == 0x5f {
        // SREG
        sram.set(d_addr, a & 0b111_1111);
    } else {
        sram.set(d_addr, a);
    };
    (pc + 1, cycle + 1)
}

pub fn nop(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    (pc + 1, cycle + 1)
}

pub fn call(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    sram.push_pc_stack(pc + 2);
    let (w1, w2) = flash_memory.double_word(pc);
    (w1.operand22(w2) as usize, cycle + 4)
}

pub fn rol(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    let d_addr = flash_memory.word(pc).operand10() as usize;
    let d_old = sram.get(d_addr);
    let c = sram.get_bit(sram.bit_map.c) as u8;
    let d_new = (d_old << 1) | c;
    sram.set(d_addr, d_new);

    sram.set_bit(sram.bit_map.h, (d_old & 0b0000_1000) >> 3 == 1);
    sram.set_bit(sram.bit_map.n, msb(d_new));
    sram.set_bit(sram.bit_map.z, d_new == 0);
    sram.set_bit(sram.bit_map.c, msb(d_old));
    sram.set_bit(
        sram.bit_map.v,
        sram.get_bit(sram.bit_map.n) ^ sram.get_bit(sram.bit_map.c),
    );
    sram.set_bit(sram.bit_map.s, sram.signed_test());

    (pc + 1, cycle + 3)
}

pub fn lsl(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    let d_addr = flash_memory.word(pc).operand10() as usize;
    let d_old = sram.get(d_addr);
    let d_new = d_old << 1;
    sram.set(d_addr, d_new);

    sram.set_bit(sram.bit_map.h, (d_old & 0b0000_1000) >> 3 == 1);
    sram.set_bit(sram.bit_map.n, msb(d_new));
    sram.set_bit(sram.bit_map.z, d_new == 0);
    sram.set_bit(sram.bit_map.c, msb(d_old));
    sram.set_bit(
        sram.bit_map.v,
        sram.get_bit(sram.bit_map.n) ^ sram.get_bit(sram.bit_map.c),
    );
    sram.set_bit(sram.bit_map.s, sram.signed_test());

    (pc + 1, cycle + 3)
}

pub fn rcall(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    let k = flash_memory.word(pc).operand12() as u32;
    let pc = pc;
    (pc + k as usize + 1, cycle + 3)
}

pub fn jmp(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    let cycle_diff = if pc == 0 { 2 } else { 3 };
    let (w1, w2) = flash_memory.double_word(pc);
    let k = w1.operand22(w2);
    (k as usize, cycle + cycle_diff)
}

pub fn rjmp(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    let k = flash_memory.word(pc).operand12();
    let pc = pc;
    let result = add_12bits_in_twos_complement_form(pc as u32, k) + 1u32;
    (result as usize, cycle + 2)
}

pub fn sts(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    let (w1, k) = flash_memory.double_word(pc);
    let d_addr = w1.operand5();
    let d = sram.get(d_addr);
    sram.set(k.0 as usize, d);
    (pc + 2, cycle + 2)
}

pub fn lpm1(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    let z_addr = sram.get_word(sram.word_map.z);
    sram.set(0, flash_memory.z_program_memory(z_addr));
    (pc + 1, cycle + 3)
}

pub fn lpm2(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    let d_addr = flash_memory.word(pc).operand5();
    let z_addr = sram.get_word(sram.word_map.z);
    sram.set(d_addr, flash_memory.z_program_memory(z_addr));
    (pc + 1, cycle + 3)
}

pub fn lpm3(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    let d_addr = flash_memory.word(pc).operand5();
    let z_addr = sram.get_word(sram.word_map.z);
    sram.set(d_addr, flash_memory.z_program_memory(z_addr));
    sram.set_word(sram.word_map.z, sram.get_word(sram.word_map.z) + 1);
    (pc + 1, cycle + 3)
}

pub fn st1(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    let d_addr = flash_memory.word(pc).operand5();
    let x_addr = sram.get_word(sram.word_map.x);
    let d = sram.get(d_addr);
    sram.set(x_addr as usize, d);
    (pc + 1, cycle + 2)
}

pub fn st2(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    let d_addr = flash_memory.word(pc).operand5();
    let x_addr = sram.get_word(sram.word_map.x);
    let d = sram.get(d_addr);
    sram.set(x_addr as usize, d);
    sram.set_word(sram.word_map.x, x_addr + 1);
    (pc + 1, cycle + 2)
}

pub fn st3(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    let d_addr = flash_memory.word(pc).operand5();
    let x_addr = sram.get_word(sram.word_map.x) - 1;
    let d = sram.get(d_addr);
    sram.set_word(sram.word_map.x, x_addr);
    sram.set(x_addr as usize, d);
    (pc + 1, cycle + 2)
}

pub fn sty1(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    let d_addr = flash_memory.word(pc).operand5();
    let y_addr = sram.get_word(sram.word_map.y);
    let d = sram.get(d_addr);
    sram.set(y_addr as usize, d);
    (pc + 1, cycle + 2)
}

pub fn sty2(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    let d_addr = flash_memory.word(pc).operand5();
    let y_addr = sram.get_word(sram.word_map.y);
    let d = sram.get(d_addr);
    sram.set(y_addr as usize, d);
    sram.set_word(sram.word_map.y, y_addr + 1);
    (pc + 1, cycle + 2)
}

pub fn sty3(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    let d_addr = flash_memory.word(pc).operand5();
    let y_addr = sram.get_word(sram.word_map.y) - 1;
    let d = sram.get(d_addr);
    sram.set_word(sram.word_map.y, y_addr);
    sram.set(y_addr as usize, d);
    (pc + 1, cycle + 2)
}

pub fn stz1(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    let d_addr = flash_memory.word(pc).operand5();
    let z_addr = sram.get_word(sram.word_map.z);
    let d = sram.get(d_addr);
    sram.set(z_addr as usize, d);
    (pc + 1, cycle + 2)
}

pub fn stz2(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    let d_addr = flash_memory.word(pc).operand5();
    let z_addr = sram.get_word(sram.word_map.z);
    let d = sram.get(d_addr);
    sram.set(z_addr as usize, d);
    sram.set_word(sram.word_map.z, z_addr + 1);
    (pc + 1, cycle + 2)
}

pub fn stz3(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    let d_addr = flash_memory.word(pc).operand5();
    let z_addr = sram.get_word(sram.word_map.z) - 1;
    let d = sram.get(d_addr);
    sram.set_word(sram.word_map.z, z_addr);
    sram.set(z_addr as usize, d);
    (pc + 1, cycle + 2)
}

pub fn cp(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    let (r_addr, d_addr) = flash_memory.word(pc).operand55();
    let (r, d) = sram.gets(r_addr, d_addr);
    let res = d.wrapping_sub(r);
    sram.set_status_by_arithmetic_instruction2(d, r, res);
    sram.set_bit(sram.bit_map.c, d < r);
    (pc + 1, cycle + 1)
}

pub fn cpi(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    let (k, d_addr) = flash_memory.word(pc).operand84();
    let d = sram.get(d_addr);
    let res = d.wrapping_sub(k);
    sram.set_status_by_arithmetic_instruction2(d, k, res);
    sram.set_bit(sram.bit_map.c, d < k);
    (pc + 1, cycle + 1)
}

pub fn cpc(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    let (r_addr, d_addr) = flash_memory.word(pc).operand55();
    let (r, d) = sram.gets(r_addr, d_addr);
    let c = sram.get_bit(sram.bit_map.c) as u8;
    let res = d.wrapping_sub(r).wrapping_sub(c);

    sram.set_bit(sram.bit_map.h, has_borrow_from_bit3_k(d, r, res));
    sram.set_bit(sram.bit_map.v, has_2complement_overflow_2(d, r, res));
    sram.set_bit(sram.bit_map.n, msb(res));
    if res != 0 {
        sram.set_bit(sram.bit_map.z, false);
    }
    sram.set_bit(sram.bit_map.c, d < r + c);
    sram.set_bit(sram.bit_map.s, sram.signed_test());
    (pc + 1, cycle + 1)
}

pub fn cpse(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    let (r_addr, d_addr) = flash_memory.word(pc).operand55();
    let (r, d) = sram.gets(r_addr, d_addr);
    if r == d {
        // The skip size is diffrenet by next instruction size.
        let next_word = flash_memory.get(pc + 1 as usize);
        let (next_instr, _) = OPCODE_TREE.with(|tree| tree.find(next_word));
        if INSTRUCTION_32_BIT.contains(&next_instr) {
            (pc + 3, cycle + 3)
        } else {
            (pc + 2, cycle + 2)
        }
    } else {
        (pc + 1, cycle + 1)
    }
}

pub fn ori(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    let (k, d_addr) = flash_memory.word(pc).operand84();
    let d = sram.get(d_addr);
    let res = d | k;
    sram.set(d_addr, res);
    sram.set_status_by_bit_instruction(res);
    (pc + 1, cycle + 1)
}

pub fn and(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    let (r_addr, d_addr) = flash_memory.word(pc).operand55();
    let (r, d) = sram.gets(r_addr, d_addr);
    let res = d & r;
    sram.set(d_addr, res);
    sram.set_status_by_bit_instruction(res);
    (pc + 1, cycle + 1)
}

pub fn andi(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    let (k, d_addr) = flash_memory.word(pc).operand84();
    let d = sram.get(d_addr);
    let res = d & k;
    sram.set(d_addr, res);
    sram.set_status_by_bit_instruction(res);
    (pc + 1, cycle + 1)
}

pub fn or(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    let (r_addr, d_addr) = flash_memory.word(pc).operand55();
    let (r, d) = sram.gets(r_addr, d_addr);
    let res = d | r;
    sram.set(d_addr, res);
    sram.set_status_by_bit_instruction(res);
    (pc + 1, cycle + 1)
}

pub fn eor(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    let (r_addr, d_addr) = flash_memory.word(pc).operand55();
    let (r, d) = sram.gets(r_addr, d_addr);
    let res = d ^ r;
    sram.set(d_addr, res);
    sram.set_status_by_bit_instruction(res);
    (pc + 1, cycle + 1)
}

pub fn breq(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    if sram.get_bit(sram.bit_map.z) {
        let k = flash_memory.word(pc).operand7();
        let pc = pc;
        let result = add_7bits_in_twos_complement_form(pc as u32, k.wrapping_add(1u8));
        (result as usize, cycle + 2)
    } else {
        (pc + 1, cycle + 1)
    }
}

pub fn brne(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    if sram.get_bit(sram.bit_map.z) {
        (pc + 1, cycle + 1)
    } else {
        let k = flash_memory.word(pc).operand7();
        let pc = pc;
        let result = add_7bits_in_twos_complement_form(pc as u32, k.wrapping_add(1u8));
        (result as usize, cycle + 2)
    }
}

pub fn brcs(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    if sram.get_bit(sram.bit_map.c) {
        let k = flash_memory.word(pc).operand7();
        let pc = pc;
        let result = add_7bits_in_twos_complement_form(pc as u32, k.wrapping_add(1u8));
        (result as usize, cycle + 2)
    } else {
        (pc + 1, cycle + 1)
    }
}

pub fn sbis(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    let (a_addr, b) = flash_memory.word(pc).operand53();
    // I/O Register starts from 0x20(0x32), so there is offset.
    let a = sram.get((a_addr + 0x20) as usize);
    if bit(a, b) {
        // TODO: ATmega328p is 16bit Program Counter machine...
        (pc + 2, cycle + 2)
    } else {
        (pc + 1, cycle + 1)
    }
}

pub fn sbiw(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    let (k, d_addr) = flash_memory.word(pc).operand62();
    let (dh, dl) = sram.gets(d_addr + 1, d_addr);
    let result = concat(dh, dl).wrapping_sub(k as u16);
    sram.set(d_addr + 1, high_byte(result));
    sram.set(d_addr, low_byte(result));

    sram.set_bit(sram.bit_map.v, msb(high_byte(result)) & !msb(dh));
    sram.set_bit(sram.bit_map.c, msb(high_byte(result)) & !msb(dh));
    sram.set_bit(sram.bit_map.n, msb(high_byte(result)));
    sram.set_bit(sram.bit_map.z, msb(high_byte(result)));
    sram.set_bit(sram.bit_map.s, sram.signed_test());
    (pc + 1, cycle + 2)
}

pub fn sei(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    sram.set_bit(sram.bit_map.i, true);
    (pc + 1, cycle + 1)
}

pub fn cli(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    sram.set_bit(sram.bit_map.i, false);
    (pc + 1, cycle + 1)
}

pub fn ret(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    let pc = sram.pop_pc_stack();
    (pc as usize, cycle + 4)
}

pub fn push(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    let d_addr = flash_memory.word(pc).operand5();
    let d = sram.get(d_addr);
    sram.push_stack(d);
    (pc + 1, cycle + 2)
}

pub fn pop(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    let d_addr = flash_memory.word(pc).operand5();
    let s = sram.pop_stack();
    sram.set(d_addr, s);
    (pc + 1, cycle + 2)
}

pub fn mov(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    let (r_addr, d_addr) = flash_memory.word(pc).operand55();
    let r = sram.get(r_addr);
    sram.set(d_addr, r);
    (pc + 1, cycle + 1)
}

pub fn movw(sram: &mut SRAM, flash_memory: &FlashMemory, pc: usize, cycle: u64) -> (usize, u64) {
    let (d_addr, r_addr) = flash_memory.word(pc).operand44();
    let (rl, rh) = sram.gets(r_addr, r_addr + 1);
    sram.set(d_addr, rl);
    sram.set(d_addr + 1, rh);
    (pc + 1, cycle + 1)
}
