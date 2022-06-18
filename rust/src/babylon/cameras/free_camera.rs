use wasm_bindgen::prelude::*;
use web_sys::Element;
use crate::babylon::Vector3;
use crate::Scene;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    pub type FreeCamera;

    #[wasm_bindgen(constructor, js_name = FreeCamera, js_namespace = ["BABYLON"])]
    pub fn new(name: &str, position: Vector3, scene: &Scene) -> FreeCamera;

    #[wasm_bindgen(method, js_name = setTarget, js_class = FreeCamera)]
    pub fn set_target(this: &FreeCamera, target: Vector3);

    #[wasm_bindgen(method, js_name = attachControl, js_class = FreeCamera)]
    pub fn attach_control(this: &FreeCamera, canvas: &Element, prevent_default: bool);
}