#![feature(extern_types)]

mod babylon;

use wasm_bindgen::prelude::*;
use crate::babylon::{Babylon, Color3, FreeCamera, Ground, GroundOptions, HemisphericLight, Sphere, SphereOptions, Vector3};
use crate::babylon::core::{Engine, Scene};

#[wasm_bindgen]
pub fn initialize(id: &str) -> Engine {
    console_error_panic_hook::set_once();

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id(id).unwrap();

    let babylon: Babylon = Babylon::new(&canvas);

    let color = Color3::from_hex_string("#ff0090");
    babylon.scene.set_clear_color(color);

    let camera = FreeCamera::new("camera", Vector3::new(0.0, 5.0, -10.0), &babylon.scene);
    camera.set_target(Vector3::zero());
    camera.attach_control(&canvas, true);

    let light = HemisphericLight::new("light", Vector3::new(0.0, 1.0, 0.0), &babylon.scene);
    light.set_intensity(0.7);

    let sphere = Sphere::new("sphere", SphereOptions { diameter: 2, segments: 32 }, &babylon.scene);
    sphere.set_absolute_position(Vector3::new(0.0, 1.0, 0.0));

    let _ground = Ground::new("ground", GroundOptions {width: 6, height: 6}, &babylon.scene);

    let Babylon {scene, engine} = babylon;

    let cb = Closure::wrap(Box::new(move || {
        scene.render();
    }) as Box<dyn FnMut()>);

    engine.run_render_loop(&cb);

    cb.forget();

    engine
}

#[wasm_bindgen]
pub fn change_background_color(scene: &Scene, color: &str) {
    let clr = Color3::from_hex_string(color);
    scene.set_clear_color(clr);
}
