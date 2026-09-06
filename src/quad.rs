
use crate::objects::{HitRecord, Hittable, HittableList};
use crate::texture::TexCoords;
use crate::util::Interval;
use crate::{aabb::Aabb, material::Material};
use crate::{Vec3, Point3};

use std::sync::Arc;

pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    mat: Arc<dyn Material>,
    bbox: Aabb,
    normal: Vec3,
    d: f32,
    w: Vec3,
}

impl Quad {
    pub fn new(q: Point3, u: Vec3, v: Vec3, mat: Arc<dyn Material>) -> Self {
        let bbox = Self::calculate_bounding_box(q, u, v);
        let n = Vec3::cross(&u, &v);
        let normal = n.unit_length();
        let d = Vec3::dot(&normal, &q);
        let w = n / Vec3::dot(&n, &n);
        Self {
            q, u, v, mat, bbox, normal, d, w,
        }
    }

    fn calculate_bounding_box(q: Point3, u: Vec3, v: Vec3) -> Aabb {
        let bbox_diagonal1 = Aabb::from_points(q, q + u + v);
        let bbox_diagonal2 = Aabb::from_points(q + u, q + v);

        return Aabb::from_2(&bbox_diagonal1, &bbox_diagonal2);
    }

    pub fn is_interior(a: f32, b: f32) -> Option<TexCoords> {
        let unit_interval = Interval::new(0.0, 1.0);

        if !unit_interval.contains(a) || !unit_interval.contains(b) {
            return None;
        } else {
            return Some(TexCoords(a, b));
        }
        
    }

    pub fn create_box(a: Point3, b: Point3, mat: Arc<dyn Material>) -> HittableList {
        let mut sides = HittableList::new_empty();

        let min = Point3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z));
        let max = Point3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z));

        let dx = Vec3::new(max.x - min.x, 0.0, 0.0);
        let dy = Vec3::new(0.0, max.y - min.y, 0.0);
        let dz = Vec3::new(0.0, 0.0, max.z - min.z);

        sides.add(Quad::new(Point3::new(min.x, min.y, max.z), dx, dy, Arc::clone(&mat))); //front
        sides.add(Quad::new(Point3::new(max.x, min.y, max.z), -dz, dy, Arc::clone(&mat))); //right
        sides.add(Quad::new(Point3::new(max.x, min.y, min.z), -dx, dy, Arc::clone(&mat))); //back
        sides.add(Quad::new(Point3::new(min.x, min.y, min.z), dz, dy, Arc::clone(&mat))); //left
        sides.add(Quad::new(Point3::new(min.x, max.y, max.z), dx, -dz, Arc::clone(&mat))); //top
        sides.add(Quad::new(Point3::new(min.x, min.y, min.z), dx, dz, Arc::clone(&mat))); //bottom
    
        return sides;
    }
}

impl Hittable for Quad {
    fn hit(&self, ray: &crate::ray::Ray, ray_t: &crate::util::Interval) -> Option<HitRecord> {
        let denom = Vec3::dot(&self.normal, &ray.dir);
        if denom.abs() < 1e-8_f32 {
            return None
        };

        let t = (self.d - Vec3::dot(&self.normal, &ray.origin)) / denom;
        if !ray_t.contains(t) {
            return None;
        }

        let intersection_point = ray.at(t);
        
        
        let planar_hitpt_vector = intersection_point - self.q;
        let alpha = Vec3::dot(&self.w, &Vec3::cross(&planar_hitpt_vector, &self.v));
        let beta = Vec3::dot(&self.w, &Vec3::cross(&self.u, &planar_hitpt_vector));

        if let Some(uv) = Self::is_interior(alpha, beta) {
            let mut hit_rec = HitRecord::new(intersection_point, t, Arc::clone(&self.mat));
            hit_rec.uv = uv;
            hit_rec.set_face_normal(ray, &self.normal);
            return Some(hit_rec);
        } else {
            return None
        }
        
    }

    fn boinding_box(&self) -> &Aabb {
        return &self.bbox;
    }
}