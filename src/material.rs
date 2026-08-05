use rand::{random_range};

use crate::{color::{self, Color}, objects::HitRecord, ray::Ray, vec3::Vec3};

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_rec: &HitRecord) -> Option<(Ray, Color)>;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, hit_rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = hit_rec.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = hit_rec.normal;
        }

        let scattered = Ray::new(hit_rec.p, scatter_direction, ray_in.time);
        let attenuation = self.albedo;
        
        return Some((scattered, attenuation));
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = Vec3::reflect(&ray_in.dir, &hit_rec.normal);
        let reflected = Vec3::unit_length(&reflected) + (self.fuzz * Vec3::random_unit_vector());
        let scattered = Ray::new(hit_rec.p, reflected, ray_in.time);
        let attenuation = self.albedo;

        if Vec3::dot(&scattered.dir, &hit_rec.normal) > 0.0 {
            return Some((scattered, attenuation));
        } else {
            return None;
        }
    }
}

pub struct Dielectric {
    pub refraction_index: f32,
}

impl Dielectric {
    pub fn new(refraction_index: f32) -> Self {
        Self {refraction_index}
    }

    pub fn reflectance(cosine: f32, refraction_index: f32) -> f32 {
        let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        let r0 = r0*r0;
        return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_rec: &HitRecord) -> Option<(Ray, Color)> {
        let attenuation = color::WHITE;
        let ri = if hit_rec.front_face { 
            1.0 / self.refraction_index } else {
                self.refraction_index
            };

        
        let unit_direction = Vec3::unit_length(&ray_in.dir);

        let cos_theta = Vec3::dot(&(-unit_direction), &hit_rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();
        
        let direction: Vec3;
        if ri * sin_theta > 1.0 || Dielectric::reflectance(cos_theta, ri) > random_range(0.0..1.0) {
            //must reflect
            direction = Vec3::reflect(&unit_direction, &hit_rec.normal);
        } else {
            //must refract
            direction = Vec3::refract(&unit_direction, &hit_rec.normal, ri);
        }

        let scattered = Ray::new(hit_rec.p, direction, ray_in.time);
        return Some((scattered, attenuation));
    }
}