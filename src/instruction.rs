use super::avr::*;
use super::utils::*;
use super::word::*;

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

pub type InstrFunc = &'static dyn Fn(&mut dyn AVR);

pub fn add(avr: &mut dyn AVR) {
    let (r_addr, d_addr) = avr.word().operand55();
    let (r, d) = avr.get_registers(r_addr, d_addr);
    let res = r.wrapping_add(d);
    avr.set_register(d_addr, res);
    avr.set_status_by_arithmetic_instruction(d, r, res);
    avr.set_bit(avr.b().c, has_borrow_from_msb(r, d, res));
    avr.pc_increment(1);
    avr.cycle_increment(1);
}

pub fn adc(avr: &mut dyn AVR) {
    let (r_addr, d_addr) = avr.word().operand55();
    let (r, d) = avr.get_registers(r_addr, d_addr);
    let c = avr.get_bit(avr.b().c) as u8;
    let res = r.wrapping_add(d).wrapping_add(c);
    avr.set_register(d_addr, res);
    avr.set_status_by_arithmetic_instruction(d, r, res);
    avr.set_bit(avr.b().c, has_borrow_from_msb(r, d, res));
    avr.pc_increment(1);
    avr.cycle_increment(1);
}

pub fn adiw(avr: &mut dyn AVR) {
    let (k, d_addr) = avr.word().operand62();
    let (dh, dl) = avr.get_registers(d_addr + 1, d_addr);
    let res = concat(dh, dl).wrapping_add(k as u16);
    avr.set_register(d_addr, high_byte(res));
    avr.set_register(d_addr + 1, low_byte(res));

    avr.set_bit(avr.b().v, !msb(dh) & msb(high_byte(res)));
    avr.set_bit(avr.b().n, msb(high_byte(res)));
    avr.set_bit(avr.b().z, res == 0);
    avr.set_bit(avr.b().c, !msb(high_byte(res)) & msb(dh));

    avr.signed_test();

    avr.pc_increment(1);
    avr.cycle_increment(1);
}

pub fn sbci(avr: &mut dyn AVR) {
    let (k, d_addr) = avr.word().operand84();
    let d = avr.get_register(d_addr);
    let c = avr.get_bit(avr.b().c) as u8;
    let res = d.wrapping_sub(k).wrapping_sub(c);
    avr.set_register(d_addr, res);

    avr.set_bit(avr.b().h, has_borrow_from_bit3(d, k, res));
    avr.set_bit(avr.b().v, has_2complement_overflow(d, k, res));
    avr.set_bit(avr.b().n, msb(res));
    if res != 0 {
        avr.set_bit(avr.b().z, false);
    };
    match d.checked_sub(k).and_then(|d_k| d_k.checked_sub(c)) {
        None => avr.set_bit(avr.b().c, true),
        _ => avr.set_bit(avr.b().c, false),
    };
    avr.signed_test();

    avr.pc_increment(1);
    avr.cycle_increment(1);
}

pub fn dec(avr: &mut dyn AVR) {
    let d_addr = avr.word().operand5();
    let d = avr.get_register(d_addr);
    let result = d.wrapping_sub(1);
    avr.set_register(d_addr, result);

    avr.set_bit(avr.b().v, d == 0x80u8);
    avr.set_bit(avr.b().n, msb(result));
    avr.set_bit(avr.b().z, result == 0);
    avr.signed_test();

    avr.pc_increment(1);
    avr.cycle_increment(1);
}

pub fn com(avr: &mut dyn AVR) {
    let d_addr = avr.word().operand5();
    let d = avr.get_register(d_addr);
    let res = 0xff - d;
    avr.set_register(d_addr, res);
    avr.set_status_by_bit_instruction(res);
    avr.set_bit(avr.b().c, false);
    avr.pc_increment(1);
    avr.cycle_increment(1);
}

pub fn sub(avr: &mut dyn AVR) {
    let (r_addr, d_addr) = avr.word().operand55();
    let (r, d) = avr.get_registers(r_addr, d_addr);
    let res = d.wrapping_sub(r);
    avr.set_register(d_addr, res);
    avr.set_status_by_arithmetic_instruction(d, r, res);
    avr.set_bit(avr.b().c, d < r);
    avr.pc_increment(1);
    avr.cycle_increment(1);
}

