pub mod vec3;
pub mod color;
pub mod ray;
pub mod objects;
pub mod util;
pub mod camera;
pub mod material;
pub mod aabb;
pub mod bvh;
pub mod texture;
pub mod perlin_noise;
pub mod quad;

use rand::random_range;
use vec3::{Point3, Vec3};
use color::Color;

use objects::{HittableList, Sphere};
use std::sync::Arc;
use crate::{camera::Camera, material::{Dielectric, DiffuseLight, Lambertian, Material, Metal}, perlin_noise::Perlin, quad::Quad, texture::Texture};


fn bouncing_spheres() {
    
    //Render

    let mut world = HittableList::new_empty();

    let ground_material: Arc<dyn Material> = Arc::new(Lambertian::from_color(Color::new(0.5, 0.5, 0.5)));
    //world.objects.push(Box::new(Sphere::new_static(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material)));
    world.add(Sphere::new_static(Point3::new(0.0, -1000.0, 0.0), 1000.0, Arc::clone(&ground_material)));
    
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_range(0.0..1.0);
            let center = Point3::new(a as f32 + 0.9 * random_range(0.0..1.0), 0.2, b as f32 + 0.9 * random_range(0.0..1.0));

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {

                if choose_mat < 0.8 {
                    //diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Lambertian::from_color(albedo);
                    let center2 = center + Vec3::new(0.0, random_range(0.0..0.5), 0.0);
                    world.add(Sphere::new_moving(center, center2, 0.2, Arc::new(sphere_material)));
                } else if choose_mat < 0.95 {
                    //metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_range(0.0..0.5);
                    let sphere_material = Metal::new(albedo, fuzz);
                    world.add(Sphere::new_static(center, 0.2, Arc::new(sphere_material)));
                } else {
                    //glass
                    let sphere_material = Dielectric::new(1.5);
                    world.add(Sphere::new_static(center, 0.2, Arc::new(sphere_material)));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Sphere::new_static(Point3::new(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = Arc::new(Lambertian::from_color(Color::new(0.4, 0.2, 0.1)));
    world.add(Sphere::new_static(Point3::new(-4.0, 1.0, 0.0), 1.0, material2));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Sphere::new_static(Point3::new(4.0, 1.0, 0.0), 1.0, material3));

    let world = HittableList::new(Arc::new(world));


    let mut cam = Camera::new(16.0 / 9.0, 400);
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400; //1200
    cam.samples_per_pixel = 10; //100
    cam.max_depth = 50;
    cam.background = Color::new(0.7, 0.8, 1.0);


    cam.vfov = 20.0;
    cam.look_from = Point3::new(13.0, 2.0, 3.0);
    cam.look_at = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;
    
    cam.render(&world);
    
}

fn checkered_spheres() {
    let mut world = HittableList::new_empty();

    let checker = Arc::new(Texture::checker_texture(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9)));
    let ground_material: Arc<dyn Material> = Arc::new(Lambertian::from_texture(checker));

    world.add(Sphere::new_static(Point3::new(0.0, -10.0, 0.0), 10.0, Arc::clone(&ground_material)));
    world.add(Sphere::new_static(Point3::new(0.0, 10.0, 0.0), 10.0, Arc::clone(&ground_material)));

    let mut cam = Camera::new(16.0 / 9.0, 400);
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400; //1200
    cam.samples_per_pixel = 10; //100
    cam.max_depth = 50;
    cam.background = Color::new(0.7, 0.8, 1.0);


    cam.vfov = 20.0;
    cam.look_from = Point3::new(13.0, 2.0, 3.0);
    cam.look_at = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;
    
    cam.render(&world);

}

fn earth() {
    let earth_texture = Arc::new(Texture::image_texture("C:/Users/1/Documents/rust_projects/simple_raytracer/assets/earthmap.jpg"));
    let earth_surface = Arc::new(Lambertian::from_texture(earth_texture));
    let globe = Sphere::new_static(Point3::new(0.0, 0.0, 0.0), 2.0, earth_surface);

    let mut cam = Camera::new(16.0 / 9.0, 400);
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400; //1200
    cam.samples_per_pixel = 10; //100
    cam.max_depth = 50;
    cam.background = Color::new(0.7, 0.8, 1.0);


    cam.vfov = 20.0;
    cam.look_from = Point3::new(13.0, 2.0, 3.0);
    cam.look_at = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    let mut world = HittableList::new_empty();
    world.add(globe);
    
    cam.render(&world);

}

fn perlin_spheres() {
    let mut world = HittableList::new_empty();

    let pertext = Arc::new(Texture::noise_texture(Perlin::new(), 4.0));
    let mat: Arc<dyn Material> = Arc::new(Lambertian::from_texture(pertext));
    world.add(Sphere::new_static(Point3::new(0.0, -1000.0, 0.0), 1000.0, Arc::clone(&mat)));
    world.add(Sphere::new_static(Point3::new(0.0, 2.0, 0.0), 2.0, Arc::clone(&mat)));
    
    


    let mut cam = Camera::new(16.0 / 9.0, 400);
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400; //1200
    cam.samples_per_pixel = 10; //100
    cam.max_depth = 50;
    cam.background = Color::new(0.7, 0.8, 1.0);


    cam.vfov = 20.0;
    cam.look_from = Point3::new(13.0, 2.0, 3.0);
    cam.look_at = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;
    
    cam.render(&world);
}

fn quads() {
    let mut world = HittableList::new_empty();

    let left_red = Arc::new(Lambertian::from_color(Color::new(1.0, 0.2, 0.2)));
    let back_green = Arc::new(Lambertian::from_color(Color::new(0.2, 1.0, 0.2)));
    let right_blue = Arc::new(Lambertian::from_color(Color::new(0.2, 0.2, 1.0)));
    let upper_orange = Arc::new(Lambertian::from_color(Color::new(1.0, 0.5, 0.0)));
    let lower_teal = Arc::new(Lambertian::from_color(Color::new(0.2, 0.8, 0.8)));

    world.add(Quad::new(Point3::from_i32s(-3, -2, 5), Vec3::from_i32s(0, 0, -4), Vec3::from_i32s(0, 4, 0), left_red));
    world.add(Quad::new(Point3::from_i32s(-2, -2, 0), Vec3::from_i32s(4, 0, 0), Vec3::from_i32s(0, 4, 0), back_green));
    world.add(Quad::new(Point3::from_i32s(3, -2, 1), Vec3::from_i32s(0, 0, 4), Vec3::from_i32s(0, 4, 0), right_blue));
    world.add(Quad::new(Point3::from_i32s(-2, 3, 1), Vec3::from_i32s(4, 0, 0), Vec3::from_i32s(0, 0, 4), upper_orange));
    world.add(Quad::new(Point3::from_i32s(-2, -3, 5), Vec3::from_i32s(4, 0, 0), Vec3::from_i32s(0, 0, -4), lower_teal));


    let mut cam = Camera::new(16.0 / 9.0, 400);
    cam.aspect_ratio = 1.0;
    cam.image_width = 400; //1200
    cam.samples_per_pixel = 10; //100
    cam.max_depth = 50;
    cam.background = Color::new(0.7, 0.8, 1.0);


    cam.vfov = 80.0;
    cam.look_from = Point3::from_i32s(0, 0, 9);
    cam.look_at = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;
    
    cam.render(&world);
}

fn simple_light() {
    let mut world = HittableList::new_empty();

    let pertext = Arc::new(Texture::noise_texture(Perlin::new(), 4.0));
    let mat: Arc<dyn Material> = Arc::new(Lambertian::from_texture(pertext));
    world.add(Sphere::new_static(Point3::new(0.0, -1000.0, 0.0), 1000.0, Arc::clone(&mat)));
    world.add(Sphere::new_static(Point3::new(0.0, 2.0, 0.0), 2.0, Arc::clone(&mat)));
    
    let diff_light: Arc<dyn Material> = Arc::new(DiffuseLight::new(Arc::new(Texture::solid_color(Color::new(1.0, 1.0, 1.0)))));
    world.add(Sphere::new_static(Point3::new(0.0, 7.0, 0.0), 2.0, Arc::clone(&diff_light)));
    world.add(Quad::new(Point3::from_i32s(3, 1, -2), Vec3::from_i32s(2, 0, 0), Vec3::from_i32s(0, 2, 0), Arc::clone(&diff_light)));
    


    let mut cam = Camera::new(16.0 / 9.0, 400);
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400; //1200
    cam.samples_per_pixel = 30; //100
    cam.max_depth = 50;
    cam.background = color::BLACK;


    cam.vfov = 20.0;
    cam.look_from = Point3::new(26.0, 3.0, 6.0);
    cam.look_at = Point3::new(0.0, 2.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;
    
    cam.render(&world);
}

fn cornell_box() {
    let mut world = HittableList::new_empty();

    let red: Arc<dyn Material> = Arc::new(Lambertian::from_color(Color::new(0.65, 0.05, 0.05)));
    let white: Arc<dyn Material> = Arc::new(Lambertian::from_color(Color::new(0.73, 0.73, 0.73)));
    let green: Arc<dyn Material> = Arc::new(Lambertian::from_color(Color::new(0.12, 0.45, 0.15)));
    let light: Arc<dyn Material> = Arc::new(Lambertian::from_color(Color::new(15.0, 15.0, 15.0)));
    

    /*world.add(Quad::new(Point3::from_i32s(555, 0, 0), Vec3::from_i32s(0, 555, 0), Vec3::from_i32s(0, 0, 555), green));
    world.add(Quad::new(Point3::from_i32s(0, 0, 0), Vec3::from_i32s(0, 555, 0), Vec3::from_i32s(0, 0, 555), red));
    world.add(Quad::new(Point3::from_i32s(343, 554, 332), Vec3::from_i32s(-130, 0, 0), Vec3::from_i32s(0, 0, -105), light));
    world.add(Quad::new(Point3::from_i32s(0, 0, 0), Vec3::from_i32s(555, 0, 0), Vec3::from_i32s(0, 0, 555), Arc::clone(&white)));
    world.add(Quad::new(Point3::from_i32s(555, 555, 555), Vec3::from_i32s(555, 555, 5550), Vec3::from_i32s(-555, 0, -555), Arc::clone(&white)));
    world.add(Quad::new(Point3::from_i32s(0, 0, 555), Vec3::from_i32s(555, 0, 0), Vec3::from_i32s(0, 555, 0), Arc::clone(&white)));
    */

    /*let diff_light: Arc<dyn Material> = Arc::new(DiffuseLight::new(
        Arc::new(Texture::solid_color(Color::new(20.0, 20.0, 20.0)))
    ));
    world.add(Sphere::new_static(Point3::new(278.0, 278.0, 0.0), 20.0, Arc::clone(&diff_light)));
    */

    let mut cam = Camera::new(16.0 / 9.0, 400);
    cam.aspect_ratio = 1.0;
    cam.image_width = 600; //1200
    cam.samples_per_pixel = 10; //100
    cam.max_depth = 50;
    cam.background = Color::new(0.0, 0.0, 0.0);


    cam.vfov = 40.0;
    cam.look_from = Point3::from_i32s(278, 278, -800);
    cam.look_at = Point3::from_i32s(278, 278, 0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;
    
    cam.render(&world);
}

pub fn main() {
    match 6 {
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        4 => perlin_spheres(),
        5 => quads(),
        6 => simple_light(),
        7 => cornell_box(),
        _ => bouncing_spheres(),
    };
    
}