use std::ops::Add;
use crate::scene::ObjectMap;

pub trait Animation {
    fn duration(&self) -> f64;
    fn update(&mut self, t: f64, scene_objects: &mut ObjectMap);
}

pub struct ParallelAnimation {
    elapsed: f64,
    animations: Vec<Box<dyn Animation>>
}

impl ParallelAnimation {
    pub fn new(a: Box<dyn Animation>, b: Box<dyn Animation>) -> Self {
        Self { elapsed: 0.0, animations: vec![a, b] }
    }

    pub fn from_single(a: Box<dyn Animation>) -> Self {
        Self { elapsed: 0.0, animations: vec![a] }
    }

    pub fn push(mut self, anim: Box<dyn Animation>) -> Self {
        self.animations.push(anim);
        self
    }

    pub fn duration(&self) -> f64 {
        let mut max_dur = f64::MIN;
        for a in &self.animations {
            max_dur = f64::max(a.duration(), max_dur);
        }
        max_dur

        // TODO: Cache duration and update it on "animations" change
    }

    pub fn elapsed(&self) -> f64 {
        self.elapsed
    }

    pub fn tick(&mut self, dt: f64, scene_objects: &mut ObjectMap) {
        self.elapsed += dt;

        for a in &mut self.animations {
            // Calculate t in [0.0 -> 1.0] based on duration of single animation
            let t = self.elapsed / a.duration();
            if t <= 1.0 {
                a.update(t, scene_objects);
            }
        }
    }

    pub fn is_finished(&self) -> bool {
        self.elapsed() >= self.duration()
    }
}

impl Add for Box<dyn Animation> {
    type Output = ParallelAnimation;

    fn add(self, rhs: Self) -> Self::Output {
        ParallelAnimation::new(self, rhs)
    }
}

impl Add<Box<dyn Animation>> for ParallelAnimation {
    type Output = ParallelAnimation;

    fn add(mut self, rhs: Box<dyn Animation>) -> Self::Output {
        self.animations.push(rhs);
        self
    }
}

impl Add<ParallelAnimation> for Box<dyn Animation> {
    type Output = ParallelAnimation;

    fn add(self, mut rhs: ParallelAnimation) -> Self::Output {
        rhs.animations.insert(0, self);
        rhs
    }
}

impl From<Box<dyn Animation>> for ParallelAnimation {
    fn from(value: Box<dyn Animation>) -> Self {
        ParallelAnimation::from_single(value)
    }
}