use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen (extends = :: js_sys :: Object, js_name = Engine, js_namespace = BABYLON)]
    pub type Engine;

    #[wasm_bindgen (method, js_name = resize, js_class = Engine)]
    pub fn resize(this: &Engine);
}