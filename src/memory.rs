use std::fmt;
use std::collections::HashMap;

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
            write!(f, "{:#04x} --> {:016b} = {:04x} \n", index, d, d);
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
    register_map: HashMap<u8, usize>,
}

impl DataMemory {
    pub fn new() -> Self {
        let mut register_map = HashMap::new();
        register_map.insert(0x3d, 32);  // SP

        Self {
            data: vec![0; 40],
            register_map: register_map,
        }
    }

    // i は機械語のままで受け取り、内部的に並びを変える
    pub fn get(&self, i: u8) -> Option<u8> {
        // WIP
        // 特殊レジスタ
        if i >= 0x3D {
            return match self.register_map.get(&i) {
                None => None,
                Some(i) => Some(self.data[*i as usize]),
            }
        // 汎用レジスタ
        } else if i <= 31 {
            return Some(self.data[i as usize])
        };
        None
    }

    pub fn set(&mut self, i: u8, v: u8) {
        // WIP
        // 特殊レジスタ
        if i >= 0x3D {
            match self.register_map.get(&i) {
                None => panic!("canot find set space"),
                Some(i) => self.data[*i as usize] = v,
            };
        // 汎用レジスタ
        } else if i <= 31 {
            self.data[i as usize] = v;
        }
    }
}
