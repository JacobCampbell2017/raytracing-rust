// Material class and ENUMS

use crate::{
    hittable::HitRecord,
    ray::Ray,
    vec3::{random_unit_vector, reflect, Color},
};

#[derive(Clone)]
pub enum Material {
    Lambertian(Box<Lambertian>),
    Metal(Box<Metal>),
}

impl Material {
    pub fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        match self {
            Material::Lambertian(l) => l.scatter(r_in, rec, attenuation, scattered),
            Material::Metal(m) => m.scatter(r_in, rec, attenuation, scattered),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo_t: Color) -> Self {
        Lambertian { albedo: albedo_t }
    }

    pub fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new_use(&rec.p, &scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

#[derive(Clone)]
pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo_t: Color) -> Self {
        Metal { albedo: albedo_t }
    }

    pub fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(*r_in.direction(), rec.normal);
        *scattered = Ray::new_use(&rec.p, &reflected);
        *attenuation = self.albedo;
        true
    }
}
