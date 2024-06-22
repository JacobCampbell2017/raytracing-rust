use crate::vec3::*;

pub type Point3 = Vec3;

#[derive(Clone, Copy)]
pub struct Ray {
    dir: Vec3,
    orig: Point3,
}

impl Ray {
    pub fn new() -> Self {
        Ray {
            dir: Vec3::new(),
            orig: Point3::new(),
        }
    }

    pub fn new_use(origin: &Point3, direction: &Vec3) -> Self {
        Ray {
            dir: *direction,
            orig: *origin,
        }
    }

    pub fn origin(&self) -> &Point3 {
        &self.orig
    }

    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}

impl Default for Ray {
    fn default() -> Self {
        Ray::new()
    }
}
