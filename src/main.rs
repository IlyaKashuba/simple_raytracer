pub mod vec3;
pub mod color;
pub mod ray;
pub mod objects;
pub mod util;
pub mod camera;
pub mod material;

use vec3::{Point3};
use color::Color;

use objects::{HittableList, Sphere};
use std::rc::Rc;
use crate::{camera::Camera, material::{Dielectric, Lambertian, Metal}};


fn main() {

    //image
    

    // Calculate the image height, and ensure that it's at least 1.
    

    //Render

    let mut world = HittableList {objects: Vec::new()};


    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_bubble = Rc::new(Dielectric::new(1.0/1.5));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.3));

        
    world.objects.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    world.objects.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.2), 0.5, material_center)));
    world.objects.push(Box::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left))); 
    world.objects.push(Box::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.4, material_bubble))); 
    world.objects.push(Box::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right))); 



    let mut cam = Camera::new(16.0 / 9.0, 400);
    cam.samples_per_pixel = 20; //100
    cam.max_depth = 50;
    cam.render(&world);
    
}
