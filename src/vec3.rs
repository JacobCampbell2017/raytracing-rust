#![allow(dead_code)]
use core::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub};

use crate::interval::Interval;

#[derive(Clone, Copy)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new() -> Self {
        Vec3 { e: [0.0, 0.0, 0.0] }
    }

    pub fn new_use(x: f64, y: f64, z: f64) -> Self {
        Vec3 { e: [x, y, z] }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        (self.e[0] * self.e[0]) + (self.e[1] * self.e[1]) + (self.e[2] * self.e[2])
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self::new()
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, i: usize) -> &Self::Output {
        &self.e[i]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.e[i]
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            e: [-(self.e[0]), -(self.e[1]), -(self.e[2])],
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            e: [
                self.e[0] + rhs.e[0],
                self.e[1] + rhs.e[1],
                self.e[2] + rhs.e[2],
            ],
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            e: [
                self.e[0] - rhs.e[0],
                self.e[1] - rhs.e[1],
                self.e[2] - rhs.e[2],
            ],
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3 {
            e: [
                self.e[0] * rhs.e[0],
                self.e[1] * rhs.e[1],
                self.e[2] * rhs.e[2],
            ],
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            e: [rhs.e[0] * self, rhs.e[1] * self, rhs.e[2] * self],
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        rhs * self
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        (1.0 / rhs) * self
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.e[0] /= rhs;
        self.e[1] /= rhs;
        self.e[2] /= rhs;
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

// Functions for vecs
pub fn dot(v: Vec3, rhs: Vec3) -> f64 {
    (v.e[0] * rhs.e[0]) + (v.e[1] * rhs.e[1]) + (v.e[2] * rhs.e[2])
}

pub fn cross(v: Vec3, rhs: Vec3) -> Vec3 {
    Vec3 {
        e: [
            (v.e[1] * rhs.e[2] - v.e[2] * rhs.e[1]),
            (v.e[2] * rhs.e[0] - v.e[0] * rhs.e[2]),
            (v.e[0] * rhs.e[1] - v.e[1] * rhs.e[0]),
        ],
    }
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

pub type Color = Vec3;

pub fn write_color(pixel_color: Color) {
    let r = pixel_color.x() / 100.0;
    let g = pixel_color.y() / 100.0;
    let b = pixel_color.z() / 100.0;

    // Translate the [0,1] component values to the byte range [0,255]
    let intensity: Interval = Interval::new_use(0.000, 0.999);

    let rbyte = (256 as f64 * intensity.clamp(r)) as i32;
    let gbyte = (256 as f64 * intensity.clamp(g)) as i32;
    let bbyte = (256 as f64 * intensity.clamp(b)) as i32;

    // Write pixel color components
    println!("{} {} {}", rbyte, gbyte, bbyte);
}
