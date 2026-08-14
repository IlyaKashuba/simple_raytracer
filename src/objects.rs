use std::rc::{self, Rc};

use crate::{aabb::Aabb, color, material::{Lambertian, Material}, ray::Ray, util::Interval, vec3::{Point3, Vec3}};


//#[derive(PartialEq)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    //pub mat: &'a dyn Material,
    pub mat: Rc<dyn Material>,
}

impl HitRecord {
    pub fn new(p: Point3, t: f32, mat: Rc<dyn Material>) -> Self {
        Self {
            p,
            normal: Vec3::new(0.0, 0.0, 0.0),
            t,
            front_face: true,
            mat
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
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord>;
    fn boinding_box(&self) -> &Aabb;
}


pub struct Sphere {
    pub center: Ray,
    pub radius: f32,
    pub mat: Rc<dyn Material>,
    bbox: Aabb,
}

impl Sphere {
    pub fn new_static(static_center: Point3, radius: f32, mat: Rc<dyn Material>) -> Self {
        let center = Ray::new(static_center, Vec3::ZERO, 0.0);
        let rvec = Vec3::new(radius, radius, radius);
        let bbox = Aabb::from_points(static_center - rvec, static_center + rvec);

        Self { center, radius: radius.max(0.0), mat, bbox}
    }

    pub fn new_moving(center1: Point3, center2: Point3, radius: f32, mat: Rc<dyn Material>) -> Self {
        let center = Ray::new(center1, center2 - center1, 0.0);
        let rvec = Vec3::new(radius, radius, radius);
        let box1 = Aabb::from_points(center.at(0.0) - rvec, center.at(0.0) + rvec);
        let box2 = Aabb::from_points(center.at(1.0) - rvec, center.at(1.0) + rvec);
        let bbox = Aabb::from_2(&box1, &box2);

        Self { center, radius: radius.max(0.0), mat, bbox}
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let current_center = self.center.at(ray.time);
        let oc = current_center - ray.origin;
        let a = ray.dir.length_squared();
        let h = Vec3::dot(&ray.dir, &oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h*h - a*c;

        if discriminant < 0.0 {
            return None;
        } 

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }
        
        let t = root;
        let p = ray.at(t);
        let outward_normal = (p - current_center) / self.radius;
        
        let mut rec = HitRecord::new(p, t, Rc::clone(&self.mat));
        rec.set_face_normal(ray, &outward_normal);

        return Some(rec);

    }

    fn boinding_box(&self) -> &Aabb {
        &self.bbox
    }
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>, //maybe Rc
    bbox: Aabb,
}

impl HittableList {
    pub fn new_empty() -> Self {
        Self { objects: Vec::new(), bbox: Aabb::new(Interval::EMPTY, Interval::EMPTY, Interval::EMPTY) }
    }

    pub fn add/*<T: , Hittable>*/(&mut self, object: impl Hittable + 'static) {
        self.bbox = Aabb::from_2(&self.bbox, &object.boinding_box());
        self.objects.push(Box::new(object));
    }

    pub fn new(obj: Box<dyn Hittable>) -> Self {
        Self {
            bbox: obj.boinding_box().clone(), objects: vec![obj]
        }
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;
        let mut hr = HitRecord {
            p: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0, 
            front_face: true,
            mat: Rc::new(Lambertian::new(color::WHITE)),
        };

        for object in self.objects.iter() {
            if let Some(hit) = object.hit(&ray, &Interval::new(ray_t.min, closest_so_far)) {
                hit_anything = true;
                closest_so_far = hit.t;
                hr = hit;
            }
        }

        if hit_anything { Some(hr) } else {None}
    }

    fn boinding_box(&self) -> &Aabb {
        return &self.bbox;
    }
}