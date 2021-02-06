use based::renderer::Renderer;
use based::camera::SimpleCamera;
use based::material::Material::{Metal, Dielectric, Lambertian};
use based::hittable::Sphere;
use glitz::vec::Vec3;
use xenon::color::Color;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    // World
    let mat_ground = Lambertian(Color::new(0.8, 0.8, 0.0));
    let mat_center = Lambertian(Color::new(0.1, 0.2, 0.5));
    let mat_left = Dielectric(1.5);
    let mat_right = Metal(Color::new(0.8, 0.6, 0.2), 0.0);
    let mut world = Vec::default();
    world.push(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, mat_ground));
    world.push(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, mat_center));
    world.push(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, mat_left.clone()));
    world.push(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.45, mat_left.clone()));
    world.push(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, mat_right));

    // Camera
    let lookfrom = Vec3::new(3.0, 3.0, 2.0);
    let lookat = Vec3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (lookfrom - lookat).length();
    let aperture = 0.5;
    let cam = SimpleCamera::new(lookfrom, lookat, vup, 20.0, aspect_ratio, aperture, dist_to_focus);

    Renderer::new(world, cam)
        .width(image_width)
        .aspect_ratio(aspect_ratio)
        .render_to_file("defocus.png")
}


