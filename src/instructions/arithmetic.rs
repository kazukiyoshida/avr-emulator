use super::super::core::Core;
use super::utilities::{operand_r5d5};

/// ADC - Add with Carry
pub fn adc(core: &mut Core) {
    println!("adc");
    let (r, d) = operand_r5d5(core.word());
    assert!(r <= 31 && d <= 31);
    core.regs.data[r as usize] =
        core.regs.data[r as usize] + core.regs.data[d as usize];
    rr = rr + rd;
    core.pc += 1;
    core.cycles += 1;
}

/// ADD - Add without Carry
pub fn add(core: &mut Core) {
    println!("add");
    let (r, d) = operand_r5d5(core.word());
    assert!(r <= 31 && d <= 31);
    core.regs.data[r as usize] =
        core.regs.data[r as usize] + core.regs.data[d as usize];
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

// pub fn add(core: &mut Core) {
//     println!("add");
//
//     let (r, d) = operand_r5d5(core.word());
//     println!("{:016b}", core.word());
//     println!("r {:b} = {}", r, r);
//     println!("d {:b} = {}", d, d);
//
//     assert!(0 <= r && r <= 31);
//     assert!(0 <= d && d <= 31);
//
//     let mut rr = core.regs.data[r as usize];
//     let mut rd = core.regs.data[d as usize];
//
//     println!("----- before ");
//     println!("R{:02} : {:08b}", r, rr);
//     println!("R{:02} : {:08b}", d, rd);
//     println!("PC     : {}", core.pc);
//
//     rr = rr + rd;
//     core.pc += 1;
//     core.cycles += 1;
//
//     println!("----- after ");
//     println!("R{:02} : {:08b}", r, rr);
//     println!("R{:02} : {:08b}", d, rd);
//     println!("PC     : {}", core.pc);
//
// }

