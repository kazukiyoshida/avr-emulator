use std::fmt;

/// Flash Program Memory
/// ・ATmega328P contains 32 KB On-chip In-System Reprogrammable Flash memory
///   for program storage.
/// ・AVR instructions are 16 or 32 bits wide, and 328P is 16bits.
/// ・Flash Program Memory is divided into two sections, Boot Loader Section and
///   Application Program Section.
#[derive(Debug)]
pub struct ProgramMemory {
    pub data: Vec<u16>,
}

impl ProgramMemory {
    pub fn new(size: usize) -> Self {
        Self { data: vec![0; size] }
    }

    pub fn get(&self, i: u16) -> u16 {
        self.data[usize::from(i)]
    }

    pub fn set(&mut self, i: u16, v: u16) {
        self.data[usize::from(i)] = v
    }
}

impl fmt::Display for ProgramMemory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut index = 0;
        for d in &self.data {
            write!(f, "{:#04x} --> {:08b} = {:02x} \n", index, d, d);
            index += 1;
        }
        Ok(())
    }
}

/// SRAM Data Memory
/// ・The first 32 locations address the Register File.
/// ・The next 64 locations address the standard I/O memory.
#[derive(Debug)]
pub struct DataMemory {
    pub data: Vec<u8>,
}

impl DataMemory {
    pub fn new(size: usize) -> Self {
        Self { data: vec![0; size] }
    }
}

/// Status Register
#[derive(Default, Debug)]
pub struct StatusRegister {
    I: bool,
    T: bool,
    H: bool,
    S: bool,
    V: bool,
    N: bool,
    Z: bool,
    C: bool,
}

/// General Purpose Register File
#[derive(Debug)]
pub struct Registers {
    pub data: Vec<u8>,
}

impl Registers {
    pub fn new() -> Self {
        Self { data: vec![0; 32] }
    }
}

/// I/O Registers
#[derive(Debug)]
pub struct IORegisters {
    pub data: Vec<u8>,
}

impl IORegisters {
    pub fn new() -> Self {
        Self { data: vec![0; 16] }
    }
}

/// Stack Pointer
/// Stack is implemented as growing from figher to lower memory locations.
/// The Stack Pointer always points to the top of the Stack.
#[derive(Default, Debug)]
pub struct StackPointer(pub u8);

/// Program Counter
#[derive(Default, Debug)]
pub struct ProgramCounter(pub u8);