pub fn sbc(avr: &mut dyn AVR) {
    let (r_addr, d_addr) = avr.word().operand55();
    let (r, d) = avr.get_registers(r_addr, d_addr);
    let c = avr.get_bit(avr.b().c) as u8;
    let res = d.wrapping_add(r).wrapping_add(c);
    avr.set_register(d_addr, res);
    avr.set_status_by_arithmetic_instruction(d, r, res);
    avr.set_bit(avr.b().c, d < (r.wrapping_add(1)));
    avr.pc_increment(1);
    avr.cycle_increment(1);
}

pub fn subi(avr: &mut dyn AVR) {
    let (k, d_addr) = avr.word().operand84();
    let d = avr.get_register(d_addr);
    let res = d.wrapping_sub(k);
    avr.set_register(d_addr, res);
    avr.set_status_by_arithmetic_instruction(d, k, res);
    avr.set_bit(avr.b().c, d < k);
    avr.pc_increment(1);
    avr.cycle_increment(1);
}

pub fn ld1(avr: &mut dyn AVR) {
    let d_addr = avr.word().operand5();
    let x_addr = avr.get_word(avr.w().x);
    avr.set_register(d_addr, avr.get_register(x_addr as usize));
    avr.pc_increment(1);
    avr.cycle_increment(1);
}

pub fn ld2(avr: &mut dyn AVR) {
    let d_addr = avr.word().operand5();
    let x_addr = avr.get_word(avr.w().x);
    let x = avr.get_register(x_addr as usize);
    avr.set_register(d_addr, x);
    avr.set_word(avr.w().x, x_addr + 1);
    avr.pc_increment(1);
    avr.cycle_increment(2);
}

pub fn ld3(avr: &mut dyn AVR) {
    let d_addr = avr.word().operand5();
    let x_addr = avr.get_word(avr.w().x) - 1;
    avr.set_word(avr.w().x, x_addr);
    let x = avr.get_register(x_addr as usize);
    avr.set_register(d_addr, x);
    avr.pc_increment(1);
    avr.cycle_increment(3);
}

pub fn ldi(avr: &mut dyn AVR) {
    let (k, d_addr) = avr.word().operand84();
    avr.set_register(d_addr, k);
    avr.pc_increment(1);
    avr.cycle_increment(1);
}

pub fn lds(avr: &mut dyn AVR) {
    let (w, k_addr) = avr.double_word();
    let d_addr = w.operand5();
    let k = avr.get_register(k_addr.0 as usize);
    avr.set_register(d_addr, k);
    avr.pc_increment(2);
    avr.cycle_increment(2);
}

pub fn lddy1(avr: &mut dyn AVR) {
    let d_addr = avr.word().operand5();
    let y_addr = avr.get_word(avr.w().y);
    avr.set_register(d_addr, avr.get_register(y_addr as usize));
    avr.pc_increment(1);
    avr.cycle_increment(2); // 1 cycles in Manual
}

pub fn lddy2(avr: &mut dyn AVR) {
    let d_addr = avr.word().operand5();
    let y_addr = avr.get_word(avr.w().y);
    avr.set_register(d_addr, avr.get_register(y_addr as usize));
    avr.set_word(avr.w().y, y_addr + 1);
    avr.pc_increment(1);
    avr.cycle_increment(2);
}

pub fn lddy3(avr: &mut dyn AVR) {
    let d_addr = avr.word().operand5();
    let y_addr = avr.get_word(avr.w().y) - 1;
    avr.set_word(avr.w().y, y_addr);
    avr.set_register(d_addr, avr.get_register(y_addr as usize));
    avr.pc_increment(1);
    avr.cycle_increment(2);
}

// ASS: "avr_studio/led_flashing.hex"
//      cycle 152: 0x04e -> 0x01  // TCNT0 のカウントアップ!
//      cycle 152: 0x084 -> 0x02  // TCNT1L のカウントアップ!
pub fn lddz1(avr: &mut dyn AVR) {
    let d_addr = avr.word().operand5();
    let z_addr = avr.get_word(avr.w().z);
    avr.set_register(d_addr, avr.get_register(z_addr as usize));
    avr.pc_increment(1);
    avr.cycle_increment(2); // 1 cycles in Manual
}

pub fn lddz2(avr: &mut dyn AVR) {
    let d_addr = avr.word().operand5();
    let z_addr = avr.get_word(avr.w().z);
    avr.set_register(d_addr, avr.get_register(z_addr as usize));
    avr.set_word(avr.w().z, z_addr + 1);
    avr.pc_increment(1);
    avr.cycle_increment(2);
}

