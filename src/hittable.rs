use crate::ray::*;
use crate::sphere::Sphere;
use crate::vec3::*;

#[derive(Clone)]
pub enum Hittable {
    Sphere(Box<Sphere>),
    HittableList(Box<HittableList>),
}

impl Hittable {
    pub fn hit(&self, r: Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool {
        match self {
            Hittable::HittableList(h) => h.hit(r, ray_tmin, ray_tmax, rec),
            Hittable::Sphere(s) => s.hit(r, ray_tmin, ray_tmax, rec),
        }
    }
}

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        // Sets the hit record normal vector
        // NOTE: the parameter `outward_normal` is assumed to have unit length.

        self.front_face = dot(*r.direction(), outward_normal) < 0.0;

        if self.front_face {
            self.normal = outward_normal;
        } else {
            self.normal = -outward_normal;
        }
    }
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord {
            p: Point3::new(),
            normal: Vec3::new(),
            t: 0.0,
            front_face: false,
        }
    }
}

// Struct for list of hittables
#[derive(Clone)]
pub struct HittableList {
    objects: Vec<Hittable>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn new_use(&mut self, object: Hittable) {
        self.add(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Hittable) {
        self.objects.push(object);
    }

    pub fn hit(&self, r: Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = ray_tmax;

        for object in &self.objects {
            if object.hit(r, ray_tmin, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }

        hit_anything
    }
}

impl Default for HittableList {
    fn default() -> Self {
        Self::new()
    }
}
