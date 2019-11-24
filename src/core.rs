use std::fs::File;
use std::io::{BufRead, BufReader};
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
            mem: ProgramMemory::new(100),
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

    // WIP: HexFile 型を作るべきだが、まだ理解が浅いので将来の課題とする.
    pub fn load_hex(&mut self, filepath: &str) {
        let f = File::open(filepath).expect("file not found");
        let f = BufReader::new(f);

        fn is_data_record(l: &Result<std::string::String, std::io::Error>) -> bool {
            let l: &str = l.as_ref().ok().unwrap();
            let record_type: u32 = l[7..9].parse().unwrap();
            record_type == 0
        }

        let mut i = 0;
        for line in f.lines()
            .skip_while(|l| is_data_record(l)).skip(1)
            .take_while(|l| is_data_record(l))
        {
            let line = line.expect("Unable to read line");
            let data = &line[9..line.len()-2];
            for d in data.chars().collect::<Vec<char>>().chunks(2) {
                let a = d[0].to_digit(16).unwrap();
                let b = d[1].to_digit(16).unwrap();
                self.mem.set(i, ( a << 4 | b ) as u16);
                i += 1;
            }
        }
    }
}