pub fn lddz3(avr: &mut dyn AVR) {
    let d_addr = avr.word().operand5();
    let z_addr = avr.get_word(avr.w().z) - 1;
    avr.set_word(avr.w().z, z_addr);
    avr.set_register(d_addr, avr.get_register(z_addr as usize));
    avr.pc_increment(1);
    avr.cycle_increment(2);
}

// ASS: "avr_studio/led_flashing.hex"
//      cycle 307: 0x023 -> 0x20  // Slightly cretical
//      cycle 360: OUT set I flg
pub fn out(avr: &mut dyn AVR) {
    let (a_addr, r_addr) = avr.word().operand65();
    let r = avr.get_register(r_addr);
    avr.set_register(a_addr, r);
    avr.pc_increment(1);
    avr.cycle_increment(1);
}

// ASS: "avr_studio/led_flashing.hex"
//      cycle 225: SREG = ( 0x5f ) = 0xb5 ~> 0x35
pub fn in_instr(avr: &mut dyn AVR) {
    let (a_addr, d_addr) = avr.word().operand65();
    let a = avr.get_register(a_addr);
    if a_addr == 0x5f {
        // SREG
        avr.set_register(d_addr, a & 0b111_1111);
    } else {
        avr.set_register(d_addr, a);
    };
    avr.pc_increment(1);
    avr.cycle_increment(1);
}

pub fn nop(avr: &mut dyn AVR) {
    avr.pc_increment(1);
    avr.cycle_increment(1);
}

pub fn call(avr: &mut dyn AVR) {
    // Push current pc to stack
    avr.push_pc_stack(avr.pc() + 2);

    // Update pc by immediate
    let (w1, w2) = avr.double_word();
    avr.set_pc(w1.operand22(w2));

    avr.cycle_increment(4);
}

pub fn rol(avr: &mut dyn AVR) {
    let d_addr = avr.word().operand10() as usize;
    let d_old = avr.get_register(d_addr);
    let c = avr.get_bit(avr.b().c) as u8;
    let d_new = (d_old << 1) | c;
    avr.set_register(d_addr, d_new);

    avr.set_bit(avr.b().h, (d_old & 0b0000_1000) >> 3 == 1);
    avr.set_bit(avr.b().n, msb(d_new));
    avr.set_bit(avr.b().z, d_new == 0);
    avr.set_bit(avr.b().c, msb(d_old));
    avr.set_bit(avr.b().v, avr.get_bit(avr.b().n) ^ avr.get_bit(avr.b().c));
    avr.signed_test();

    avr.pc_increment(1);
    avr.cycle_increment(3);
}

pub fn lsl(avr: &mut dyn AVR) {
    let d_addr = avr.word().operand10() as usize;
    let d_old = avr.get_register(d_addr);
    let d_new = d_old << 1;
    avr.set_register(d_addr, d_new);

    avr.set_bit(avr.b().h, (d_old & 0b0000_1000) >> 3 == 1);
    avr.set_bit(avr.b().n, msb(d_new));
    avr.set_bit(avr.b().z, d_new == 0);
    avr.set_bit(avr.b().c, msb(d_old));
    avr.set_bit(avr.b().v, avr.get_bit(avr.b().n) ^ avr.get_bit(avr.b().c));
    avr.signed_test();

    avr.pc_increment(1);
    avr.cycle_increment(3);
}

pub fn rcall(avr: &mut dyn AVR) {
    let k = avr.word().operand12() as u32;
    let pc = avr.pc();
    avr.set_pc(pc + k + 1);
    avr.cycle_increment(3);
}

pub fn jmp(avr: &mut dyn AVR) {
    let (w1, w2) = avr.double_word();
    let k = w1.operand22(w2);
    avr.set_pc(k);
    avr.cycle_increment(2); // 3 cycles in Manual
}

pub fn rjmp(avr: &mut dyn AVR) {
    let k = avr.word().operand12();
    let pc = avr.pc();
    let result = add_12bits_in_twos_complement_form(pc, k) + 1u32;
    avr.set_pc(result);
    avr.cycle_increment(2);
}

pub fn sts(avr: &mut dyn AVR) {
    let (w1, k) = avr.double_word();
    let d_addr = w1.operand5();
    let d = avr.get_register(d_addr);
    avr.set_register(k.0 as usize, d);
    avr.pc_increment(2);
    avr.cycle_increment(2);
}

