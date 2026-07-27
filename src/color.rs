use crate::vec3::Vec3;
use crate::util::Interval;
use std::ops::Mul;

pub type Color = Vec3;

pub const WHITE: Color = Vec3 {x: 1.0, y: 1.0, z: 1.0};
pub const BLACK: Color = Vec3 {x: 0.0, y: 0.0, z: 0.0};


pub fn write_color(pixel_color: &Color) {
    
    let r = linear_to_gamma(pixel_color.x);
    let g = linear_to_gamma(pixel_color.y);
    let b = linear_to_gamma(pixel_color.z);

    let intensity = Interval::new(0.0, 0.999);
    let r = (intensity.clamp(r) * 256.0) as i32;
    let g = (intensity.clamp(g) * 256.0) as i32;
    let b = (intensity.clamp(b) * 256.0) as i32;

    
    println!("{} {} {}", r, g, b);
}

pub fn linear_to_gamma(linear_component: f32) -> f32 {
    if linear_component > 0.0 {
        return linear_component.sqrt();
    }
    return 0.0;
}

impl Mul<Color> for Color {
    type Output = Color;
    fn mul(self, rhs: Color) -> Self::Output {
        Color {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}