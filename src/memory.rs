//! Interface with Screeps' `Memory` global variable
//!
//! Screeps' memory lives in the javascript `Memory` global variable
//! and is encoded as a javascript object.
//! Cast to wasm-bindgen types to access the values within the memory.
//! References cannot (currently) be serialized.

use wasm_bindgen::{prelude::*, JsCast};

#[wasm_bindgen(inline_js = "export function setMemory(v) { global.Memory = v; }; export function getMemory() { return global.Memory; }")]
extern "C" {
    #[wasm_bindgen(js_name = "getMemory")]
    fn get_memory_internal() -> JsValue;

    #[wasm_bindgen(js_name = "setMemory")]
    fn set_memory_internal(v: JsValue);
}

pub fn get_memory() -> JsValue {
    get_memory_internal()
}

pub fn set_memory<JsRefType: JsCast>(v: JsRefType) {
    set_memory_internal(v.into());
}
