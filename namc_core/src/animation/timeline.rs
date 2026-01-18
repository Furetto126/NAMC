use std::collections::VecDeque;

use crate::{math::round_at, scene::ObjectMap};
use super::animation::ParallelAnimation;

#[derive(Default)]
pub struct Timeline {
    pub time: f64,
    tracks: VecDeque<ParallelAnimation>
}

impl Timeline {
    pub fn new() -> Self {
        Timeline { time: 0.0, tracks: VecDeque::new() }
    }

    pub fn update(&mut self, dt: f64, scene_objects: &mut ObjectMap) {
        if let Some(pa) = self.tracks.front_mut() {
            if !pa.is_finished() {
                //println!("Time: {}-{}", self.time, round_at(self.time+dt, 3));
                pa.tick(dt, scene_objects);
            }
            else {
                self.tracks.pop_front();
            }
        }

        self.time = round_at(self.time+dt, 3);
    }

    pub fn add_animation<T>(&mut self, anim: T) where T: Into<ParallelAnimation> {
        self.tracks.push_back(anim.into());
    }
}