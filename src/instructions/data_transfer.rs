use super::super::core::Core;
use super::utilities::{
    operand_K8d4,
    operand_A6r5,
    operand_to_mem_index,
};

/// LDI – Load Immediate
pub fn ldi(core: &mut Core) {
    println!("ldi");
    let (k, d) = operand_K8d4(core.word());
    assert!(k <= 255);
    core.regs.data[operand_to_mem_index(d)] = k;
    core.pc += 1;
    core.cycles += 1;
}

/// OUT - Store Register to I/O Location
pub fn out(core: &mut Core) {
    let (a, r) = operand_A6r5(core.word());
    println!("out {:x} = {} <- R{}", a, a, r);
    if a == 0x3d { // 0x3d はレジスタマップ上の SP を指す
        core.sp = core.regs.data[r as usize];
    }
    core.pc += 1;
    core.cycles += 1;
}
