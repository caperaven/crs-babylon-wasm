use wasm_bindgen::prelude::*;
use web_sys::Element;
use crate::{Color3, Engine};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    pub type Scene;

    #[wasm_bindgen(constructor, js_name = Scene, js_namespace = ["BABYLON"])]
    pub fn new(canvas: &Engine) -> Scene;

    #[wasm_bindgen(method, setter = clearColor)]
    pub fn set_clear_color(this: &Scene, val: Color3);

    #[wasm_bindgen(method, getter = clearColor)]
    pub fn clear_color(this: &Scene) -> Color3;
}