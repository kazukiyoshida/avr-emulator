use super::super::core::Core;
use super::utilities::{operand_k12, operand_k7};

pub fn rcall(core: &mut Core) {
    let k = operand_k12(core.word());
    println!("rcall {} = {:b}", k, k);
    core.pc += k + 1;
    core.cycles += 1;
}

/// BRNE – Branch if Not Equal
pub fn brne(core: &mut Core) {
    // Two's complement form
    let k = operand_k7(core.word());
    println!("brne {}", k);
    core.pc = (core.pc as i16).wrapping_add(k as i16) as u16;
    core.pc += 1;
    core.cycles += 1;
}
