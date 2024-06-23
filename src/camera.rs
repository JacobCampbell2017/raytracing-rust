#![allow(dead_code)]

use crate::hittable::*;
use crate::interval::*;
use crate::ray::*;
use crate::rtweekend::random_double;
use crate::vec3::*;

use core::f64::INFINITY;

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    image_height: i32,
    pixel_samples_scale: f64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    // Public Camera Paraeters
    pub fn render(&mut self, world: Hittable) {
        Self::initialize(self);

        print!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        for j in 0..(self.image_height) {
            eprintln!("\rScanlines remaining: {}", self.image_height - j);
            for i in 0..(self.image_width) {
                let mut pixel_color = Color::new();
                for _s in 0..(self.samples_per_pixel) {
                    let r = self.get_ray(i, j);
                    pixel_color += Self::ray_color(&r, self.max_depth, world.clone());
                }
                // eprintln!("Pixel Color: {}\t", pixel_color);
                write_color(pixel_color);
            }
        }
        eprintln!("\rDone.              ");
    }

    pub fn new() -> Self {
        Camera {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            image_height: 0,
            pixel_samples_scale: 0.0,
            center: Point3::new(),
            pixel00_loc: Point3::new(),
            pixel_delta_u: Vec3::new(),
            pixel_delta_v: Vec3::new(),
        }
    }

    // Private
    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        if self.image_height < 1 {
            self.image_height = 1;
        }

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        self.center = Point3::new();

        // Determine Viewport Dimensions
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width =
            viewport_height * ((self.image_width as f64) / self.image_height as f64);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3::new_use(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new_use(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate the location of the upper left pixel
        let viewport_upper_left = self.center
            - Vec3::new_use(0.0, 0.0, focal_length)
            - viewport_u / 2.0
            - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        // Construct a camera ray originiating from the origin and directed at a randomly sampled
        // point around the pixel location i, j.

        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new_use(&ray_origin, &ray_direction)
    }

    fn sample_square() -> Vec3 {
        Vec3::new_use(random_double() - 0.5, random_double() - 0.5, 0.0)
    }

    fn ray_color(r: &Ray, depth: i32, world: Hittable) -> Color {
        if depth <= 0 {
            return Color::new_use(0.0, 0.0, 0.0);
        }

        let mut rec: HitRecord = HitRecord::new();

        if world.hit(*r, Interval::new_use(0.001, INFINITY), &mut rec) {
            let scattered = Ray::new();
            let attenuation = Color::new();
            let direction = rec.normal + random_unit_vector();
            return 0.5 * Self::ray_color(&Ray::new_use(&rec.p, &direction), depth - 1, world);
        }

        let unit_direction = unit_vector(*r.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::new_use(1.0, 1.0, 1.0) + a * Color::new_use(0.5, 0.7, 1.0)
    }
}