pub fn lpm1(avr: &mut dyn AVR) {
    avr.set_register(0, avr.z_program_memory());
    avr.pc_increment(1);
    avr.cycle_increment(3);
}

// ASS: "avr_studio/led_flashing.hex"
//      cycle 218: 0x46 -> 0x02  // TCNT0 のカウントアップ!
//      cycle 218: 0x84 -> 0x03  // TCNT1L のカウントアップ!
pub fn lpm2(avr: &mut dyn AVR) {
    let d_addr = avr.word().operand5();
    avr.set_register(d_addr, avr.z_program_memory());
    avr.pc_increment(1);
    avr.cycle_increment(3);
}

pub fn lpm3(avr: &mut dyn AVR) {
    let d_addr = avr.word().operand5();
    avr.set_register(d_addr, avr.z_program_memory());
    avr.set_word(avr.w().z, avr.get_word(avr.w().z) + 1);
    avr.pc_increment(1);
    avr.cycle_increment(3);
}

pub fn st1(avr: &mut dyn AVR) {
    let d_addr = avr.word().operand5();
    let x_addr = avr.get_word(avr.w().x);
    let d = avr.get_register(d_addr);
    avr.set_register(x_addr as usize, d);
    avr.pc_increment(1);
    avr.cycle_increment(2);
}

pub fn st2(avr: &mut dyn AVR) {
    let d_addr = avr.word().operand5();
    let x_addr = avr.get_word(avr.w().x);
    let d = avr.get_register(d_addr);
    avr.set_register(x_addr as usize, d);
    avr.set_word(avr.w().x, x_addr + 1);
    avr.pc_increment(1);
    avr.cycle_increment(2);
}

pub fn st3(avr: &mut dyn AVR) {
    let d_addr = avr.word().operand5();
    let x_addr = avr.get_word(avr.w().x) - 1;
    let d = avr.get_register(d_addr);
    avr.set_word(avr.w().x, x_addr);
    avr.set_register(x_addr as usize, d);
    avr.pc_increment(1);
    avr.cycle_increment(2);
}

pub fn sty1(avr: &mut dyn AVR) {
    let d_addr = avr.word().operand5();
    let y_addr = avr.get_word(avr.w().y);
    let d = avr.get_register(d_addr);
    avr.set_register(y_addr as usize, d);
    avr.pc_increment(1);
    avr.cycle_increment(2);
}

pub fn sty2(avr: &mut dyn AVR) {
    let d_addr = avr.word().operand5();
    let y_addr = avr.get_word(avr.w().y);
    let d = avr.get_register(d_addr);
    avr.set_register(y_addr as usize, d);
    avr.set_word(avr.w().y, y_addr + 1);
    avr.pc_increment(1);
    avr.cycle_increment(2);
}

pub fn sty3(avr: &mut dyn AVR) {
    let d_addr = avr.word().operand5();
    let y_addr = avr.get_word(avr.w().y) - 1;
    let d = avr.get_register(d_addr);
    avr.set_word(avr.w().y, y_addr);
    avr.set_register(y_addr as usize, d);
    avr.pc_increment(1);
    avr.cycle_increment(2);
}

// ASS: "avr_studio/led_flashing.hex"
//      cycle 112: 0x84 -> 0x01  // TCNT1L のカウントアップ!
pub fn stz1(avr: &mut dyn AVR) {
    let d_addr = avr.word().operand5();
    let z_addr = avr.get_word(avr.w().z);
    let d = avr.get_register(d_addr);
    avr.set_register(z_addr as usize, d);
    avr.pc_increment(1);
    avr.cycle_increment(2);
}

pub fn stz2(avr: &mut dyn AVR) {
    let d_addr = avr.word().operand5();
    let z_addr = avr.get_word(avr.w().z);
    let d = avr.get_register(d_addr);
    avr.set_register(z_addr as usize, d);
    avr.set_word(avr.w().z, z_addr + 1);
    avr.pc_increment(1);
    avr.cycle_increment(2);
}

pub fn stz3(avr: &mut dyn AVR) {
    let d_addr = avr.word().operand5();
    let z_addr = avr.get_word(avr.w().z) - 1;
    let d = avr.get_register(d_addr);
    avr.set_word(avr.w().z, z_addr);
    avr.set_register(z_addr as usize, d);
    avr.pc_increment(1);
    avr.cycle_increment(2);
}

