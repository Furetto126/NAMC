use nalgebra::Vector3;
use namc_core::{animation::Animation, math::{InterpolationFunction, interpolate::LERP}, scene::{ObjectId, ObjectMap, SceneObject, scene_object::ObjectHandle}};
use namc_macro::derive_animation;
use namc_core::math::Interpolable;

#[derive_animation]
pub struct FadeIn<T: SceneObject> {
    pub target: ObjectHandle<T>,
}
impl<T: SceneObject> FadeIn<T> {
    pub fn animate(&mut self, t: f64, scene_objects: &mut ObjectMap) {
        let object = scene_objects.get_object_mut(&self.target).expect("Invalid ObjectId: ID not present in ObjectMap");
        object.set_opacity(t);
    }
}

#[derive_animation]
pub struct MoveTo<T: SceneObject> {
    pub target: ObjectHandle<T>,
    pub start_pos: Option<Vector3<f64>>,
    pub target_pos: Vector3<f64>,

}
impl<T: SceneObject> MoveTo<T> {
    pub fn animate(&mut self, t: f64, scene_objects: &mut ObjectMap) {
        let object = scene_objects.get_object_mut(&self.target).expect("Invalid ObjectId: ID not present in ObjectMap");

        if self.start_pos.is_none() {
            self.start_pos = Some(object.position());
        }

        object.set_position(self.start_pos.unwrap().interpolate(&self.target_pos, t));
    }
}

pub trait CoreAnimations {
    fn fade_in(self, duration: f64, interpolation_function: Option<InterpolationFunction>) -> Box<dyn Animation>;
    fn move_to(self, target_pos: Vector3<f64>, duration: f64, interpolation_function: Option<InterpolationFunction>) -> Box<dyn Animation>;
} 

impl<T: SceneObject> CoreAnimations for ObjectHandle<T> {
    fn fade_in(self, duration: f64, interpolation_function: Option<InterpolationFunction>) -> Box<dyn Animation> {
        FadeIn::new(self, duration, interpolation_function.unwrap_or(LERP))
    }

    fn move_to(self, target_pos: Vector3<f64>, duration: f64, interpolation_function: Option<InterpolationFunction>) -> Box<dyn Animation> {
        MoveTo::new(self, None, target_pos, duration, interpolation_function.unwrap_or(LERP))
    }
}