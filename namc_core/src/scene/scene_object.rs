use std::{fmt::Debug, marker::PhantomData};

use nalgebra::Vector3;
use slotmap::new_key_type;

pub trait SceneObject {
    fn position(&self) -> Vector3<f64>;
    fn opacity(&self)  -> f64;

    fn set_position(&mut self, pos: Vector3<f64>);
    fn set_opacity(&mut self, op: f64);
}

new_key_type! { pub struct ObjectId; }

#[derive(Copy, Clone)]
pub struct ObjectHandle<T: SceneObject> {
    pub raw: ObjectId,
    pub _marker: std::marker::PhantomData<T>
}

impl<T: SceneObject> ObjectHandle<T> {
    pub fn new(raw: ObjectId) -> ObjectHandle<T> {
        Self { raw, _marker: PhantomData }
    }
}