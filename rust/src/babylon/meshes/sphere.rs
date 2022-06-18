use wasm_bindgen::prelude::*;
use crate::babylon::core::{Scene, Vector3};

#[wasm_bindgen]
pub struct SphereOptions {
    pub diameter: i32,
    pub segments: i32
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    pub type Sphere;

    #[wasm_bindgen(static_method_of = Sphere, js_namespace = ["BABYLON"], js_class = "MeshBuilder", js_name = "CreateSphere")]
    pub fn new(id: &str, options: SphereOptions, scene: &Scene) -> Sphere;

    #[wasm_bindgen(method, js_name="setAbsolutePosition", js_class="Sphere")]
    pub fn set_absolute_position(this: &Sphere, position: Vector3);
}

