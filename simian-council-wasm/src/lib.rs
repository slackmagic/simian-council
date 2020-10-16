mod utils;

use simian_council;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, simian-council-wasm!");
}

#[wasm_bindgen]
pub fn ask_for_a_pokemon() -> String {
    ask_for_a_pokemon()
}

#[wasm_bindgen]
pub fn ask_for_a_secret(num: u8, start_enthro_level: u8, end_enthro_level: u8) -> String {
    ask_for_a_secret(num, start_enthro_level, end_enthro_level)
}
