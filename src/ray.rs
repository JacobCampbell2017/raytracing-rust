use crate::color::*;
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

pub fn hit_sphere(center: Point3, radius: f64, r: Ray) -> f64 {
    let oc = center - *r.origin();
    let a = r.direction().length_squared();
    let h = dot(*r.direction(), oc);
    let c = oc.length_squared() - radius * radius;
    let discriminant = h * h - a * c;

    if discriminant < 0.0 {
        return -1.0;
    }
    (h - discriminant.sqrt()) / a
}

pub fn ray_color(r: &Ray) -> Color {
    let t = hit_sphere(Point3::new_use(0.0, 0.0, -1.0), 0.5, *r);
    if t > 0.0 {
        let n = unit_vector(r.at(t) - Vec3::new_use(0.0, 0.0, -1.0));
        return 0.5 * Color::new_use(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
    }

    let unit_direction = unit_vector(*r.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * Color::new_use(1.0, 1.0, 1.0) + a * Color::new_use(0.5, 0.7, 1.0)
}
