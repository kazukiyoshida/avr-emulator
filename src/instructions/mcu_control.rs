use super::super::core::Core;

pub fn nop(core: &mut Core) {
    println!("nop");
    core.pc += 1;
    core.cycles += 1;
}
