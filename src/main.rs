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
use rtweekend::{random_double, random_double_2};
use sphere::Sphere;
use vec3::{Color, Vec3};

// Left off 6.2

fn main() {
    // World
    let mut world = HittableList::new();

    let ground_material =
        Material::Lambertian(Box::new(Lambertian::new(Color::new_use(0.5, 0.5, 0.5))));
    world.add(Hittable::Sphere(Box::new(Sphere::new_use(
        Point3::new_use(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ))));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new_use(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );

            if (center - Point3::new_use(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Material;

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    sphere_material = Material::Lambertian(Box::new(Lambertian::new(albedo)));
                    world.add(Hittable::Sphere(Box::new(Sphere::new_use(
                        center,
                        0.2,
                        sphere_material,
                    ))));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_2(0.5, 1.0);
                    let fuzz = random_double_2(0.0, 0.5);
                    sphere_material = Material::Metal(Box::new(Metal::new(albedo, fuzz)));
                    world.add(Hittable::Sphere(Box::new(Sphere::new_use(
                        center,
                        0.2,
                        sphere_material,
                    ))));
                } else {
                    // glass
                    sphere_material = Material::Dielectric(Box::new(Dielectric::new(1.5)));
                    world.add(Hittable::Sphere(Box::new(Sphere::new_use(
                        center,
                        0.2,
                        sphere_material,
                    ))));
                }
            }
        }
    }

    let material1 = Material::Dielectric(Box::new(Dielectric::new(1.5)));
    world.add(Hittable::Sphere(Box::new(Sphere::new_use(
        Point3::new_use(0.0, 1.0, 0.0),
        1.0,
        material1,
    ))));

    let material2 = Material::Lambertian(Box::new(Lambertian::new(Color::new_use(0.4, 0.2, 0.1))));
    world.add(Hittable::Sphere(Box::new(Sphere::new_use(
        Point3::new_use(-4.0, 1.0, 0.0),
        1.0,
        material2,
    ))));

    let material3 = Material::Metal(Box::new(Metal::new(Color::new_use(0.7, 0.6, 0.5), 0.0)));
    world.add(Hittable::Sphere(Box::new(Sphere::new_use(
        Point3::new_use(4.0, 1.0, 0.0),
        1.0,
        material3,
    ))));

    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.vfov = 20.0;
    cam.lookfrom = Point3::new_use(13.0, 2.0, 3.0);
    cam.lookat = Point3::new_use(0.0, 0.0, 0.0);
    cam.vup = Vec3::new_use(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;

    cam.render(Hittable::HittableList(Box::new(world)));
}
