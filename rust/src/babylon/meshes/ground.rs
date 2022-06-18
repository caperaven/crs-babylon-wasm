use wasm_bindgen::prelude::*;
use crate::babylon::core::{Scene, Vector3};

#[wasm_bindgen]
pub struct GroundOptions {
    pub width: i32,
    pub height: i32
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    pub type Ground;

    #[wasm_bindgen(static_method_of = Ground, js_namespace = ["BABYLON"], js_class = "MeshBuilder", js_name = "CreateGround")]
    pub fn new(id: &str, options: GroundOptions, scene: &Scene) -> Ground;

    #[wasm_bindgen(method, js_name="setAbsolutePosition", js_class="Ground")]
    pub fn set_absolute_position(this: &Ground, position: Vector3);
}

