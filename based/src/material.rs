use glitz::vec::Vec3;
use crate::hittable::Hit;
use crate::ray::Ray;
use crate::random::with_rng;
use xenon::color::Color;

pub trait Material {
    fn scatter(&self, hit: &Hit, ri: Ray) -> Option<(Ray, Color)>;
}

#[derive(Clone)]
pub struct Lambertian {
    albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, hit: &Hit, _ri: Ray) -> Option<(Ray, Color)> {
        let mut scatter_direction = hit.normal + with_rng(Vec3::random_unit_vec);
        if scatter_direction.near_zero() {
            scatter_direction = hit.normal;
        }
        let scattered = Ray::new(hit.point, scatter_direction);
        Some((scattered, self.albedo))
    }
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}

#[derive(Clone)]
pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Metal {
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, hit: &Hit, ri: Ray) -> Option<(Ray, Color)> {
        let reflected = ri.d.unit_vec().reflect(&hit.normal);
        let scattered = Ray::new(hit.point, reflected + self.fuzz * with_rng(Vec3::random_in_unit_sphere));
        if scattered.d.dot(&hit.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct Dielectric {
    ir: f32,
}

impl Dielectric {
    pub fn new(ir: f32) -> Dielectric {
        Dielectric { ir }
    }

    fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
        // Use Schlick's approximation for reflectance.
        let r0_temp = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0_temp * r0_temp;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, hit: &Hit, ri: Ray) -> Option<(Ray, Color)> {
        let refraction_ratio = if hit.front_face { 1.0 / self.ir } else { self.ir };

        let unit_direction = ri.d.unit_vec();
        let cos_theta = (-unit_direction).dot(&hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        Some(if refraction_ratio * sin_theta > 1.0 || Self::reflectance(cos_theta, refraction_ratio) > with_rng(rand::Rng::gen) {
            (Ray::new(hit.point, unit_direction.reflect(&hit.normal)), Color::BLACK)
        } else {
            (Ray::new(hit.point, unit_direction.refract(&hit.normal, refraction_ratio)), Color::BLACK)
        })
    }
}

