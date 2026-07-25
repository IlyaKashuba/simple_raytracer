//TODO: добавить тесты

use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign, Neg};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub type Point3 = Vec3;

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x, y, z,
        }
    }

        pub fn length(&self) -> f32 {
            return self.x * self.x + self.y * self.y + self.z * self.z;
        }

        pub fn normalize(&mut self) {
           *self /= self.length(); 
        }

        pub fn unit_length(&self) -> Self {
            let length = self.length();
            Self {
                x: self.x / length,
                y: self.y / length,
                z: self.z / length,
            }
        }
    
        pub fn dot(u: &Vec3, v: &Vec3) -> f32{
            return u.x * v.x + u.y * v.y + u.z * v.z;
        }

        pub fn cross(u: &Vec3, v: &Vec3) -> Self {
            Self {
                x: u.y * v.z - u.z * v.y,
                y: u.z * v.x - u.x * v.z,
                z: u.x * v.y - u.y * v.x,
            }
        }

    }

    impl Add for Vec3 {
        type Output = Vec3;
        fn add(self, other: Self) -> Self {
            Self {
                x: self.x + other.x,
                y: self.y + other.y,
                z: self.z + other.z,
            }
        }
    }

    impl AddAssign for Vec3 {
        fn add_assign(&mut self, other: Self) {
            *self = Self { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
        }
    }

    impl Sub for Vec3{
        type Output = Vec3;
        fn sub(self, other: Self) -> Self {
            Self {
                x: self.x - other.x,
                y: self.y - other.y,
                z: self.z - other.z,
            }
        }
    }

    impl SubAssign for Vec3 {
        fn sub_assign(&mut self, other: Self) {
            *self = Self { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
        }
    }

    impl Div<f32> for Vec3 {
        type Output = Vec3;
        fn div(self, denominator: f32) -> Self {
            if denominator == 0.0 {
                panic!("Cannot divide by zero");
            }

            Self {
                x: self.x / denominator,
                y: self.y / denominator,
                z: self.z / denominator,
            }
        }
    }

    impl DivAssign<f32> for Vec3 {
        fn div_assign(&mut self, denominator: f32) {
            if denominator == 0.0 {
                panic!("Cannot divide by zero");
            }

            *self = Self {
                x: self.x / denominator,
                y: self.y / denominator,
                z: self.z / denominator,
            }
        }
    }

    impl Mul<f32> for Vec3 {
        type Output = Vec3;
        fn mul(self, rhs: f32) -> Self {
            Self {
                x: self.x * rhs,
                y: self.y * rhs,
                z: self.z * rhs,
            }
        }
    }

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Mul<Vec3> for i32 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        let n = self as f32;

        Self::Output {
            x: n * rhs.x,
            y: n * rhs.y,
            z: n * rhs.z,
        }
    }
}

    impl MulAssign<f32> for Vec3 {
        fn mul_assign(&mut self, rhs: f32) {
            *self = Self {
                x: self.x * rhs,
                y: self.y * rhs,
                z: self.z * rhs,
            }
        }
    }

    /*impl Mul<Vec3> for Vec3 {
        type Output = Vec3;
        fn mul(self, rhs: Vec3) -> Self {
            Self {
                x: self.x * rhs.x,
                y: self.y * rhs.y,
                z: self.z * rhs.y,
            }
        }
    }

    impl MulAssign<Vec3> for Vec3 {
        fn mul_assign(&mut self, rhs: Vec3) {
            *self = Self {
                x: self.x * rhs.x,
                y: self.y * rhs.y,
                z: self.z * rhs.z,
            }
        }
    }*/

    impl Index<usize> for Vec3 {
        type Output = f32;
        fn index(&self, index: usize) -> &Self::Output {

            match index {
                0 => &self.x,
                1 => &self.y,
                2 => &self.z,
                _ => panic!("index must me between 0 and 2"),
            }
        }
    }

    impl IndexMut<usize> for Vec3 {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            match index {
                0 => &mut self.x,
                1 => &mut self.y,
                2 => &mut self.z,
                _ => panic!("index must me between 0 and 2"),
            }
        }
    }

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
