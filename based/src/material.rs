use glitz::vec::Vec3;
use crate::hittable::Hit;
use crate::ray::Ray;
use crate::random::with_rng;
use xenon::color::Color;

pub enum Material {
    Lambertian(Color),
    Metal(Color, f64),
    Dielectric(f64),
}

impl Material {
    fn scatter_lambertian(albedo: Color, hit: Hit) -> Option<(Ray, Color)> {
        let mut scatter_direction = hit.normal + with_rng(Vec3::random_unit_vec);
        if scatter_direction.near_zero() {
            scatter_direction = hit.normal;
        }
        let scattered = Ray::new(hit.point, scatter_direction);
        Some((scattered, albedo))
    }

    fn scatter_metal(albedo: Color, fuzz: f64, hit: Hit, r: Ray) -> Option<(Ray, Color)> {
        let reflected = r.d.unit_vec().reflect(&hit.normal);
        let scattered = Ray::new(hit.point, reflected + fuzz * with_rng(Vec3::random_in_unit_sphere));
        if scattered.d.dot(&hit.normal) > 0.0 {
            Some((scattered, albedo))
        } else {
            None
        }
    }

    fn scatter_dielectric(ir: f64, hit: Hit, r: Ray) -> Option<(Ray, Color)> {
        let refraction_ratio = if hit.front_face { 1.0 / ir } else { ir };

        let unit_direction = r.d.unit_vec();
        let cos_theta = (-unit_direction).dot(&hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        Some(if refraction_ratio * sin_theta > 1.0 || Self::reflectance(cos_theta, refraction_ratio) > with_rng(rand::Rng::gen) {
            (Ray::new(hit.point, unit_direction.reflect(&hit.normal)), Color::BLACK)
        } else {
            (Ray::new(hit.point, unit_direction.refract(&hit.normal, refraction_ratio)), Color::BLACK)
        })
    }
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let r0_temp = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0_temp * r0_temp;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }

    pub fn scatter(&self, hit: Hit, r: Ray) -> Option<(Ray, Color)> {
        use self::Material::*;
        match self {
            Lambertian(color) => Self::scatter_lambertian(*color, hit),
            Metal(color, fuzz) => Self::scatter_metal(*color, *fuzz, hit, r),
            Dielectric(ir) => Self::scatter_dielectric(*ir, hit, r),
        }
    }
}
