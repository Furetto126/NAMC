use std::fmt::Debug;
use slotmap::SlotMap;
use derive_more::{Deref, DerefMut};

use crate::animation::{ParallelAnimation, Timeline};
use super::{SceneObject, ObjectId};

// Wrapper Type over SlotMap
#[derive(Deref, DerefMut)]
pub struct ObjectMap(SlotMap<ObjectId, Box<dyn SceneObject>>);

impl ObjectMap {
    pub fn new() -> Self {
        ObjectMap(SlotMap::with_key())
    }

    pub fn get_object(&self, object_id: ObjectId) -> Option<&dyn SceneObject> {
        self.0.get(object_id).map(|o| o.as_ref())
    }
    pub fn get_object_mut(&mut self, object_id: ObjectId) -> Option<&mut (dyn SceneObject + 'static)> {
        self.0.get_mut(object_id).map(|o| o.as_mut())
    }
}

pub struct Scene {
    pub timeline: Timeline,
    objects: ObjectMap
}

impl Scene {
    pub fn new() -> Self {
        Scene { timeline: Timeline::new(), objects: ObjectMap::new() }
    }

    pub fn add_object<T>(&mut self, obj: T) -> ObjectId
    where T: SceneObject + 'static,
    {
        self.objects.insert(Box::new(obj) as Box<dyn SceneObject>)
    }

    pub fn get_object(&self, object_id: ObjectId) -> Option<&dyn SceneObject> {
        self.objects.get_object(object_id)
    }

    pub fn get_object_mut(&mut self, object_id: ObjectId) -> Option<&mut (dyn SceneObject + 'static)> {
        self.objects.get_object_mut(object_id)
    }

    pub fn play<T>(&mut self, animation: T)
    where T: Into<ParallelAnimation>
    {
        self.timeline.add_animation(animation);
    }

    pub fn render_frame(&mut self, dt: f64) {
        let Scene { objects, timeline } = self;
        timeline.update(dt, objects);
    }
}