use crate::vec3::Vec3;
use std::ops::Mul;
use crate::util::Interval;

pub type Color = Vec3;

pub const WHITE: Color = Vec3 {x: 1.0, y: 1.0, z: 1.0};
pub const BLACK: Color = Vec3 {x: 0.0, y: 0.0, z: 0.0};


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

pub fn to_rgb8(pixel_color: Color) -> [u8; 3] {
    let r = linear_to_gamma(pixel_color.x);
    let g = linear_to_gamma(pixel_color.y);
    let b = linear_to_gamma(pixel_color.z);

    let intensity = Interval::new(0.0, 0.999);

    let r = (intensity.clamp(r) * 256.0) as u8;
    let g = (intensity.clamp(g) * 256.0) as u8;
    let b = (intensity.clamp(b) * 256.0) as u8;
    //return [(pixel_color.x * 256.0) as u8, (pixel_color.y * 256.0) as u8, (pixel_color.z * 256.0) as u8];
    return [r, g, b];
}