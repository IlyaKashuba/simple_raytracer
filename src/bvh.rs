use std::cmp::Ordering;
use std::sync::Arc;


use crate::objects::Hittable;
use crate::aabb::{Aabb};
use crate::util::Interval;



pub struct BVHNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: Aabb,
}

impl BVHNode {
    /*pub fn new(left: &'a dyn Hittable, right: &'a dyn Hittable, bbox: Aabb) -> Self {
        Self {
            left, right, bbox
        }
    }*/


    pub fn new(objects: &mut Vec<Arc<dyn Hittable>>, start: usize, end: usize) -> Self {
        
        let mut bbox = Aabb::EMPTY;
        for i in start..end {
            bbox = Aabb::from_2(&bbox, objects[i].boinding_box());
        }
        
        let left: Arc<dyn Hittable>;
        let right: Arc<dyn Hittable>;
        let axis =  bbox.longest_axis();
        let comparator = match axis {
            0 => {box_x_compare},
            1 => {box_y_compare},
            _ => {box_z_compare}
        };

        let object_span = end - start;

        if object_span == 1 {
            left = Arc::clone(&objects[start]);
            right = Arc::clone(&objects[start]);
        } else if object_span == 2 {
            left = Arc::clone(&objects[start]);
            right = Arc::clone(&objects[start+1]);
        } else {
            objects[start..end].sort_by(comparator);

            let mid = start + object_span/2;
            left = Arc::new(BVHNode::new(objects, start, mid));
            right = Arc::new(BVHNode::new(objects, start, mid));
        }
        //let left_bbox = *left.boinding_box();
        //let right_bbox = *right.boinding_box();
        Self {
            left, right, bbox/* : Aabb::from_2(&left_bbox, &right_bbox) */
        }
    }
}


impl Hittable for BVHNode {
    fn hit(&self, ray: &crate::ray::Ray, ray_t: &crate::util::Interval) -> Option<crate::objects::HitRecord> {
        if !self.bbox.hit(ray, *ray_t) {
            return None;
        }

        let hit_left = self.left.hit(ray, ray_t);
        if hit_left.is_some() {return hit_left};
        let a = if let Some(rec) = hit_left {rec.t} else {ray_t.max};
        let hit_right = self.right.hit(ray, &Interval::new(ray_t.min, a));
        return hit_right;
    }

    fn boinding_box(&self) -> &Aabb {
        return &self.bbox;
    }
}


fn box_compare<'h1, 'h2>(a: &'h1 Arc<dyn Hittable>, b: &'h2 Arc<dyn Hittable>, axis_index: usize) -> Ordering {
    let a_axis_interval = a.boinding_box().axis_interval(axis_index);
    let b_axis_interval = b.boinding_box().axis_interval(axis_index);
    return a_axis_interval.min.partial_cmp(&b_axis_interval.min).unwrap_or(Ordering::Less);
}

fn box_x_compare<'h1, 'h2>(a: &'h1 Arc<dyn Hittable>, b: &'h2 Arc<dyn Hittable>) -> Ordering {
    return box_compare(a, b, 0);
}

fn box_y_compare<'h1, 'h2>(a: &'h1 Arc<dyn Hittable>, b: &'h2 Arc<dyn Hittable>) -> Ordering {
    return box_compare(a, b, 1);
}
fn box_z_compare<'h1, 'h2>(a: &'h1 Arc<dyn Hittable>, b: &'h2 Arc<dyn Hittable>) -> Ordering {
    return box_compare(a, b, 2);
}