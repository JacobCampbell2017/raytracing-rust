mod camera;
mod hittable;
mod interval;
mod material;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;

use camera::*;
use hittable::{Hittable, HittableList};
use ray::*;
use sphere::Sphere;

// Left off 6.2

fn main() {
    // World
    let mut world = HittableList::new();

    world.add(Hittable::Sphere(Box::new(Sphere::new_use(
        Point3::new_use(0.0, 0.0, -1.0),
        0.5,
    ))));
    world.add(Hittable::Sphere(Box::new(Sphere::new_use(
        Point3::new_use(0.0, -100.5, -1.0),
        100.0,
    ))));

    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.render(Hittable::HittableList(Box::new(world)));
}
