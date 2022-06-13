use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen (extends = :: js_sys :: Object, js_name = Color3, js_namespace = BABYLON)]
    pub type Color3;

    #[wasm_bindgen (method, js_name = FromHexString, js_class = Color3)]
    pub fn from_hex_string(this: &Color3, color: &str) -> Color3;
}