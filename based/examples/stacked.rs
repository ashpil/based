use based::renderer::Renderer;
use based::camera::SimpleCamera;
use based::material::Material::{Metal, Dielectric, Lambertian};
use based::hittable::Sphere;
use glitz::vec::Vec3;
use xenon::color::Color;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1200;
    let num_samples = 500;

    // World
    let ground = Lambertian(Color::new(0.4, 0.6, 0.6));
    let blue = Lambertian(Color::new(0.1, 0.2, 0.5));
    let red = Lambertian(Color::new(0.9, 0.05, 0.05));
    let gold = Metal(Color::new(0.8, 0.6, 0.2), 0.0);
    let malachite = Metal(Color::new(0.2, 0.8, 0.2), 0.3);
    let glass = Dielectric(2.8);
    let glass2 = Dielectric(1.5);
    let mut world = Vec::default();
    world.push(Sphere::new(Vec3::new(0.0, -1000.75, 0.0), 1000.0, ground));
    world.push(Sphere::new(Vec3::new(0.0, 0.0, 0.0), 0.75, blue));
    world.push(Sphere::new(Vec3::new(0.0, 0.0, 1.6), 0.75, gold));
    world.push(Sphere::new(Vec3::new(0.0, 0.0, -1.6), 0.75, glass));
    world.push(Sphere::new(Vec3::new(0.0, 1.2, 0.8), 0.75, red));
    world.push(Sphere::new(Vec3::new(0.0, 1.2, -0.8), 0.75, malachite));
    world.push(Sphere::new(Vec3::new(0.0, 2.4, 0.0), 0.75, glass2));

    // Camera
    let lookfrom = Vec3::new(17.0, 4.0, 3.0);
    let lookat = Vec3::new(0.0, 1.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 17.0;
    let aperture = 0.5;

    let cam = SimpleCamera::new(lookfrom, lookat, vup, 20.0, aspect_ratio, aperture, dist_to_focus);

    Renderer::new(world, cam)
        .width(image_width)
        .aspect_ratio(aspect_ratio)
        .num_samples(num_samples)
        .render_to_file("stacked.png")
}


