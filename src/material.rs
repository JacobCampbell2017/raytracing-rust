// Material class and ENUMS

use crate::{
    hittable::HitRecord,
    ray::Ray,
    rtweekend::random_double,
    vec3::{dot, random_unit_vector, reflect, refract, unit_vector, Color, Vec3},
};

#[derive(Clone)]
pub enum Material {
    Lambertian(Box<Lambertian>),
    Metal(Box<Metal>),
    Dielectric(Box<Dielectric>),
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
            Material::Dielectric(d) => d.scatter(r_in, rec, attenuation, scattered),
        }
    }

    pub fn new() -> Self {
        Material::Lambertian(Box::new(Lambertian::new(Color::new())))
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
        _r_in: &Ray,
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
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo_t: Color, fuzz_t: f64) -> Self {
        Metal {
            albedo: albedo_t,
            fuzz: (if fuzz_t < 1.0 { fuzz_t } else { 1.0 }),
        }
    }

    pub fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuaion: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut reflected = reflect(*r_in.direction(), rec.normal);
        reflected = unit_vector(reflected) + (self.fuzz * random_unit_vector());
        *scattered = Ray::new_use(&rec.p, &reflected);
        *attenuaion = self.albedo;
        dot(*scattered.direction(), rec.normal) > 0.0
    }
}

#[derive(Clone)]
pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(r_index: f64) -> Self {
        Dielectric {
            refraction_index: r_index,
        }
    }

    pub fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attentuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attentuation = Color::new_use(1.0, 1.0, 1.0);
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = unit_vector(*r_in.direction());
        let cos_theta = f64::min(dot(-unit_direction, rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;
        let direction: Vec3;

        if cannot_refract || Self::reflectance(&self, cos_theta, ri) > random_double() {
            direction = reflect(unit_direction, rec.normal);
        } else {
            direction = refract(unit_direction, rec.normal, ri);
        }

        *scattered = Ray::new_use(&rec.p, &direction);
        true
    }

    fn reflectance(&self, cosine: f64, refraction_index: f64) -> f64 {
        // Use Schlick's approximation for reflectance
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * f64::powf(1.0 - cosine, 5.0)
    }
}
