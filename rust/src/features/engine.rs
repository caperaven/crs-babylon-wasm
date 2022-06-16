use wasm_bindgen::prelude::*;
use web_sys::Element;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    pub type Engine;

    #[wasm_bindgen(constructor, js_name = Engine, js_namespace = ["BABYLON"])]
    pub fn new(canvas: Element) -> Engine;
}