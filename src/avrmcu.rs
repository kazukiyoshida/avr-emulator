pub trait AVRMCU {
    fn program(&self, hex: String);
    fn initialize(&self);
    fn get_pins(&self) -> Vec<bool>;
    fn set_pins(&self, pins: Vec<bool>);
}
