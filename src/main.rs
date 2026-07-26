pub mod vec3;
pub mod color;
pub mod ray;
pub mod objects;
pub mod util;
pub mod camera;

use vec3::{Vec3, Point3};
use color::Color;
use ray::Ray;

use objects::{HittableList, Sphere, Hittable};
use util::Interval;

use crate::camera::Camera;


fn main() {

    //image
    

    // Calculate the image height, and ensure that it's at least 1.
    

    //Render

    let mut world = HittableList {objects: Vec::new()};
        
    world.objects.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));
    world.objects.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
        
    let mut cam = Camera::new(16.0 / 9.0, 400);
    cam.samples_per_pixel = 20; //100
    cam.max_depth = 50;
    cam.render(&world);
    
}
