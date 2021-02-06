use std::ops::{Sub, Add, Div, Mul, Neg};
use rand::distributions::Standard;
use rand::distributions::Uniform;
use rand::prelude::Distribution;
use rand::Rng;
use std::iter::FromIterator;
use rand_distr::StandardNormal;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct VecN<const N: usize>([f64; N]);

impl<const N: usize> VecN<{ N }> {
    #[inline]
    pub fn dot(self, other: Self) -> f64 {
        self.0.iter().zip(other.0.iter()).map(|(a, b)| a * b).sum()
    }

    #[inline]
    pub fn length(self) -> f64 {
        self.dot(self).sqrt()
    }

    #[inline]
    pub fn unit_vec(self) -> Self {
        self / self.length()
    }

    #[inline]
    pub fn reflect(self, n: Self) -> Self {
        self - 2.0 * self.dot(n) * n
    }

    #[inline]
    pub fn refract(self, n: Self, etai_over_etat: f64) -> Self {
        let cos_theta = ((-self).dot(n)).min(1.0);
        let r_out_perp = etai_over_etat * (self + cos_theta * n);
        let r_out_parallel = -((1.0 - r_out_perp.dot(r_out_perp)).abs().sqrt()) * n;
        r_out_perp + r_out_parallel
    }

    // Return true if the vector is close to zero in all dimensions.
    #[inline]
    pub fn near_zero(self) -> bool {
        self.0.iter().map(|x| x.abs()).all(|x| x < 1e-8)
    }
 
     #[inline]
     pub fn random_unit_vec(rng: &mut impl Rng) -> Self {
        Self::from_iter(std::iter::from_fn(|| Some(rng.sample(StandardNormal))).take(N)).unit_vec()
     }
 
    #[inline]
    pub fn random_in_unit_sphere(rng: &mut impl Rng) -> Self {
        let mut p: Self = rng.gen();
        while p.dot(p) >= 1.0 {
            p = rng.gen();
        }
        p
    }
}

pub type Vec3 = VecN<3>;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self([x, y, z])
    }

    pub fn x(self) -> f64 {
        self.0[0]
    }
    pub fn y(self) -> f64 {
        self.0[1]
    }

    pub fn z(self) -> f64 {
        self.0[2]
    }

    #[inline]
    pub fn cross(self, other: Self) -> Self {
        Self::new(
            self.y()*other.z() - self.z()*other.y(),
            self.z()*other.x() - self.x()*other.z(),
            self.x()*other.y() - self.y()*other.x(),
        )
    }

}

impl<const N: usize> Distribution<VecN<{ N }>> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> VecN<{ N }> {
        let between = Uniform::new_inclusive(-1.0, 1.0);
        let mut result = [f64::default(); N];
        for rref in result.iter_mut() {
            *rref = between.sample(rng);
        }
        result.into()
    }
}

impl<const N: usize> Add for VecN<{ N }> {
    type Output = VecN<{ N }>;

    #[inline]
    fn add(self, other: Self) -> Self::Output {
        Self::from_iter(self.0.iter().zip(other.0.iter()).map(|(a, b)| a + b))
    }
}

impl<const N: usize> Sub for VecN<{ N }> {
    type Output = VecN<{ N }>;

    #[inline]
    fn sub(self, other: Self) -> Self::Output {
        Self::from_iter(self.0.iter().zip(other.0.iter()).map(|(a, b)| a - b))
    }
}

impl<const N: usize> Neg for VecN<{ N }> {
    type Output = VecN<{ N }>;

    #[inline]
    fn neg(self) -> Self::Output {
        Self::from_iter(self.0.iter().map(|a| -a))
    }
}

impl<const N: usize> Mul<f64> for VecN<{ N }> {
    type Output = VecN<{ N }>;

    #[inline]
    fn mul(self, s: f64) -> Self::Output {
        Self::from_iter(self.0.iter().map(|a| a * s))
    }
}

impl<const N: usize> Mul<VecN<{ N }>> for f64 {
    type Output = VecN<{ N }>;

    #[inline]
    fn mul(self, vec: VecN<{ N }>) -> Self::Output {
        Self::Output::from_iter(vec.0.iter().map(|a| a * self))
    }
}

impl<const N: usize> Div<f64> for VecN<{ N }> {
    type Output = VecN<{ N }>;

    #[inline]
    fn div(self, s: f64) -> Self::Output {
        Self::from_iter(self.0.iter().map(|a| a / s))
    }
}

impl<const N: usize> From<[f64; N]> for VecN<{ N }> {
    fn from(arr: [f64; N]) -> Self {
        Self(arr)
    }
}


impl<const N: usize> FromIterator<f64> for VecN<{ N }> {
    fn from_iter<I: IntoIterator<Item=f64>>(iter: I) -> Self {
        let mut result = [f64::default(); N];
        for (rref, val) in result.iter_mut().zip(iter) {
            *rref = val;
        }
        result.into()
    }
}

#[cfg(test)]
mod vec3_tests {
    use super::*;

    #[test]
    fn test_dot() {
        let vec1 = Vec3::new(0.0, 1.0, 2.0);
        let vec2 = Vec3::new(2.0, 1.0, 0.0);
        assert_eq!(vec1.dot(vec2), 1.0);
        assert_eq!(vec2.dot(vec1), 1.0);
        assert_eq!(vec2.dot(vec2), 5.0);
    }

    #[test]
    fn test_cross() {
        let vec1 = Vec3::new(0.0, 1.0, 2.0);
        let vec2 = Vec3::new(2.0, 1.0, 0.0);
        assert_eq!(vec1.cross(vec2), Vec3::new(-2.0, 4.0, -2.0));
        assert_eq!(vec2.cross(vec1), Vec3::new(2.0, -4.0, 2.0));
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
        let root5 = 5.0_f64.sqrt();
        assert_eq!(vec1.length(), root5);
        assert_eq!(vec2.length(), root5);
    }

    #[test]
    fn test_unit_vec() {
        let vec1 = Vec3::new(0.0, 1.0, 2.0);
        let vec2 = Vec3::new(2.0, 1.0, 0.0);
        let root5 = 5.0_f64.sqrt();
        let norm1 = Vec3::new(0.0, 1.0/root5, 2.0/root5);
        let norm2 = Vec3::new(2.0/root5, 1.0/root5, 0.0);
        assert_eq!(vec1.unit_vec(), norm1);
        assert_eq!(vec2.unit_vec(), norm2);
    }
}

