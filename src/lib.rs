mod utils;

use seahash;
use skyrim_cell_dump;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn parse_plugin(input: &[u8]) -> JsValue {
    JsValue::from_serde(&skyrim_cell_dump::parse_plugin(input).unwrap()).unwrap()
}

#[wasm_bindgen]
pub fn hash_plugin(input: &[u8]) -> JsValue {
    JsValue::from(seahash::hash(&input))
}
