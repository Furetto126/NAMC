use std::{any::Any, fmt::Debug, marker::PhantomData};

use nalgebra::Vector3;
use slotmap::new_key_type;

pub trait SceneObject: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn position(&self) -> Vector3<f64>;
    fn opacity(&self)  -> f64;

    fn set_position(&mut self, pos: Vector3<f64>);
    fn set_opacity(&mut self, op: f64);
}

new_key_type! { pub struct ObjectId; }

#[derive(Copy, Clone)]
pub struct ObjectHandle<T: SceneObject> {
    pub raw: ObjectId,
    _marker: std::marker::PhantomData<T>
}

impl<T: SceneObject> ObjectHandle<T> {
    pub fn new(raw: ObjectId) -> ObjectHandle<T> {
        Self { raw, _marker: PhantomData }
    }
}