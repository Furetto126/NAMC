use crate::math::round_at;
use nalgebra::{RealField, SVector};

pub trait Interpolable {
    fn interpolate(&self, other: &Self, t: f64) -> Self;
}

impl Interpolable for f64 {
    fn interpolate(&self, other: &Self, t: f64) -> Self {
        (1.0 - t) * self + t * other
    }
}

impl<N, const D: usize> Interpolable for SVector<N, D> where N: Interpolable + RealField {
    fn interpolate(&self, other: &Self, t: f64) -> Self {
        self.zip_map(other, |a, b| a.interpolate(&b, t))
    }
}

pub type InterpolationFunction = fn(t: f64) -> f64;

pub const LERP: InterpolationFunction = {
    const fn lerp(x: f64) -> f64 { round_at(x.clamp(0.0, 1.0), 3) }
    lerp
};

pub const SMOOTHSTEP: InterpolationFunction = {
    const fn smoothstep(x: f64) -> f64 { round_at((3.0*x*x - 2.0*x*x*x).clamp(0.0, 1.0), 3) }
    smoothstep
};

pub const EASE_IN_QUART: InterpolationFunction = {
    const fn ease_in_quart(x: f64) -> f64 { round_at((x*x*x*x).clamp(0.0, 1.0), 3) }
    ease_in_quart
};

pub const EASE_OUT_QUART: InterpolationFunction = {
    const fn ease_out_quart(x: f64) -> f64 { round_at((1.0 - (1.0-x)*(1.0-x)*(1.0-x)*(1.0-x)).clamp(0.0, 1.0), 3) }
    ease_out_quart
};

pub const EASE_INOUT_QUART: InterpolationFunction = {
    const fn ease_inout_quart(x: f64) -> f64 { 
        round_at(
            if x < 0.5 {
                (8.0*x*x*x*x).clamp(0.0, 1.0)
            } else {
                (1.0-(-2.0*x + 2.0)*(-2.0*x + 2.0)*(-2.0*x + 2.0)*(-2.0*x + 2.0)).clamp(0.0, 1.0)
            }
        , 3)
    }
    ease_inout_quart
};