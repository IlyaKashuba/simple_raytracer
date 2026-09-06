use std::f32::{consts::PI};
use std::ops::Add;


pub fn degrees_to_radians(degrees: f32) -> f32 {
    return degrees * PI / 180.0;
}

#[derive(Clone, Copy)]
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub const EMPTY: Interval = Interval {min: f32::INFINITY, max: -f32::INFINITY};
    pub const UNIVERSE: Interval = Interval {min: -f32::INFINITY, max: f32::INFINITY};

    pub fn new(min: f32, max: f32) -> Self {
        Self {min, max}
    }

    pub fn from_2(a: &Interval, b: &Interval) -> Self {
        Self {
            min: a.min.min(b.min),
            max: a.max.max(b.max),
        }
    }

    pub fn contains(&self, x: f32) -> bool {
        return self.min <= x && x <= self.max;
    }

    pub fn surrounds(&self, x: f32) -> bool {
        return self.min < x && x < self.max;
    }

    pub fn size(&self) -> f32 {
        return self.max - self.min;
    }

    pub fn clamp(&self, x: f32) -> f32 {
        if x < self.min {
            return self.min; 
        } else if x > self.max {
            return self.max; 
        } else {
            return x;
        }
    }

    pub fn expand(&self, delta: f32) -> Self {
        let padding = delta / 2.0;
        return Interval::new(self.min - padding, self.max + padding);
    }

}

impl Add<f32> for Interval {
    type Output = Interval;
    fn add(self, rhs: f32) -> Self::Output {
        Interval {
            min: self.min + rhs,
            max: self.max + rhs,
        }
    }
}