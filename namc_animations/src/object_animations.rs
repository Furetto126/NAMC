use namc_core::{animation::Animation, math::{InterpolationFunction, interpolate::LERP}, scene::{ObjectId, ObjectMap}};
use namc_macro::animation;

#[animation]
pub struct FadeIn {
    pub target: ObjectId,
}

impl FadeIn {
    pub fn animate(&self, t: f64, scene_objects: &mut ObjectMap) {
        let object = scene_objects.get_object_mut(self.target).expect("Invalid ObjectId: ID not present in ObjectMap");
        object.set_opacity(t);
    }
}

pub trait CoreAnimations {
    fn fade_in(self, duration: f64, function: Option<InterpolationFunction>) -> Box<dyn Animation>;
} 

impl CoreAnimations for ObjectId {
    fn fade_in(self, duration: f64, interpolation_function: Option<InterpolationFunction>) -> Box<dyn Animation> {
        FadeIn::new(self, duration, interpolation_function.unwrap_or(LERP))
    }
}