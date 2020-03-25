use super::avrmcu::*;
use super::arch::*;
use super::util::bit::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen(start)]
pub fn initialize() {
    set_panic_hook();
}

// エラー時により詳細なスタックトレースを表示
pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub struct AvrMcu {
    avr: Box<dyn AVRMCU>
}

#[wasm_bindgen]
impl AvrMcu {
    pub fn new_atmega328p() -> AvrMcu {
        let avr = atmega328p::ATmega328P::new(atmega328p::Package::PDIP28);
        AvrMcu {
            avr: Box::new(avr),
        }
    }

    pub fn program(&self, hex: String) {
        self.avr.program(hex)
    }

    pub fn initialize(&mut self) {
        self.avr.initialize();
    }

    pub fn get_pins(&self) -> String {
        from_vec_bool_to_string(&self.avr.get_pins())
    }

    pub fn set_pins(&self, pins: String) {
        self.avr.set_pins(from_string_to_vec_bool(&pins));
    }
}
