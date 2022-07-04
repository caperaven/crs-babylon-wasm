use wasm_bindgen::prelude::*;
use web_sys::Element;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    pub type Engine;

    #[wasm_bindgen(constructor, js_name = Engine, js_namespace = ["BABYLON"])]
    pub fn new(canvas: &Element) -> Engine;

    #[wasm_bindgen(method, js_name = runRenderLoop, js_class = Engine)]
    pub fn run_render_loop(this: &Engine, value: &::js_sys::Function);
}