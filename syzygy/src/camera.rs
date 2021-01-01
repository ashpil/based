use glitz::vec::Vec3;
use crate::ray::Ray;
use rand::Rng;

pub trait Camera {
    fn make_ray(&self, u: f64, v: f64, rng: &mut impl Rng) -> Ray;
}

pub struct SimpleCamera {
    origin: Vec3,
    lower_left: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64,
}

impl SimpleCamera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f64, aspect_ratio: f64, aperture: f64, focus_dist: f64) -> SimpleCamera {
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
    fn make_ray(&self, s: f64, t: f64, rng: &mut impl Rng) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk(rng);
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            offset + self.origin,
            self.lower_left + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}

