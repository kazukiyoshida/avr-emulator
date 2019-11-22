use super::memory::{ProgramMemory, DataMemory, StatusRegister, Registers, IORegisters};
use super::instructions::opcode;

#[derive(Debug)]
pub struct Core {
    pub regs: Registers,
    pub ioregs: IORegisters,
    pub sreg: StatusRegister,
    pub mem: ProgramMemory,
    pub sram: DataMemory,
    pub sp: u8,
    pub pc: u8,
    pub cycles: u32,
}

impl Core {
    pub fn new() -> Self {
        Self {
            regs: Registers::new(),
            sram: DataMemory::new(8),
            mem: ProgramMemory::new(8),
            sreg: StatusRegister::default(),
            ioregs: IORegisters::new(),
            pc: 0,
            sp: 0,
            cycles: 0,
        }
    }

    pub fn next(&mut self) {
        println!("pc: {:#04x}", self.pc);
        println!(" └─> mem: {:#018b}", self.mem.get(self.pc as u16));
        match opcode::decode(self.word()) {
            Some(instruction) => instruction(self),
            _ => (),
        }
    }

    pub fn word(&self) -> u16 {
        self.mem.get(self.pc as u16)
    }
}
