use std::fs::File;
use xenon::color::Color;
use xenon::write::fn_to_png;
use syzygy::random::with_rng;
use syzygy::ray::Ray;
use syzygy::camera::SimpleCamera;
use syzygy::camera::Camera;
use syzygy::hittable::{Hittables, Sphere, Hittable};
use syzygy::material::{Metal, Lambertian, Dielectric};
use glitz::vec::Vec3;
use std::rc::Rc;
use antsy::LoadingBar;

fn ray_color(r: Ray, to_hit: &impl Hittable, depth: u16) -> Color {
    if depth <= 0 {
        Color::new(0.0, 0.0, 0.0)
    } else {
        if let Some(hit) = to_hit.intersect(&r, 0.00001, f64::INFINITY) {
            if let Some((scattered_ray, atten)) = hit.mat.scatter(&hit, r) {
                atten * ray_color(scattered_ray, to_hit, depth - 1)
            } else {
                Color::new(0.0, 0.0, 0.0)
            }
        } else {
            let unit_dir = r.d.unit_vec();
            let t = 0.5 * (unit_dir.y + 1.0);
            (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
        }
    }
}

fn main() {
    // Image
    const NUM_SAMPLES: u16 = 1000;
    const MAX_DEPTH: u16 = 50;

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 2000;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    // World
    let mat_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let mat_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let mat_left = Rc::new(Dielectric::new(1.5));
    let mat_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));
    let mut world = Hittables::new();
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, mat_ground)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, mat_center)));
    world.add(Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, mat_left.clone())));
    world.add(Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.45, mat_left.clone())));
    world.add(Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, mat_right)));

    // Camera
    let lookfrom = Vec3::new(3.0, 3.0, 2.0);
    let lookat = Vec3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (lookfrom - lookat).length();
    let aperture = 0.5;
    let cam = SimpleCamera::new(lookfrom, lookat, vup, 20.0, aspect_ratio, aperture, dist_to_focus);
    
    // File
    let file = File::create("out.png").unwrap();
    let mut loadingbar = LoadingBar::new(image_height as u16);

    fn_to_png(image_width, image_height, file, |i, j| {
        let mut color = Color::new(0.0, 0.0, 0.0);
        loadingbar.update((image_height - j) as u16);
        for _ in 0..=NUM_SAMPLES {
            let u = (i as f64 + with_rng(rand::Rng::gen::<f64>)) / (image_width - 1) as f64;
            let v = (j as f64 + with_rng(rand::Rng::gen::<f64>)) / (image_height - 1) as f64;
            let r = cam.make_ray(u, v);
            color += ray_color(r, &world, MAX_DEPTH);
        }
        color / NUM_SAMPLES as f64
    });
    loadingbar.finish();
}