pub fn cp(avr: &mut dyn AVR) {
    let (r_addr, d_addr) = avr.word().operand55();
    let (r, d) = avr.get_registers(r_addr, d_addr);
    let res = d.wrapping_sub(r);
    avr.set_status_by_arithmetic_instruction(d, r, res);
    avr.set_bit(avr.b().c, d < r);
    avr.pc_increment(1);
    avr.cycle_increment(1);
}

pub fn cpi(avr: &mut dyn AVR) {
    let (k, d_addr) = avr.word().operand84();
    let d = avr.get_register(d_addr);
    let res = d.wrapping_sub(k);
    avr.set_status_by_arithmetic_instruction(d, k, res);
    avr.set_bit(avr.b().c, d < k);
    avr.pc_increment(1);
    avr.cycle_increment(1);
}

pub fn cpc(avr: &mut dyn AVR) {
    let (r_addr, d_addr) = avr.word().operand55();
    let (r, d) = avr.get_registers(r_addr, d_addr);
    let c = avr.get_bit(avr.b().c) as u8;
    let res = d.wrapping_sub(r).wrapping_sub(c);

    avr.set_bit(avr.b().h, has_borrow_from_bit3(d, r, res));
    avr.set_bit(avr.b().v, has_2complement_overflow(d, r, res));
    avr.set_bit(avr.b().n, msb(res));
    if res != 0 {
        avr.set_bit(avr.b().z, false);
    }
    avr.set_bit(avr.b().c, d < r + c);
    avr.signed_test();
    avr.pc_increment(1);
    avr.cycle_increment(1);
}

pub fn cpse(avr: &mut dyn AVR) {
    let (r_addr, d_addr) = avr.word().operand55();
    let (r, d) = avr.get_registers(r_addr, d_addr);
    if r == d {
        // The skip size is diffrenet by next instruction size.
        let next_opcode = Word(avr.fetch(avr.pc() + 1));
        let (instr, _) = avr.decode_instr(next_opcode);
        if INSTRUCTION_32_BIT.contains(&instr) {
            avr.set_pc(avr.pc() + 3);
            avr.cycle_increment(3);
        } else {
            avr.set_pc(avr.pc() + 2);
            avr.cycle_increment(2);
        };
    } else {
        avr.pc_increment(1);
        avr.cycle_increment(1);
    }
}

pub fn ori(avr: &mut dyn AVR) {
    let (k, d_addr) = avr.word().operand84();
    let d = avr.get_register(d_addr);
    let res = d | k;
    avr.set_register(d_addr, res);
    avr.set_status_by_bit_instruction(res);
    avr.pc_increment(1);
    avr.cycle_increment(1);
}

pub fn and(avr: &mut dyn AVR) {
    let (r_addr, d_addr) = avr.word().operand55();
    let (r, d) = avr.get_registers(r_addr, d_addr);
    let res = d & r;
    avr.set_register(d_addr, res);
    avr.set_status_by_bit_instruction(res);
    avr.pc_increment(1);
    avr.cycle_increment(1);
}

pub fn andi(avr: &mut dyn AVR) {
    let (k, d_addr) = avr.word().operand84();
    let d = avr.get_register(d_addr);
    let res = d & k;
    avr.set_register(d_addr, res);
    avr.set_status_by_bit_instruction(res);
    avr.pc_increment(1);
    avr.cycle_increment(1);
}

pub fn or(avr: &mut dyn AVR) {
    let (r_addr, d_addr) = avr.word().operand55();
    let (r, d) = avr.get_registers(r_addr, d_addr);
    let res = d | r;
    avr.set_register(d_addr, res);
    avr.set_status_by_bit_instruction(res);
    avr.pc_increment(1);
    avr.cycle_increment(1);
}

pub fn eor(avr: &mut dyn AVR) {
    let (r_addr, d_addr) = avr.word().operand55();
    let (r, d) = avr.get_registers(r_addr, d_addr);
    let res = d ^ r;
    avr.set_register(d_addr, res);
    avr.set_status_by_bit_instruction(res);
    avr.pc_increment(1);
    avr.cycle_increment(1);
}

