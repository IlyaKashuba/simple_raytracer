use crate::vec3::Vec3;
use std::ops::Mul;

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