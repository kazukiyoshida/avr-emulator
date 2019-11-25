use super::super::core::Core;
use super::utilities::{
    operand_K8d4,
    operand_A6r5,
    operand_to_mem_index,
};

/// LDI – Load Immediate
pub fn ldi(core: &mut Core) {
    let (k, d) = operand_K8d4(core.word());
    let d = d + 16; // オフセットが謎に存在する..
    println!("ldi R{} <- {}", d, k);
    assert!(k <= 255);
    core.regs()[d as usize] = k;
    core.pc += 1;
    core.cycles += 1;
}

/// OUT - Store Register to I/O Location
pub fn out(core: &mut Core) {
    let (a, r) = operand_A6r5(core.word());
    println!("out {:x} = {} <- R{}", a, a, r);
    let d = core.regs()[r as usize];
    core.sram.set(a, d);
    core.pc += 1;
    core.cycles += 1;
}
