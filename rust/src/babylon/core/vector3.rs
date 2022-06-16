use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    pub type Vector3;

    #[wasm_bindgen(constructor, js_name = Vector3, js_namespace = ["BABYLON"])]
    pub fn new(x: f64, y: f64, z: f64) -> Vector3;

    #[wasm_bindgen(js_name = Zero, js_namespace = ["BABYLON", "Vector3"])]
    pub fn zero() -> Vector3;
}