use glitz::vec::Vec3;
use crate::ray::Ray;
use crate::material::Material;

pub struct Hit<'a> {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat: &'a dyn Material,
}

impl<'a> Hit<'a> {
    pub fn new(point: Vec3, normal: Vec3, t: f64, front_face: bool, mat: &'a dyn Material) -> Hit<'a> {
        Hit {
            point, 
            normal,
            t,
            front_face,
            mat,
        }
    }
}

pub trait Hittable {
    fn intersect(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<Hit>;
}

pub struct Sphere<M: Material> {
    center: Vec3,
    radius: f64,
    mat: M,
}

impl<M: Material> Sphere<M> {
    pub fn new(center: Vec3, radius: f64, mat: M) -> Sphere<M> {
        Sphere {
            center,
            radius,
            mat,
        }
    }
}

impl<M: Material> Hittable for Sphere<M> {

    fn intersect(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<Hit> {
        let oc = r.o - self.center;
        let a = r.d.dot(&r.d);
        let half_b = oc.dot(&r.d);
        let c = oc.dot(&oc) - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            None
        } else {
            let sqrtd = discriminant.sqrt();

            // Find the nearest root that lies in the acceptable range.
            let mut root = (-half_b - sqrtd) / a;
            if !(tmin..tmax).contains(&root) {
                root = (-half_b + sqrtd) / a;
                if !(tmin..tmax).contains(&root) {
                    return None;
                }
            }
            let point = r.at(root);
            let outward_normal = (point - self.center) / self.radius;
            let front_face = r.d.dot(&outward_normal) < 0.0;
            let normal = if front_face { outward_normal } else { -outward_normal };
            Some(Hit::new(point, normal, root, front_face, &self.mat))
        }
    }
}

pub type HittableList = Vec<Box<dyn Hittable + Sync>>;
impl Hittable for HittableList {
    fn intersect(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<Hit> {
        let mut result = None;
        let mut closest_so_far = tmax;
        for obj in self {
            if let Some(ray_hit) = obj.intersect(r, tmin, closest_so_far) {
                closest_so_far = ray_hit.t;
                result = Some(ray_hit);
            }
        }
        result
    }
}

