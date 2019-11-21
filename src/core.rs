use super::memory::{ProgramMemory, DataMemory, StatusRegister, Registers, IORegisters};

#[derive(Debug)]
pub struct Core {
    pub regs: Registers,
    pub ioregs: IORegisters,
    pub sreg: StatusRegister,
    pub mem: ProgramMemory,
    pub sram: DataMemory,
    pub sp: u8,
    pub pc: u8,
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
        }
    }

    pub fn next(&mut self) {
        println!("pc: {:#04x}", self.pc);
        let opcode = self.mem.get(self.pc as u16);
        println!("mem[pc] : {:#018b}", opcode);
        // let instruction = decode(opcode);
        // instruction(&mut self);
    }
}
