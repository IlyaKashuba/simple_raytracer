use rand::{random, random_range};
use crate::Point3;

const POINT_COUNT: usize = 256;

pub struct Perlin {
    randfloat: Vec<f32>,
    perm_x: Vec<u32>,
    perm_y: Vec<u32>,
    perm_z: Vec<u32>,
}

impl Perlin {
    fn permute(p: &mut Vec<u32>) {
        for i in (0..POINT_COUNT-1).rev() {
            let target = random_range(0..=i);
            p.swap(i, target);
        }
    }

    fn perlin_generate_perm() -> Vec<u32> {
        let mut arr = Vec::<u32>::with_capacity(POINT_COUNT);
        for i in 0..POINT_COUNT {
            arr.push(i as u32);
        }
        Self::permute(&mut arr);

        return arr;
    }

    pub fn new() -> Self {
        let mut randfloat = Vec::<f32>::with_capacity(POINT_COUNT);
        for _ in 0..POINT_COUNT {
            randfloat.push(random_range(0.0..1.0));
        }

        Self {
            randfloat, 
            perm_x: Self::perlin_generate_perm(),
            perm_y: Self::perlin_generate_perm(),
            perm_z: Self::perlin_generate_perm(),
        }
    }

    pub fn noise(&self, p: &Point3) -> f32 {
        let i = (32.0 * p.x) as usize & 255;
        let j = (32.0 * p.y) as usize & 255;
        let k = (32.0 * p.z) as usize & 255;

        return self.randfloat[(self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]) as usize];
    }
}