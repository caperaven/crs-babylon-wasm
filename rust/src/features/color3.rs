// #![allow(unused_imports)]
// use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    pub type Color3;

    #[wasm_bindgen(method, getter)]
    pub fn r(this: &Color3) -> i32;

    #[wasm_bindgen(method, setter)]
    pub fn set_r(this: &Color3, val: i32);

    #[wasm_bindgen(method, getter)]
    pub fn g(this: &Color3) -> i32;

    #[wasm_bindgen(method, setter)]
    pub fn set_g(this: &Color3, val: i32);

    #[wasm_bindgen(method, getter)]
    pub fn b(this: &Color3) -> i32;

    #[wasm_bindgen(method, setter)]
    pub fn set_b(this: &Color3, val: i32);

    #[wasm_bindgen(method, js_name = toString)]
    pub fn to_string(this: &Color3) -> String;

    #[wasm_bindgen(js_name = Black, js_namespace = ["BABYLON", "Color3"])]
    pub fn black() -> Color3;

    #[wasm_bindgen(js_name = Red, js_namespace = ["BABYLON", "Color3"])]
    pub fn red() -> Color3;
}