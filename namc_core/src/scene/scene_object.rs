use std::fmt::Debug;

use nalgebra::Vector3;
use slotmap::new_key_type;

pub trait SceneObject: Debug {
    fn position(&self) -> Vector3<f64>;
    fn opacity(&self)  -> f64;

    fn set_position(&mut self, pos: Vector3<f64>);
    fn set_opacity(&mut self, op: f64);
}


new_key_type! { pub struct ObjectId; }