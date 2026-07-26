use rand::random_range;

use crate::objects::{Hittable};
use crate::color::Color;
use crate::ray::{Ray};
use crate::util::Interval;
use crate::vec3::{Point3, Vec3};
use crate::color;

//use color::{BLACK, WHITE};


pub struct Camera {
    pub aspect_ratio: f32,
    pub image_width: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    image_height: u32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_samples_scale: f32, //color scale factor for a sum of pixel samples
}

impl Camera {
    pub fn new(aspect_ratio: f32, image_width: u32) -> Self {
        Self {
            aspect_ratio, 
            image_width, 
            samples_per_pixel: 10,
            max_depth: 10,
            image_height: 0,
            center: Point3::new(0.0, 0.0, 0.0),
            pixel00_loc: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_u: Point3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Point3::new(0.0, 0.0, 0.0),
            pixel_samples_scale: 1.0,
        }
    }

    pub fn render(&mut self, world: &impl Hittable) {
        self.initialize();

        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        for i in 0..self.image_height {
            for j in 0..self.image_width {
                
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let ray: Ray = self.get_ray(j, i);
                    pixel_color += self.ray_color(&ray, self.max_depth, world);
                }
                color::write_color(&(pixel_color * self.pixel_samples_scale));
            }
        }
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f32 / self.aspect_ratio) as u32;
        self.image_height = if self.image_height < 1 {1} else {self.image_height};

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f32;

        
        //camera
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f32 / self.image_height as f32);
        let camera_center = Point3::new(0.0, 0.0, 0.0); 

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / self.image_width as f32;
        self.pixel_delta_v = viewport_v / self.image_height as f32;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = camera_center - Vec3::new(0.0, 0.0, focal_length) 
            - (viewport_u/2.0) - viewport_v/2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
        

    }

    fn ray_color(&self, ray: &Ray, depth: u32, world: &impl Hittable) -> Color {
        if depth <= 0 {
            return color::BLACK;
        }

        if let Some(hit_record) = world.hit(&ray, &Interval::new(0.001, f32::INFINITY)) {
            let direction = hit_record.normal + Vec3::random_unit_vector();
            return 0.5 * self.ray_color(&Ray::new(hit_record.p, direction), depth - 1, world);
        }

        let unit_direction = ray.dir.unit_length();
        let a = 0.5 * (unit_direction.y + 1.0);
        return (1.0 - a)*Color::new(1.0, 1.0, 1.0) + a*Color::new(0.5, 0.7, 1.0);
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc + ((i as f32 + offset.x) * self.pixel_delta_u) + ((j as f32 + offset.y) * self.pixel_delta_v);

        let ray_origin = self.center.clone();
        let ray_direction = pixel_sample - ray_origin;

        return Ray::new(ray_origin, ray_direction);
    }

    fn sample_square(&self) -> Vec3 {
        return Vec3 {
            x: random_range(-0.5..0.5),
            y: random_range(-0.5..0.5),
            z: 0.0
        };
    }
}