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
use material::{Dielectric, Lambertian, Material, Metal};
use ray::*;
use sphere::Sphere;
use vec3::Color;

// Left off 6.2

fn main() {
    // World
    let mut world = HittableList::new();

    let material_ground =
        Material::Lambertian(Box::new(Lambertian::new(Color::new_use(0.8, 0.8, 0.0))));
    let material_center =
        Material::Lambertian(Box::new(Lambertian::new(Color::new_use(0.1, 0.2, 0.5))));
    let material_left = Material::Dielectric(Box::new(Dielectric::new(1.50)));
    let material_right = Material::Metal(Box::new(Metal::new(Color::new_use(0.8, 0.6, 0.2), 1.0)));

    world.add(Hittable::Sphere(Box::new(Sphere::new_use(
        Point3::new_use(0.0, 0.0, -1.2),
        0.5,
        material_center,
    ))));
    world.add(Hittable::Sphere(Box::new(Sphere::new_use(
        Point3::new_use(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ))));
    world.add(Hittable::Sphere(Box::new(Sphere::new_use(
        Point3::new_use(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    ))));
    world.add(Hittable::Sphere(Box::new(Sphere::new_use(
        Point3::new_use(1.0, 0.0, -1.0),
        0.5,
        material_right,
    ))));

    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.render(Hittable::HittableList(Box::new(world)));
}
