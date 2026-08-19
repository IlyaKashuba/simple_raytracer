use crate::{color::Color, util::Interval};
use crate::vec3::Point3;
use std::{ rc::Rc};
use image::{DynamicImage, GenericImageView, Rgba};

#[derive(Clone, Copy, Debug)]
pub struct TexCoords(pub f32, pub f32);

pub enum Texture {
    SolidColorTerxture {albedo: Color},
    CheckerTexture {inv_scale: f32, even: Rc<Texture>, odd: Rc<Texture>},
    ImageTexture {img: DynamicImage},
}

impl Texture {
    pub fn value(&self, tex_coords: TexCoords, p: &Point3) -> (TexCoords, Color) {
        match self {
            Texture::SolidColorTerxture {albedo} => {
                return (TexCoords(0.0, 0.0), *albedo);
            }
            Texture::CheckerTexture { inv_scale, even, odd } => {
                let x_int = (inv_scale * p.x).floor() as i32;
                let y_int = (inv_scale * p.y).floor() as i32;
                let z_int = (inv_scale * p.z).floor() as i32;

                let is_even = (x_int + y_int + z_int) % 2 == 0;

                if is_even { 
                    return even.value(tex_coords, p);
                } else {
                    return odd.value(tex_coords, p);
                }
            }
            Texture::ImageTexture { img } => {
                if img.height() <= 0 { return (TexCoords(0.0, 0.0), Color::new(0.1, 1.0, 1.0));}

                let u = Interval::new(0.0, 1.0).clamp(tex_coords.0);
                let v = Interval::new(0.0, 1.0).clamp(tex_coords.1);

                let i = (img.width() as f32 * u) as u32;
                let j = (img.width() as f32 * v) as u32;
                let pixel = get_pixel_at_xy(&img, i, j);
                //let image::Rgb(data) = *pixel;
                let color = Color::new(pixel[0] as f32 / 255.0, pixel[1] as f32 / 255.0, pixel[2] as f32 / 255.0);
                return (TexCoords(0.0, 0.0), color);
            }
        }
    }

    pub fn solid_color(albedo: Color) -> Texture {
        Self::SolidColorTerxture { albedo }
    }

    pub fn checker_texture(scale: f32, c1: Color, c2: Color) -> Texture {
        Self::CheckerTexture { 
            inv_scale: scale, 
            even: Rc::new(Texture::SolidColorTerxture { albedo: c1 }), 
            odd: Rc::new(Texture::SolidColorTerxture { albedo: c2 }) 
        }
    }

    pub fn image_texture(filename: &str) -> Self {
        let img = image::open(filename).unwrap();
        let texture_image = img.flipv();
        Texture::ImageTexture { img: texture_image }
    }

}

/*pub struct SolidColorTerxture {
    pub albedo: Color
}*/

fn clamp(x: u32, low: u32, high: u32) -> u32 {
    if x < low { 
        return low;
    } else if x >= high {
        return high - 1;
    } else {
        return x;
    }
}
pub fn get_pixel_at_xy(img: &DynamicImage, x: u32, y: u32) -> Rgba<u8> {
    let i = clamp(x, 0, img.width());
    let j = clamp(y, 0, img.height());

    return img.get_pixel(i, j);
}