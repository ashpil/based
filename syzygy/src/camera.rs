use glitz::vec::Vec3;
use crate::ray::Ray;
use crate::random::with_rng;
use rand::distributions::Uniform;
use rand::Rng;

pub trait Camera {
    fn make_ray(&self, u: f32, v: f32) -> Ray;
}

pub struct SimpleCamera {
    origin: Vec3,
    lower_left: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f32,
}

impl SimpleCamera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f32, aspect_ratio: f32, aperture: f32, focus_dist: f32) -> SimpleCamera {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit_vec();
        let u = vup.cross(&w).unit_vec();
        let v = w.cross(&u);

        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;

        SimpleCamera {
            origin,
            horizontal,
            vertical,
            u,
            v,
            lower_left: origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w,
            lens_radius: aperture / 2.0,
        }
    }
}

impl Camera for SimpleCamera {
    fn make_ray(&self, s: f32, t: f32) -> Ray {

        let range = Uniform::new_inclusive(-1.0, 1.0);
        let mut rd = [with_rng(|r| Rng::sample(r, range)), with_rng(|r| Rng::sample(r, range))];
        while rd[0] * rd[0] + rd[1] * rd[1] >= 1.0 {
            rd = [with_rng(|r| Rng::sample(r, range)), with_rng(|r| Rng::sample(r, range))];
        }

        let offset = (self.u * rd[0] + self.v * rd[1]) * self.lens_radius;

        Ray::new(
            offset + self.origin,
            self.lower_left + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}

