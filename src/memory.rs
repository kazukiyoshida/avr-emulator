use std::fmt::LowerHex;

pub trait Memory<T>
where
    T: LowerHex,
{
    fn get(&self, a: usize) -> T;
    fn set(&mut self, a: usize, v: T);
}
