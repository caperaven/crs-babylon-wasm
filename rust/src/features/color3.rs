#![allow(unused_imports)]
use super::*;

use wasm_bindgen::prelude::*;
use web_sys::{Element, HtmlCanvasElement, Window};

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

    #[wasm_bindgen(method, js_name = "toString")]
    pub fn to_string(this: &Color3) -> String;

    #[wasm_bindgen(js_name = Black, js_namespace = ["BABYLON", "Color3"])]
    pub fn black() -> Color3;

    #[wasm_bindgen(js_name = Red, js_namespace = ["BABYLON", "Color3"])]
    pub fn red() -> Color3;
}



// https://github.com/rustwasm/wasm-bindgen/tree/main/crates/web-sys/src
// const canvas = await crs.dom.get_element(step, context, process, item);
// const camera = await crs.process.getValue(step.args.camera)
// const color = await crs.process.getValue(step.args.color);
//
// const engine = new BABYLON.Engine(canvas);
// const scene  = new BABYLON.Scene(engine);
//
// if (color != null) {
// scene.clearColor = BABYLON.Color3.FromHexString(color);
// }
//
// canvas.__layers = [];
// canvas.__layers.push(scene);
// canvas.__engine = engine;
//
// await crs.call("gfx_camera", "initialize", { element: canvas, type: camera });
// await crs.call("gfx_materials", "initialize", { element: canvas });
//
// canvas.__renderLoop = renderLoop.bind(canvas);
// canvas.__engine.runRenderLoop(canvas.__renderLoop);
//
// canvas.__resize = resize.bind(canvas);
//
// window.addEventListener("resize", canvas.__resize);
