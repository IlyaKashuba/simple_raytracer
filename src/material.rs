use crate::{color::Color, objects::HitRecord, ray::Ray, vec3::Vec3};

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

        let scattered = Ray::new(hit_rec.p, scatter_direction);
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
        let scattered = Ray::new(hit_rec.p, reflected);
        let attenuation = self.albedo;

        if Vec3::dot(&scattered.dir, &hit_rec.normal) > 0.0 {
            return Some((scattered, attenuation));
        } else {
            return None;
        }
    }
}