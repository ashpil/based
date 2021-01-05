use xenon::color::Color;
use based::material::{Lambertian, Metal, Dielectric};
use based::hittable::{Sphere, HittableList};
use based::renderer::Renderer;
use based::random::with_rng;
use based::camera::SimpleCamera;
use glitz::vec::Vec3;
use rand::Rng;

fn random_scene() -> HittableList {
    let ground_mat = Lambertian::new(Color::new(0.3, 0.9, 0.3));
    let mut world = HittableList::new();

    world.push(Box::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, ground_mat)));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = with_rng(Rng::gen::<f64>);
            let center = Vec3::new(a as f64 + 0.9*with_rng(Rng::gen::<f64>), 0.2, b as f64 + 0.9 * with_rng(Rng::gen::<f64>));

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Color::new(with_rng(Rng::gen::<f64>), with_rng(Rng::gen::<f64>), with_rng(Rng::gen::<f64>)) * Color::new(with_rng(Rng::gen::<f64>), with_rng(Rng::gen::<f64>), with_rng(Rng::gen::<f64>));
                    let sphere_mat = Lambertian::new(albedo);
                    world.push(Box::new(Sphere::new(center, 0.2, sphere_mat)));
                } else if choose_mat < 0.95 {
                    let albedo = Color::new(with_rng(|r| r.gen_range(0.5..1.0)), with_rng(|r| r.gen_range(0.5..1.0)), with_rng(|r| r.gen_range(0.5..1.0)));
                    let fuzz = with_rng(Rng::gen::<f64>);
                    let sphere_mat = Metal::new(albedo, fuzz);
                    world.push(Box::new(Sphere::new(center, 0.2, sphere_mat)));
                } else {
                    let sphere_mat = Dielectric::new(1.5);
                    world.push(Box::new(Sphere::new(center, 0.2, sphere_mat)));
                }
            }
        }
    }

    let mat1 = Dielectric::new(1.5);
    world.push(Box::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, mat1)));

    let mat2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.push(Box::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, mat2)));

    let mat3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.push(Box::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, mat3)));

    world
}
fn main() {
    let aspect_ratio = 3.0 / 2.0;
    let width = 800;
    let num_samples = 500;

    let world = random_scene();
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = SimpleCamera::new(lookfrom, lookat, vup, 20.0, aspect_ratio, aperture, dist_to_focus);

    Renderer::new(world, cam)
        .width(width)
        .aspect_ratio(aspect_ratio)
        .num_samples(num_samples)
        .render_to_file("oneweekend.png")
}
