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
    pub fn new() -> SimpleCamera {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Vec3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left = origin - (horizontal / 2.0) - (vertical / 2.0) - Vec3::new(0.0, 0.0, focal_length);

        SimpleCamera {
            origin,
            horizontal,
            vertical,
            lower_left,
        }
    }
}

impl Camera for SimpleCamera {
    fn generate_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}

