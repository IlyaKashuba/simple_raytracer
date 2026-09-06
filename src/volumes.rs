use crate::color::Color;
use crate::objects::{HitRecord, Hittable};
use crate::material::{Isotropic, Material};
use crate::texture::{TexCoords, Texture};
use crate::util::Interval;
use crate::vec3::Vec3;
use std::sync::Arc;
use rand::{self, random_range};


pub struct ConstantMedium  {
    pub boundary: Arc<dyn Hittable>,
    pub neg_inv_density: f32,
    pub phase_function: Arc<dyn Material>,
}  

impl ConstantMedium {
    pub fn from_texture(boundary: Arc<dyn Hittable>, density: f32, tex: Arc<Texture>) -> Self {
        Self { boundary, neg_inv_density: -1.0 / density, phase_function: Arc::new(Isotropic::from_texture(Arc::clone(&tex))) }
    }

    pub fn from_color(boundary: Arc<dyn Hittable>, density: f32, albedo: Color) -> Self {
        Self { boundary, neg_inv_density: -1.0 / density, phase_function: Arc::new(Isotropic::from_color(albedo)) }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: &crate::ray::Ray, ray_t: &crate::util::Interval) -> Option<HitRecord> {
        if let Some(mut hr1) = self.boundary.hit(ray, &Interval::UNIVERSE) {
            if let Some(mut hr2) = self.boundary.hit(ray, &Interval::new(hr1.t + 0.0001, f32::INFINITY)) {
                if hr1.t < ray_t.min {
                    hr1.t = ray_t.min;
                }
                if hr2.t > ray_t.max {
                    hr2.t = ray_t.max;
                }

                if hr1.t >= hr2.t {
                    return None;
                }

                if hr1.t < 0.0 {
                    hr1.t = 0.0;
                }

                let ray_length  = ray.dir.length();
                let distance_inside_boundary = (hr2.t - hr1.t) * ray_length;
                let hit_distance = self.neg_inv_density * (random_range(0.0..1.0) as f32).ln();

                if hit_distance > distance_inside_boundary {
                    return None;
                }
                
                let t = hr1.t + hit_distance / ray_length;
                let p = ray.at(t);
                
                let hr = HitRecord{
                    p: p,
                    t: t, 
                    normal: Vec3::new(1.0, 0.0, 0.0),
                    front_face: true,
                    uv: TexCoords(0.0, 0.0),
                    mat: Arc::clone(&self.phase_function),
                };
                return Some(hr);
            } else {
                return None;
            }
        } else {
            return None;
        }
    }

    fn boinding_box(&self) -> &crate::aabb::Aabb {
        return self.boundary.boinding_box();
    }
}