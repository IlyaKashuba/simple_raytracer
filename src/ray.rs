use crate::vec3::{Vec3, Point3};

pub struct Ray {
    pub origin: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, dir: Vec3) -> Self {
        Self {
            origin, dir,
        }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        return self.origin + self.dir * t;
    }
}