#![feature(extern_types)]

mod features;
use wasm_bindgen::prelude::*;
use crate::features::{Color3, Engine};

#[wasm_bindgen]
pub fn initialize(id: &str) -> String {
    console_error_panic_hook::set_once();

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id(id).unwrap();

    let engine = Engine::new(canvas);

    let color = Color3::red();
    color.to_string()
}
