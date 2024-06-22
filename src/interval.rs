#![allow(dead_code)]
use core::f64;
use std::f64::{INFINITY, NEG_INFINITY};

pub const EMPTY: Interval = Interval {
    min: INFINITY,
    max: NEG_INFINITY,
};

pub const UNIVERSAL: Interval = Interval {
    min: NEG_INFINITY,
    max: INFINITY,
};

#[derive(Clone)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new() -> Self {
        Interval {
            min: INFINITY,
            max: NEG_INFINITY,
        }
    }

    pub fn new_use(minimum: f64, maximum: f64) -> Self {
        Interval {
            min: minimum,
            max: maximum,
        }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }
}

impl Default for Interval {
    fn default() -> Self {
        Self::new()
    }
}
