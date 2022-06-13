#![allow(unused_imports)]
use super::*;

mod babylonjs;
mod engine;
mod scene;
mod color3;

pub use babylonjs::Babylon;
pub use engine::Engine;
pub use scene::Scene;
pub use color3::Color3;

pub fn babylon() -> Option<Babylon> {
    use wasm_bindgen::JsCast;

    js_sys::global().dyn_into::<Babylon>().ok()
}