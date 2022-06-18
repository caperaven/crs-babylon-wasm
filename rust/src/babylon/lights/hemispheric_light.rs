use wasm_bindgen::prelude::*;
use crate::babylon::core::{Scene, Vector3};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    pub type HemisphericLight;

    #[wasm_bindgen(constructor, js_name = HemisphericLight, js_namespace = ["BABYLON"])]
    pub fn new(name: &str, position: Vector3, scene: &Scene) -> HemisphericLight;

    #[wasm_bindgen(method, getter = intensity)]
    pub fn intensity(this: &HemisphericLight) -> f32;

    #[wasm_bindgen(method, setter = intensity)]
    pub fn set_intensity(this: &HemisphericLight, val: f32);
}