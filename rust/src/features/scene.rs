use wasm_bindgen::prelude::*;
use crate::features::Color3;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen (extends = :: js_sys :: Object, js_name = Engine, js_namespace = BABYLON)]
    pub type Scene;

    #[wasm_bindgen (structural , method , getter , js_class = "Scene" , js_name = clearColor)]
    pub fn get_clear_color(this: &Scene) -> Color3;

    #[wasm_bindgen (structural , method , setter , js_class = "Scene" , js_name = clearColor)]
    pub fn set_clear_color(this: &Scene, value: Color3);

    #[wasm_bindgen (method, js_class = Scene, js_name = resize)]
    pub fn resize(this: &Scene);
}