#![feature(test)]
extern crate test;

use test::Bencher;
use glitz::vec::Vec3;
use rand_xoshiro::Xoshiro256Plus;
use rand::SeedableRng;
use rand::Rng;

#[bench]
fn random_in_unit_sphere(b: &mut Bencher) {
    let mut rng = Xoshiro256Plus::from_entropy();
    b.iter(|| {
        Vec3::random_in_unit_sphere(&mut rng);
    });
}

#[bench]
fn random_unit_vec(b: &mut Bencher) {
    let mut rng = Xoshiro256Plus::from_entropy();
    b.iter(|| {
        Vec3::random_unit_vec(&mut rng);
    });
}

#[bench]
fn dot_prod(b: &mut Bencher) {
    let mut rng = Xoshiro256Plus::from_entropy();
    let vec1: Vec3 = rng.gen();
    let vec2: Vec3 = rng.gen();
    b.iter(|| {
        vec1.dot(&vec2)
    });
}

