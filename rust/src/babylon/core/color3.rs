use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    pub type Color3;

    #[wasm_bindgen(method, getter = r)]
    pub fn r(this: &Color3) -> i32;

    #[wasm_bindgen(method, setter = r)]
    pub fn set_r(this: &Color3, val: i32);

    #[wasm_bindgen(method, getter = g)]
    pub fn g(this: &Color3) -> i32;

    #[wasm_bindgen(method, setter = g)]
    pub fn set_g(this: &Color3, val: i32);

    #[wasm_bindgen(method, getter = b)]
    pub fn b(this: &Color3) -> i32;

    #[wasm_bindgen(method, setter = b)]
    pub fn set_b(this: &Color3, val: i32);

    #[wasm_bindgen(method, js_name = toString)]
    pub fn to_string(this: &Color3) -> String;

    #[wasm_bindgen(js_name = Black, js_namespace = ["BABYLON", "Color3"])]
    pub fn black() -> Color3;

    #[wasm_bindgen(js_name = Red, js_namespace = ["BABYLON", "Color3"])]
    pub fn red() -> Color3;

    #[wasm_bindgen(js_name = FromHexString, js_namespace = ["BABYLON", "Color3"])]
    pub fn from_hex_string(color: &str) -> Color3;
}