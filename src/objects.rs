use std::rc::Rc;

use crate::{vec3::{Point3, Vec3}, ray::Ray};


pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: Point3, t: f32) -> Self {
        Self {
            p,
            normal: Vec3::new(0.0, 0.0, 0.0),
            t,
            front_face: true,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        // Sets the hit record normal vector.
        // NOTE: the parameter `outward_normal` is assumed to have unit length.

        self.front_face = Vec3::dot(&ray.dir, &outward_normal) < 0.0;
        self.normal = if self.front_face {*outward_normal} else {-*outward_normal};
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_tmin: f32, ray_tmax: f32) -> Option<HitRecord>;
}


pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32) -> Self {
        Self { center, radius: radius.max(0.0)}
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_tmin: f32, ray_tmax: f32) -> Option<HitRecord> {
        let oc = self.center - ray.origin;
        let a = ray.dir.length();
        let h = Vec3::dot(&ray.dir, &oc);
        let c = oc.length() - self.radius * self.radius;
        let discriminant = h*h - a*c;

        if discriminant < 0.0 {
            return None;
        } 

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrtd) / a;
        if root <= ray_tmin || root >= ray_tmax {
            root = (h + sqrtd) / a;
            if root <= ray_tmin || root >= ray_tmax {
                return None;
            }
        }
        
        let t = root;
        let p = ray.at(t);
        let outward_normal = (p - self.center) / self.radius;
        
        let mut rec = HitRecord::new(p, t);
        rec.set_face_normal(ray, &outward_normal);

        return Some(rec);

    }
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>, //maybe Rc
}

impl HittableList {
    pub fn new() -> Self {
        Self { objects: Vec::new() }
    }

    /*pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(Box::new(object))
    }*/
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_tmin: f32, ray_tmax: f32) -> Option<HitRecord> {
        let mut hit_anything = false;
        let mut closest_so_far = ray_tmax;
        let mut hr = HitRecord {
            p: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0, 
            front_face: true,
        };

        for object in self.objects.iter() {
            if let Some(hit) = object.hit(&ray, ray_tmin, ray_tmax) {
                hit_anything = true;
                closest_so_far = hit.t;
                hr = hit;
            }
        }

        if hit_anything { Some(hr) } else {None}
    }
}