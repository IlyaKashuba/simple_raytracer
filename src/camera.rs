use rand::random_range;
use image::{Rgb};
use rayon::prelude::*;
//use std::sync::

use crate::objects::{Hittable};
use crate::color::Color;
use crate::ray::{Ray};
use crate::util::{self, Interval};
use crate::vec3::{Point3, Vec3};
use crate::color;


pub struct Camera {
    pub aspect_ratio: f32,
    pub image_width: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub vfov: f32,
    pub look_from: Point3,
    pub look_at: Point3,
    pub vup: Vec3,
    pub defocus_angle: f32,
    pub focus_dist: f32,
    image_height: u32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_samples_scale: f32, //color scale factor for a sum of pixel samples
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f32, image_width: u32) -> Self {
        Self {
            aspect_ratio, 
            image_width, 
            samples_per_pixel: 10,
            max_depth: 10,
            vfov: 90.0,
            look_from: Point3::new(0.0, 0.0, 0.0),
            look_at: Point3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 100.0,
            image_height: 0,
            center: Point3::new(0.0, 0.0, 0.0),
            pixel00_loc: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_u: Point3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Point3::new(0.0, 0.0, 0.0),
            pixel_samples_scale: 1.0,
            v: Vec3::new(0.0, 0.0, 0.0),
            u: Vec3::new(0.0, 0.0, 0.0),
            w: Vec3::new(0.0, 0.0, 0.0),
            defocus_disk_u: Vec3::new(0.0, 0.0, 0.0),
            defocus_disk_v: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn render(&mut self, world: &(impl Hittable + Sync + Send)) {
        self.initialize();


        let row_pixels: Vec<Vec<u8>> = (0..self.image_height).into_par_iter().map(|y| {
            let mut row = vec![0u8; (self.image_width * 3) as usize];

            for x in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let ray: Ray = self.get_ray(x, y);
                    pixel_color += self.ray_color(&ray, self.max_depth, world);
                }
                let rgb = color::to_rgb8(pixel_color * self.pixel_samples_scale);

                let idx = (x * 3) as usize;
                row[idx] = rgb[0];
                row[idx+1] = rgb[1];
                row[idx+2] = rgb[2];
            } 
            row
        }).collect();

        let pixels: Vec<u8> = row_pixels.into_iter().flatten().collect();

        /*let mut imgbuf = image::ImageBuffer::new(self.image_width, self.image_height);
        

        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {

            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let ray: Ray = self.get_ray(x, y);
                    pixel_color += self.ray_color(&ray, self.max_depth, world);
                }

            *pixel = image::Rgb(color::to_rgb8(pixel_color * self.pixel_samples_scale));
        }
        imgbuf.save("image.png").unwrap();*/
        
        let imgbuf: image::ImageBuffer<Rgb<u8>, Vec<u8>> = image::ImageBuffer::from_raw(self.image_width, self.image_height, pixels).unwrap();
        imgbuf.save("image.png").unwrap();
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f32 / self.aspect_ratio) as u32;
        self.image_height = if self.image_height < 1 {1} else {self.image_height};

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f32;

        self.center = self.look_from;
        
        //camera
        
        let theta = util::degrees_to_radians(self.vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f32 / self.image_height as f32);
        
        self.w = (self.look_from - self.look_at).unit_length();
        self.u = Vec3::cross(&self.vup, &self.w).unit_length();
        self.v = Vec3::cross(&self.w, &self.u);


        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * -self.v;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / self.image_width as f32;
        self.pixel_delta_v = viewport_v / self.image_height as f32;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = self.center - self.focus_dist * self.w - viewport_u / 2.0 - viewport_v / 2.0;            
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
        
        let defocus_radius = self.focus_dist * util::degrees_to_radians(self.defocus_angle / 2.0).tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;

    }

    fn ray_color(&self, ray: &Ray, depth: u32, world: &impl Hittable) -> Color {
        if depth <= 0 {
            return color::BLACK;
        }

        if let Some(hit_rec) = world.hit(&ray, &Interval::new(0.001, f32::INFINITY)) {
            if let Some((scattered, attenuation)) = hit_rec.mat.scatter(ray, &hit_rec) {
                return attenuation * self.ray_color(&scattered, depth-1, world);
            }
            return color::BLACK;
        }

        let unit_direction = ray.dir.unit_length();
        let a = 0.5 * (unit_direction.y + 1.0);
        return (1.0 - a)*Color::new(1.0, 1.0, 1.0) + a*Color::new(0.5, 0.7, 1.0);
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc + ((i as f32 + offset.x) * self.pixel_delta_u) + ((j as f32 + offset.y) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0.0 {self.center} else {self.defocus_disk_sample()};
        let ray_direction = pixel_sample - ray_origin;
        let ray_time = random_range(0.0..1.0);

        return Ray::new(ray_origin, ray_direction, ray_time);
    }

    fn sample_square(&self) -> Vec3 {
        return Vec3 {
            x: random_range(-0.5..0.5),
            y: random_range(-0.5..0.5),
            z: 0.0
        };
    }

    fn defocus_disk_sample(&self) -> Point3 {
        //returns a random oint in the camera defocus disk
        let p = Vec3::random_in_unit_disk();
        return self.center + (p[0] * self.defocus_disk_u) + (p[1] * self.defocus_disk_v)
    }

}