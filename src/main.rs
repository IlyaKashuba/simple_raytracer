pub mod vec3;
pub mod color;
pub mod ray;
pub mod objects;
pub mod util;
pub mod camera;
pub mod material;

use rand::random_range;
use vec3::{Point3, Vec3};
use color::Color;

use objects::{HittableList, Sphere};
use std::rc::Rc;
use crate::{camera::Camera, material::{Dielectric, Lambertian, Material, Metal}};


fn main() {

    //image
    

    // Calculate the image height, and ensure that it's at least 1.
    

    //Render

    let mut world = HittableList {objects: Vec::new()};

    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.objects.push(Box::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material)));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_range(0.0..1.0);
            let center = Point3::new(a as f32 + 0.9 * random_range(0.0..1.0), 0.2, b as f32 + 0.9 * random_range(0.0..1.0));

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {

                if choose_mat < 0.8 {
                    //diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Lambertian::new(albedo);
                    world.objects.push(Box::new(Sphere::new(center, 0.2, Rc::new(sphere_material))));
                } else if choose_mat < 0.95 {
                    //metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_range(0.0..0.5);
                    let sphere_material = Metal::new(albedo, fuzz);
                    world.objects.push(Box::new(Sphere::new(center, 0.2, Rc::new(sphere_material))));
                } else {
                    //glass
                    let sphere_material = Dielectric::new(1.5);
                    world.objects.push(Box::new(Sphere::new(center, 0.2, Rc::new(sphere_material))));
                }
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.objects.push(Box::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1)));

    let material2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.objects.push(Box::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2)));

    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.objects.push(Box::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3)));



    let mut cam = Camera::new(16.0 / 9.0, 400);
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 1200;
    cam.samples_per_pixel = 10; //100
    cam.max_depth = 50;


    cam.vfov = 20.0;
    cam.look_from = Point3::new(13.0, 2.0, 3.0);
    cam.look_at = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;
    
    cam.render(&world);
    
}