// ASS: "avr_studio/led_flashing.hex"
//      cycle 280: 0x46 -> 0x03  // TCNT0 のカウントアップ!
//      cycle 280: 0x84 -> 0x04  // TCNT1L のカウントアップ!
pub fn breq(avr: &mut dyn AVR) {
    if avr.get_bit(avr.b().z) {
        let k = avr.word().operand7();
        let pc = avr.pc();
        let result = add_7bits_in_twos_complement_form(pc, k.wrapping_add(1u8));
        avr.set_pc(result);
        avr.cycle_increment(2);
    } else {
        avr.pc_increment(1);
        avr.cycle_increment(1);
    }
}

pub fn brne(avr: &mut dyn AVR) {
    if avr.get_bit(avr.b().z) {
        avr.pc_increment(1);
        avr.cycle_increment(1);
    } else {
        let k = avr.word().operand7();
        let pc = avr.pc();
        let result = add_7bits_in_twos_complement_form(pc, k.wrapping_add(1u8));
        avr.set_pc(result as u32);
        avr.cycle_increment(2);
    }
}

pub fn brcs(avr: &mut dyn AVR) {
    if avr.get_bit(avr.b().c) {
        let k = avr.word().operand7();
        let pc = avr.pc();
        let result = add_7bits_in_twos_complement_form(pc, k.wrapping_add(1u8));
        avr.set_pc(result as u32);
        avr.cycle_increment(2);
    } else {
        avr.pc_increment(1);
        avr.cycle_increment(1);
    }
}

pub fn sbis(avr: &mut dyn AVR) {
    let (a_addr, b) = avr.word().operand53();
    // I/O Register starts from 0x20(0x32), so there is offset.
    let a = avr.get_register((a_addr + 0x20) as usize);
    if bit(a, b) {
        // WIP: ATmega328p is 16bit Program Counter machine...
        avr.set_pc(avr.pc() + 2);
        avr.cycle_increment(2);
    } else {
        avr.pc_increment(1);
        avr.cycle_increment(1);
    }
}

pub fn sbiw(avr: &mut dyn AVR) {
    let (k, d_addr) = avr.word().operand62();
    let (dh, dl) = avr.get_registers(d_addr + 1, d_addr);
    let result = concat(dh, dl).wrapping_sub(k as u16);
    avr.set_register(d_addr + 1, high_byte(result));
    avr.set_register(d_addr, low_byte(result));

    avr.set_bit(avr.b().v, msb(high_byte(result)) & !msb(dh));
    avr.set_bit(avr.b().c, msb(high_byte(result)) & !msb(dh));
    avr.set_bit(avr.b().n, msb(high_byte(result)));
    avr.set_bit(avr.b().z, msb(high_byte(result)));
    avr.signed_test();
    avr.pc_increment(1);
    avr.cycle_increment(2);
}

pub fn sei(avr: &mut dyn AVR) {
    avr.set_bit(avr.b().i, true);
    avr.pc_increment(1);
    avr.cycle_increment(1);
}

pub fn cli(avr: &mut dyn AVR) {
    avr.set_bit(avr.b().i, false);
    avr.pc_increment(1);
    avr.cycle_increment(1);
}

pub fn ret(avr: &mut dyn AVR) {
    let pc = avr.pop_pc_stack();
    avr.set_pc(pc as u32);
    avr.cycle_increment(4);
}

pub fn push(avr: &mut dyn AVR) {
    let d_addr = avr.word().operand5();
    let d = avr.get_register(d_addr);
    avr.push_stack(d);
    avr.pc_increment(1);
    avr.cycle_increment(2);
}

pub fn pop(avr: &mut dyn AVR) {
    let d_addr = avr.word().operand5();
    let s = avr.pop_stack();
    avr.set_register(d_addr, s);
    avr.pc_increment(1);
    avr.cycle_increment(2);
}

pub fn mov(avr: &mut dyn AVR) {
    let (r_addr, d_addr) = avr.word().operand55();
    let r = avr.get_register(r_addr);
    avr.set_register(d_addr, r);
    avr.pc_increment(1);
    avr.cycle_increment(1);
}

// ASS: "avr_studio/led_flashing.hex"
//      cycle 190: 0x37 -> 0x01 // TIFR: Timer Interrupt Flg の設定
pub fn movw(avr: &mut dyn AVR) {
    let (d_addr, r_addr) = avr.word().operand44();
    let (rl, rh) = avr.get_registers(r_addr, r_addr + 1);
    avr.set_register(d_addr, rl);
    avr.set_register(d_addr + 1, rh);
    avr.pc_increment(1);
    avr.cycle_increment(1);
}
