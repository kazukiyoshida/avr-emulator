use super::super::core::Core;
use super::utilities::{operand_r5d5};

pub fn rcall(core: &mut Core) {
    println!("rcall");
    core.pc += 1;
    core.cycles += 1;
}

pub fn brne(core: &mut Core) {
    println!("brne");
    core.pc += 1;
    core.cycles += 1;
}
