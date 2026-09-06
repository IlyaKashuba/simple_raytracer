use crate::objects::{Hittable, HitRecord};
use core::f32;
use std::sync::Arc;

use crate::{aabb::Aabb, ray::Ray, util::{self, Interval}, vec3::{Point3, Vec3}};



pub struct Translate {
    pub object: Arc<dyn Hittable>,
    pub offset: Vec3,
    bbox: Aabb,
}

impl Translate {
    pub fn new(object: Arc<dyn Hittable>, offset: Vec3) -> Self {
        let bbox = *object.boinding_box() + offset;
        Self {
            object, offset, bbox
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let offset_ray = Ray::new(ray.origin - self.offset, ray.dir, ray.time);

        if let Some(mut hr) = self.object.hit(&offset_ray, ray_t) {
            hr.p = hr.p + self.offset;
            return Some(hr);
        } else {
            return None;
        }
    }

    fn boinding_box(&self) -> &Aabb {
        return &self.bbox;
    }
}


pub struct RotateY {
    object: Arc<dyn Hittable>,
    sin_theta: f32,
    cos_theta: f32,
    bbox: Aabb,
}

impl RotateY {
    pub fn new(object: Arc<dyn Hittable>, angle: f32) -> Self {
        let radians = util::degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = *object.boinding_box();

        let mut min = Point3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
        let mut max = Point3::new(-f32::INFINITY, -f32::INFINITY, -f32::INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f32 * bbox.x.max + (1-i) as f32 * bbox.x.min;
                    let y = j as f32 * bbox.y.max + (1-j) as f32 * bbox.y.min;
                    let z = k as f32 * bbox.z.max + (1-k) as f32 * bbox.z.min;

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(newx, y, newz);

                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }

        let bbox = Aabb::from_points(min, max);

        Self {
            object, sin_theta, cos_theta, bbox
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let origin = Point3 {
            x: (self.cos_theta * ray.origin.x) - (self.sin_theta * ray.origin.z),
            y: ray.origin.y,
            z: (self.sin_theta * ray.origin.x) + (self.cos_theta * ray.origin.z),
        };

        let direction = Point3 {
            x: (self.cos_theta * ray.dir.x) - (self.sin_theta * ray.dir.z),
            y: ray.dir.y,
            z: (self.sin_theta * ray.dir.x) + (self.cos_theta * ray.dir.z),
        };

        let rotated_ray = Ray::new(origin, direction, ray.time);

        if let Some(mut hr) = self.object.hit(&rotated_ray, ray_t) {
            hr.p = Point3 {
                x: (self.cos_theta * hr.p.x) - (self.sin_theta * hr.p.z),
                y: hr.p.y,
                z: (self.sin_theta * hr.p.x) + (self.cos_theta * hr.p.z),
            };
            hr.normal = Point3 {
                x: (self.cos_theta * hr.normal.x) - (self.sin_theta * hr.normal.z),
                y: hr.p.y,
                z: (self.sin_theta * hr.normal.x) + (self.cos_theta * hr.normal.z),
            };

            return Some(hr);
        } else {
            return None;
        }
    }

    fn boinding_box(&self) -> &Aabb {
        return &self.bbox;
    }
}