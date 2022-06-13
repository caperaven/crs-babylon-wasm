#![feature(extern_types)]

mod features;
use wasm_bindgen::prelude::*;
use crate::features::{babylon};

#[wasm_bindgen]
pub fn initialize(id: &str) {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id(id).unwrap();

    let babylon = babylon().unwrap();
    let engine = babylon.engine(canvas).unwrap();
    let scene = babylon.scene(engine).unwrap();
    scene.set_clear_color(babylon.Color3().unwrap().from_hex_string("#ff0090"));
}
