use wasm_bindgen::prelude::*;
use web_sys::Element;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    pub type Engine;

    #[wasm_bindgen(constructor, js_name = Engine, js_namespace = ["BABYLON"])]
    pub fn new(canvas: &Element) -> Engine;

    #[wasm_bindgen(method, js_name = runRenderLoop, js_class = Engine)]
    pub fn run_render_loop(this: &Engine, value: &Closure<dyn FnMut()>);
}


// #[wasm_bindgen]
// extern {
//     fn setTimeout(closure: &Closure<FnMut()>, time: u32);
//
//     #[wasm_bindgen(js_namespace = console)]
//     fn log(s: &str);
// }
//
// #[wasm_bindgen]
// pub struct ClosureHandle(Closure<FnMut()>);
//
// #[wasm_bindgen]
// pub fn test() -> ClosureHandle {
//     // First up we use `Closure::wrap` to wrap up a Rust closure and create a JS closure.
//     let cb = Closure::wrap(Box::new(move || {
//         log("timeout elapsed!");
//     }) as Box<FnMut()>);
//
//     // Next we pass this via reference to the `setTimeout` function, and
//     // `setTimeout` gets a handle to the corresponding JS closure.
//     setTimeout(&cb, 1_000);
//
//     // If we were to drop `cb` here it would cause an exception to be raised
//     // when the timeout elapses. Instead we *return* our handle back to JS
//     // so JS can tell us later when it would like to deallocate this handle.
//     ClosureHandle(cb)
// }