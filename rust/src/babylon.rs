#![allow(unused_imports)]

mod cameras;
mod lights;
mod meshes;
mod core;

use web_sys::Element;
use crate::babylon::core::{Engine, Scene};
use super::*;

pub use cameras::*;
pub use lights::*;
pub use self::core::*;
pub use meshes::*;

pub struct Babylon {
    pub engine: Engine,
    pub scene: Scene,
    loop_handle: Option<Closure<dyn FnMut()>>
}

impl Babylon {
    pub fn new(canvas: &Element) -> Babylon {
        let engine = Engine::new(&canvas);
        let scene = Scene::new(&engine);

        Babylon {
            engine,
            scene,
            loop_handle: None
        }
    }

    pub fn run_loop(&mut self) {
        // self.engine.run_render_loop(|| => self.scene.render())
    }
}