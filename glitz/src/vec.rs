use std::ops::{Sub, Add, Div, Mul, Neg};
use rand::distributions::Standard;
use rand::distributions::Uniform;
use rand::prelude::Distribution;
use rand::Rng;
use rand_distr::StandardNormal;

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    #[inline]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    #[inline]
    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    #[inline]
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    #[inline]
    pub fn unit_vec(&self) -> Self {
        *self / self.length()
    }

    #[inline]
    pub fn cross(&self, other: &Self) -> Self {
        Self::new(
            self.y*other.z - self.z*other.y,
            self.z*other.x - self.x*other.z,
            self.x*other.y - self.y*other.x,
        )
    }

    #[inline]
    pub fn reflect(&self, n: &Self) -> Self {
        *self - 2.0 * self.dot(n) * *n
    }

    #[inline]
    pub fn refract(&self, n: &Vec3, etai_over_etat: f32) -> Vec3 {
        let cos_theta = ((-*self).dot(&n)).min(1.0);
        let r_out_perp = etai_over_etat * (*self + cos_theta * *n);
        let r_out_parallel = -((1.0 - r_out_perp.dot(&r_out_perp)).abs().sqrt()) * *n;
        r_out_perp + r_out_parallel
    }

    // Return true if the vector is close to zero in all dimensions.
    #[inline]
    pub fn near_zero(&self) -> bool {
        let e = 1e-8;
        (self.x.abs() < e) && (self.y.abs() < e) && (self.z.abs() < e)
    }

    #[inline]
    pub fn random_unit_vec(rng: &mut impl Rng) -> Self {
        Vec3::new(rng.sample(StandardNormal), rng.sample(StandardNormal), rng.sample(StandardNormal)).unit_vec()
    }

    #[inline]
    pub fn random_in_unit_sphere(rng: &mut impl Rng) -> Self {
        let mut p: Vec3 = rng.gen();
        while p.dot(&p) >= 1.0 {
            p = rng.gen();
        }
        p
    }
}

impl Distribution<Vec3> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec3 {
        let between = Uniform::new_inclusive(-1.0, 1.0);
        Vec3::new(between.sample(rng), between.sample(rng), between.sample(rng)) 
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    #[inline]
    fn add(self, other: Self) -> Self::Output {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    #[inline]
    fn sub(self, other: Self) -> Self::Output {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    #[inline]
    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, s: f32) -> Self::Output {
        Vec3 {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    #[inline]
    fn mul(self, vec: Vec3) -> Self::Output {
        Vec3 {
            x: vec.x * self,
            y: vec.y * self,
            z: vec.z * self,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn div(self, s: f32) -> Self::Output {
        Vec3 {
            x: self.x / s,
            y: self.y / s,
            z: self.z / s,
        }
    }
}

impl From<[f32; 3]> for Vec3 {
    fn from(arr: [f32; 3]) -> Self {
        Vec3 {
            x: arr[0],
            y: arr[1],
            z: arr[2],
        }
    }
}

#[cfg(test)]
mod vec3_tests {
    use super::*;

    #[test]
    fn test_dot() {
        let vec1 = Vec3::new(0.0, 1.0, 2.0);
        let vec2 = Vec3::new(2.0, 1.0, 0.0);
        assert_eq!(vec1.dot(&vec2), 1.0);
        assert_eq!(vec2.dot(&vec1), 1.0);
        assert_eq!(vec2.dot(&vec2), 5.0);
    }

    #[test]
    fn test_cross() {
        let vec1 = Vec3::new(0.0, 1.0, 2.0);
        let vec2 = Vec3::new(2.0, 1.0, 0.0);
        assert_eq!(vec1.cross(&vec2), Vec3::new(-2.0, 4.0, -2.0));
        assert_eq!(vec2.cross(&vec1), Vec3::new(2.0, -4.0, 2.0));
    }

    #[test]
    fn test_sub() {
        let vec1 = Vec3::new(0.0, 1.0, 2.0);
        let vec2 = Vec3::new(2.0, 1.0, 0.0);
        assert_eq!(vec1 - vec2, Vec3::new(-2.0, 0.0, 2.0));
        assert_eq!(vec2 - vec1, Vec3::new(2.0, 0.0, -2.0));
    }

    #[test]
    fn test_add() {
        let vec1 = Vec3::new(0.0, 1.0, 2.0);
        let vec2 = Vec3::new(2.0, 1.0, 0.0);
        assert_eq!(vec1 + vec2, Vec3::new(2.0, 2.0, 2.0));
        assert_eq!(vec2 + vec1, Vec3::new(2.0, 2.0, 2.0));
    }

    #[test]
    fn test_mul() {
        let vec = Vec3::new(0.0, 1.0, 2.0);
        assert_eq!(vec * 5.0, Vec3::new(0.0, 5.0, 10.0));
        assert_eq!(5.0 * vec, Vec3::new(0.0, 5.0, 10.0));
    }

    #[test]
    fn test_length() {
        let vec1 = Vec3::new(0.0, 1.0, 2.0);
        let vec2 = Vec3::new(2.0, 1.0, 0.0);
        let root5 = 5.0_f32.sqrt();
        assert_eq!(vec1.length(), root5);
        assert_eq!(vec2.length(), root5);
    }

    #[test]
    fn test_unit_vec() {
        let vec1 = Vec3::new(0.0, 1.0, 2.0);
        let vec2 = Vec3::new(2.0, 1.0, 0.0);
        let root5 = 5.0_f32.sqrt();
        let norm1 = Vec3::new(0.0, 1.0/root5, 2.0/root5);
        let norm2 = Vec3::new(2.0/root5, 1.0/root5, 0.0);
        assert_eq!(vec1.unit_vec(), norm1);
        assert_eq!(vec2.unit_vec(), norm2);
    }
}

