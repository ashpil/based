#![feature(test)]
extern crate test;

use test::Bencher;
use glitz::vec::Vec3;
use rand_xoshiro::Xoshiro256Plus;
use rand::SeedableRng;
use rand::Rng;

#[bench]
fn dot_prod(b: &mut Bencher) {
    let mut rng = Xoshiro256Plus::from_entropy();
    let mut vecs1: Vec<Vec3> = Vec::default();
    let mut vecs2: Vec<Vec3> = Vec::default();
    for _ in 0..10 {
        vecs1.push(rng.gen());
        vecs2.push(rng.gen());
    }
    let zipped = vecs1.into_iter().zip(vecs2.into_iter());
    b.iter(|| {
        zipped.clone().map(|(a, b)| a.dot(b))
    });
}

#[bench]
fn add(b: &mut Bencher) {
    let mut rng = Xoshiro256Plus::from_entropy();
    let mut vecs1: Vec<Vec3> = Vec::default();
    let mut vecs2: Vec<Vec3> = Vec::default();
    for _ in 0..10 {
        vecs1.push(rng.gen());
        vecs2.push(rng.gen());
    }
    let zipped = vecs1.into_iter().zip(vecs2.into_iter());
    b.iter(|| {
        zipped.clone().map(|(a, b)| a + b)
    });
}

#[bench]
fn sub(b: &mut Bencher) {
    let mut rng = Xoshiro256Plus::from_entropy();
    let mut vecs1: Vec<Vec3> = Vec::default();
    let mut vecs2: Vec<Vec3> = Vec::default();
    for _ in 0..10 {
        vecs1.push(rng.gen());
        vecs2.push(rng.gen());
    }
    let zipped = vecs1.into_iter().zip(vecs2.into_iter());
    b.iter(|| {
        zipped.clone().map(|(a, b)| a - b)
    });
}

#[bench]
fn mul(b: &mut Bencher) {
    let mut rng = Xoshiro256Plus::from_entropy();
    let mut vecs1: Vec<Vec3> = Vec::default();
    let mut vecs2: Vec<f64> = Vec::default();
    for _ in 0..10 {
        vecs1.push(rng.gen());
        vecs2.push(rng.gen());
    }
    let zipped = vecs1.into_iter().zip(vecs2.into_iter());
    b.iter(|| {
        zipped.clone().map(|(a, b)| a * b)
    });
}

#[bench]
fn div(b: &mut Bencher) {
    let mut rng = Xoshiro256Plus::from_entropy();
    let mut vecs1: Vec<Vec3> = Vec::default();
    let mut vecs2: Vec<f64> = Vec::default();
    for _ in 0..10 {
        vecs1.push(rng.gen());
        vecs2.push(rng.gen());
    }
    let zipped = vecs1.into_iter().zip(vecs2.into_iter());
    b.iter(|| {
        zipped.clone().map(|(a, b)| a / b)
    });
}

#[bench]
fn length(b: &mut Bencher) {
    let mut rng = Xoshiro256Plus::from_entropy();
    let mut vecs1: Vec<Vec3> = Vec::default();
    for _ in 0..10 {
        vecs1.push(rng.gen());
    }
    let iter = vecs1.into_iter();
    b.iter(|| {
        iter.clone().map(|a| a.length())
    });
}

#[bench]
fn unit(b: &mut Bencher) {
    let mut rng = Xoshiro256Plus::from_entropy();
    let mut vecs1: Vec<Vec3> = Vec::default();
    for _ in 0..10 {
        vecs1.push(rng.gen());
    }
    let iter = vecs1.into_iter();
    b.iter(|| {
        iter.clone().map(|a| a.unit_vec())
    });
}

#[bench]
fn cross(b: &mut Bencher) {
    let mut rng = Xoshiro256Plus::from_entropy();
    let mut vecs1: Vec<Vec3> = Vec::default();
    let mut vecs2: Vec<Vec3> = Vec::default();
    for _ in 0..10 {
        vecs1.push(rng.gen());
        vecs2.push(rng.gen());
    }
    let zipped = vecs1.into_iter().zip(vecs2.into_iter());
    b.iter(|| {
        zipped.clone().map(|(a, b)| a.cross(b))
    });
}

