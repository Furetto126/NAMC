use nalgebra::Vector3;
use namc_animations::object_animations::CoreAnimations;
use namc_core::{math::interpolate::LERP, scene::{Scene, SceneObject}};


#[derive(Debug, Default)]
struct MyObject {
    position: Vector3<f64>,
    opacity: f64
}

impl SceneObject for MyObject {
    fn position(&self) -> Vector3<f64> {
        self.position
    }

    fn opacity(&self)  -> f64 {
        self.opacity
    }

    fn set_position(&mut self, pos: Vector3<f64>) {
        self.position = pos;
    }

    fn set_opacity(&mut self, op: f64) {
        self.opacity = op;
    }
}

fn main() {
    let mut scene = Scene::new();
    let obj_handle1 = scene.add_object(
        MyObject { position: [0.0, 0.0, 0.0].into(), opacity: 0.0 }
    );
    let obj_handle2 = scene.add_object(
        MyObject { position: [0.0, 0.0, 0.0].into(), opacity: 0.0 }
    );
    scene.play(
        obj_handle1.fade_in(10.0, Some(LERP)) + obj_handle2.fade_in(6.0, Some(LERP))
    );
    
    // Simulate rendering
    // ------------------
    let dt_ms = 100.0;
    while scene.timeline.time < 10.0 {
        scene.render_frame(dt_ms / 1000.0);
    }
}
