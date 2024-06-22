#![allow(dead_code)]
use core::f64::consts::PI;
use rand::Rng;

// Utility functions

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_double() -> f64 {
    rand::thread_rng().gen_range(0.0..1.0)
}

pub fn random_double_2(min: f64, max: f64) -> f64 {
    rand::thread_rng().gen_range(min..max)
}
