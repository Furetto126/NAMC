use nalgebra::Vector3;
use namc_animations::object_animations::CoreAnimations;
use namc_core::{math::interpolate::LERP, scene::Scene};
use namc_macro::derive_scene_object;

#[derive_scene_object]
#[derive(Debug, Default)]
struct MyObject;

fn main() {
    let mut scene = Scene::new();
    let obj_handle1 = scene.add_object(
        MyObject { position: [0.0, 0.0, 0.0].into(), opacity: 0.0 }
    );
    let obj_handle2 = scene.add_object(
        MyObject { position: [0.0, 0.0, 0.0].into(), opacity: 0.0 }
    );
    scene.play(
        obj_handle1.fade_in(10.0, Some(LERP)) +
        obj_handle2.move_to([1.0, 2.0, 4.0].into(), 11.0, Some(LERP))
    );
    
    // Simulate rendering
    // ------------------
    let dt_ms = 100.0;
    while scene.timeline.time < 11.0 {
        scene.render_frame(dt_ms / 1000.0);
    }
}
