use crate::{util::Interval, vec3::Point3};
use crate::ray::Ray;

#[derive(Clone, Copy)]
pub struct Aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl Aabb {

    pub const EMPTY: Aabb = Aabb { x: Interval::EMPTY, y: Interval::EMPTY, z: Interval::EMPTY };
    pub const UNIVERSE: Aabb = Aabb { x: Interval::UNIVERSE, y: Interval::UNIVERSE, z: Interval::UNIVERSE };

    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    pub fn from_points(a: Point3, b: Point3) -> Self {
        Self {
            x: if a.x < b.x {Interval::new(a.x, b.x)} else {Interval::new(b.x, a.x)},
            y: if a.y < b.y {Interval::new(a.y, b.y)} else {Interval::new(b.y, a.y)},
            z: if a.z < b.z {Interval::new(a.z, b.z)} else {Interval::new(b.z, a.z)},
        }
    }

    pub fn from_2(box0: &Aabb, box1: &Aabb) -> Self {
        Self {
            x: Interval::from_2(&box0.x, &box1.x),
            y: Interval::from_2(&box0.y, &box1.y),
            z: Interval::from_2(&box0.z, &box1.z),
        }
    }

    pub fn axis_interval(&self, n: usize) -> &Interval {
        match n {
            0 => &self.x,
            1 => &self.y,
            _ => &self.z
        }
    }

    pub fn hit(&self, r: &Ray, mut ray_t: Interval) -> bool {
        
        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let adinv = 1.0 / r.dir[axis];

            let t0 = (ax.min - r.origin[axis]) * adinv;
            let t1 = (ax.max - r.origin[axis]) * adinv;

            if t0 < t1 {
                if t0 > ray_t.min { ray_t.min = t0};
                if t1 < ray_t.max { ray_t.max = t1};
            } else {
                if t1 > ray_t.min { ray_t.min = t1};
                if t0 < ray_t.max { ray_t.max = t0};
            }

            if ray_t.max <= ray_t.min { return false; }
        }

        return true;
    }

    pub fn longest_axis(&self) -> usize {
        todo!();
    }
}