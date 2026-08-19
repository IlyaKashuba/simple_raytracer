use std::fmt::format;
use std::fs::File;
use std::io::Write;
use std::str::Bytes;
use std::vec;

use crate::color::{self, BLACK, Color, linear_to_gamma};
use crate::util::Interval;

pub enum FileFormat {
    PPM,
    //PNG,
}

pub struct Image {
    pub name: String,
    file_contents: String,
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Color>,
    file_format: FileFormat,
    
}

impl Image {

    pub fn create(name: String, width: u32, height: u32, file_format: FileFormat) -> Self {
        let file_contents = match file_format {
            FileFormat::PPM => format!("P3\n{} {}\n255\n", width, height),
            //FileFormat::PNG => ,
            _ => String::from(""),
        };
        let pixels = vec![color::BLACK; (width*height) as usize];
        Self {
            name, file_contents, width, height, file_format, pixels,
        }
    }

    pub fn write_color(&mut self, x: u32, y: u32, pixel_color: &Color) {
    
        let r = linear_to_gamma(pixel_color.x);
        let g = linear_to_gamma(pixel_color.y);
        let b = linear_to_gamma(pixel_color.z);

        let intensity = Interval::new(0.0, 0.999);
        /*let r = (intensity.clamp(r) * 256.0) as i32;
        let g = (intensity.clamp(g) * 256.0) as i32;
        let b = (intensity.clamp(b) * 256.0) as i32;*/

        let r = intensity.clamp(r);
        let g = intensity.clamp(g);
        let b = intensity.clamp(b);


        
        //self.contents += format!("{r} {g} {b}\n").as_str();
        self.pixels[(y * self.width + x) as usize] = Color::new(r, g, b);
    }

    pub fn save(&mut self) {
        let mut file = File::create(&self.name).unwrap();

        //PPM only for now
        for y in 0..self.height {
            for x in 0..self.width {
                let pixel_color = self.pixels[(y * self.width + x) as usize];
                let r = (pixel_color.x * 256.0) as i32;
                let g = (pixel_color.y * 256.0) as i32;
                let b = (pixel_color.z * 256.0) as i32;

                self.file_contents += format!("{r} {g} {b}\n").as_str();
            }
        }

        let _ = file.write_all(self.file_contents.as_bytes());
    }
}
