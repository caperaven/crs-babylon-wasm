#![allow(unused_imports)]
use super::*;

mod color3;
mod engine;

pub use color3::Color3;
pub use engine::Engine;


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
