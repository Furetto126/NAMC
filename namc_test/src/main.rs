use nalgebra::Vector3;
use namc_animations::object_animations::CoreAnimations;
use namc_core::{animation::Animation, math::{InterpolationFunction, interpolate::LERP}, scene::{ObjectId, ObjectMap, Scene, SceneObject, scene_object::{self, ObjectHandle}}};
use namc_macro::{derive_animation, derive_scene_object};

#[derive_scene_object]
#[derive(Debug, Default)]
struct MyObject;

#[derive_scene_object]
#[derive(Debug, Default)]
struct MyCircle {
    radius: f64
}

#[derive_animation]
struct ChangeRadius {
    target: ObjectHandle<MyCircle>
}
impl ChangeRadius {
    pub fn animate(&mut self, t: f64, scene_objects: &mut ObjectMap) {
        let object = scene_objects.get_object_mut(&self.target).expect("Invalid ObjectId: ID not present in ObjectMap");
        object.radius = t;
    }
}

trait CircleAnimations {
    fn change_radius(self, duration: f64, interpolation_function: Option<InterpolationFunction>) -> Box<dyn Animation>;
}

impl CircleAnimations for ObjectHandle<MyCircle> {
    fn change_radius(self, duration: f64, interpolation_function: Option<InterpolationFunction>) -> Box<dyn Animation> {
        ChangeRadius::new(self, duration, interpolation_function.unwrap_or(LERP))
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
    let circle_handle = scene.add_object(
        MyCircle { position: [0.0, 0.0, 0.0].into(), opacity: 0.0, radius: 0.0 }
    );
    scene.play(
        obj_handle1.fade_in(10.0, Some(LERP)) +
        obj_handle2.move_to([1.0, 2.0, 4.0].into(), 11.0, Some(LERP)) +
        circle_handle.change_radius(5.0, Some(LERP))
    );
    
    // Simulate rendering
    // ------------------
    let dt_ms = 100.0;
    while scene.timeline.time < 11.0 {
        scene.render_frame(dt_ms / 1000.0);
    }
}
