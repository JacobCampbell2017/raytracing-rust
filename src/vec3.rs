// vec3 class

use core::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub};

#[derive(Clone)]
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

    pub fn dot(&self, rhs: Vec3) -> f64 {
        (self.e[0] * rhs.e[0]) + (self.e[1] * rhs.e[1]) + (self.e[2] * rhs.e[2])
    }

    pub fn cross(&self, rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [
                (self.e[1] * rhs.e[2] - self.e[2] * rhs.e[1]),
                (self.e[2] * rhs.e[0] - self.e[0] * rhs.e[2]),
                (self.e[0] * rhs.e[1] - self.e[1] * rhs.e[0]),
            ],
        }
    }

    pub fn unit_vector(&self) -> Vec3 {
        self.clone() / self.length()
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
