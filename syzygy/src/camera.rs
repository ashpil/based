use glitz::vec::Vec3;
use crate::ray::Ray;

pub trait Camera {
    fn generate_ray(&self, u: f64, v: f64) -> Ray;
}

pub struct SimpleCamera {
    origin: Vec3,
    lower_left: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl SimpleCamera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f64, aspect_ratio: f64) -> SimpleCamera {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit_vec();
        let u = vup.cross(&w).unit_vec();
        let v = w.cross(&u);

        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let origin = lookfrom;

        SimpleCamera {
            origin,
            horizontal,
            vertical,
            lower_left: origin - horizontal / 2.0 - vertical / 2.0 - w,
        }
    }
}

impl Camera for SimpleCamera {
    fn generate_ray(&self, s: f64, t: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left + s * self.horizontal + t * self.vertical - self.origin,
        )
    }
}

