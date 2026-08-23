use rand::random_range;
use crate::Point3;
use crate::Vec3;

const POINT_COUNT: usize = 256;

pub struct Perlin {
    randvec: Vec<Vec3>,
    perm_x: Vec<u32>,
    perm_y: Vec<u32>,
    perm_z: Vec<u32>,
}

impl Perlin {
    fn permute(p: &mut Vec<u32>) {
        for i in (0..POINT_COUNT).rev() {
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
        let mut randvec = Vec::<Vec3>::with_capacity(POINT_COUNT);
        for _ in 0..POINT_COUNT {
            randvec.push(Vec3::random_range(-1.0, 1.0).unit_length());
        }

        //let perm_x = Self::perlin_generate_perm();
        //slet

        Self {
            randvec, 
            perm_x: Self::perlin_generate_perm(),
            perm_y: Self::perlin_generate_perm(),
            perm_z: Self::perlin_generate_perm(),
        }
    }

    pub fn noise(&self, p: &Point3) -> f32 {

        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;

        let mut c = [[[Vec3::ZERO; 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di as usize][dj as usize][dk as usize] = self.randvec[
                        (self.perm_x[((i+di) & 255) as usize] ^ 
                        self.perm_y[((j+dj) & 255) as usize] ^
                        self.perm_z[((k+dk) & 255) as usize]) as usize
                    ];
                }
            }
        }

        return Self::perlin_interp(&c, u, v, w);
    }

    pub fn debug() {
        let arr1 = Self::perlin_generate_perm();
        let arr2 = Self::perlin_generate_perm();
        println!("{:?}", arr1);
        println!("{:?}", arr2);
    }

    pub fn turb(&self, p: &Point3, depth: usize) -> f32 {
        let mut accum = 0.0;
        let mut temp_p = p.clone();
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * Self::noise(&self, &temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        return accum.abs();
    }

    fn perlin_interp(c: &[[[Vec3; 2]; 2]; 2], u: f32, v: f32, w:f32) -> f32{
        let uu = u*u*(3.0-2.0*u);
        let vv = v*v*(3.0-2.0*v);
        let ww = w*w*(3.0-2.0*w);

        let mut accum = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3::new(u - i as f32, v - j as f32, w - k as f32);
                    accum += (i as f32 * uu + (1-i) as f32 * (1.0-uu))
                        * (j as f32 * vv + (1-j) as f32 * (1.0-vv))
                        * (k as f32 * ww + (1-k) as f32 * (1.0-ww))
                        * Vec3::dot(&c[i][j][k], &weight_v);
                }
            }
        }

        return accum;
    }
}