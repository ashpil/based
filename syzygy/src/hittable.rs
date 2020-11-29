use glitz::vec::Vec3;
use crate::ray::Ray;
use crate::material::Material;
use std::rc::Rc;

pub struct Hit {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat: Rc<dyn Material>,
}

impl Hit {
    pub fn new(point: Vec3, normal: Vec3, t: f64, front_face: bool, mat: Rc<dyn Material>) -> Hit {
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

pub struct Sphere {
    center: Vec3,
    radius: f64,
    mat: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, mat: Rc<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            mat,
        }
    }
}

impl Hittable for Sphere {

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
            let roots = [(-half_b - sqrtd) / a, (-half_b + sqrtd) / a];
            if let Some(root) = roots.iter().find(|x| (tmin..tmax).contains(&x)) {
                let point = r.at(*root);
                let outward_normal = (point - self.center) / self.radius;
                let front_face = r.d.dot(&outward_normal) < 0.0;
                let normal = if front_face { outward_normal } else { -outward_normal };
                Some(Hit::new(point, normal, *root, front_face, Rc::clone(&self.mat)))
            } else {
                None
            }
        }
    }
}

pub struct Hittables(Vec<Box<dyn Hittable>>);

impl Hittables {
    pub fn new() -> Self {
        Hittables(Vec::new())
    }

    pub fn add(&mut self, shape: Box<dyn Hittable>) {
        self.0.push(shape);
    }
}

impl Hittable for Hittables {
    fn intersect(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<Hit> {
        self.0.iter().fold(None, |acc, curr| 
            curr.intersect(r, tmin, acc.as_ref().map_or(tmax, |x| x.t)).or(acc)
        )
    }
}

