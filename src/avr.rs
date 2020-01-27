pub trait AVR {
    // Program Counter
    fn pc(&self) -> u16;
    fn set_pc(&mut self, v: u16);

    // Stack Pointer
    fn sp(&self) -> u16;

    // General Purpose Register
    fn gprg(&self, addr: usize) -> u8;
    fn set_gprg(&mut self, addr: usize, v: u8);
}

pub enum Sreg { I, T, H, S, V, N, Z, C }

// 8bit, 16bit のメモリ
pub trait Memory<T> {
    fn get(&self, a: usize) -> T;
    fn set(&mut self, a: usize, v: T);
}

