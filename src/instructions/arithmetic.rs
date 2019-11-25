use super::super::core::Core;
use super::utilities::{operand_r5d5, operand_d5};

/// ADC - Add with Carry
pub fn adc(core: &mut Core) {
    println!("adc");
    // let (r, d) = operand_r5d5(core.word());
    // assert!(r <= 31 && d <= 31);
    // core.regs.data[r as usize] =
    //     core.regs.data[r as usize] + core.regs.data[d as usize];
    core.pc += 1;
    core.cycles += 1;
}

/// ADD - Add without Carry
pub fn add(core: &mut Core) {
    println!("add");
    // let (r, d) = operand_r5d5(core.word());
    // assert!(r <= 31 && d <= 31);
    // core.regs.data[r as usize] =
    //     core.regs.data[r as usize] + core.regs.data[d as usize];
    core.pc += 1;
    core.cycles += 1;
}

pub fn adiw(core: &mut Core) {
    println!("adiw");
}

pub fn andi(core: &mut Core) {
    println!("andi");
}

pub fn and(core: &mut Core) {
    println!("and");
}

/// DEC - Decrement
pub fn dec(core: &mut Core) {
    let d = operand_d5(core.word());
    println!("dec R{}", d);
    match core.sram.get(d as u8) {
        None => panic!("cannnot get register"),
        Some(rd) => core.sram.set(d as u8, rd - 1),
    }
    core.pc += 1;
    core.cycles += 1;
}
