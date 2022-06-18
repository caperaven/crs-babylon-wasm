#![feature(extern_types)]

mod babylon;
use wasm_bindgen::prelude::*;
use crate::babylon::{
    Color3, FreeCamera, HemisphericLight, Engine, Scene, Vector3,
    Sphere, SphereOptions, Ground, GroundOptions};

#[wasm_bindgen]
pub fn initialize(id: &str) -> Result<BabylonResult, String> {
    console_error_panic_hook::set_once();

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id(id).unwrap();

    let engine = Engine::new(&canvas);
    let scene = Scene::new(&engine);

    let color = Color3::from_hex_string("#ff0090");
    scene.set_clear_color(color);

    let camera = FreeCamera::new("camera", Vector3::new(0.0, 5.0, -10.0), &scene);
    camera.set_target(Vector3::zero());
    camera.attach_control(&canvas, true);

    let light = HemisphericLight::new("light", Vector3::new(0.0, 1.0, 0.0), &scene);
    light.set_intensity(0.7);

    let sphere = Sphere::new("sphere", SphereOptions { diameter: 2, segments: 32 }, &scene);
    sphere.set_absolute_position(Vector3::new(0.0, 1.0, 0.0));

    let _ground = Ground::new("ground", GroundOptions {width: 6, height: 6}, &scene);
}
