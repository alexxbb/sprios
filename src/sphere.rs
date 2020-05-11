use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;
use crate::vec::*;
use crate::material::{Material, DefaultMaterial};
use std::rc::Rc;


#[derive(Clone)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
    pub material: Rc<dyn Material>
}

impl Sphere {
    pub fn new<P: Into<Point3> >(center: P, radius: f32) -> Sphere{
        Sphere{center: center.into(), radius, material: Rc::new(DefaultMaterial{})}
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = Vec3::dot(&oc, &ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                let mut rec = HitRecord::new(Rc::clone(&self.material));
                rec.t = temp;
                rec.p = ray.at(temp);
                let outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(ray, &outward_normal);
                return Some(rec)
            }
            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                let mut rec = HitRecord::new(Rc::clone(&self.material));
                rec.t = temp;
                rec.p = ray.at(temp);
                let outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(ray, &outward_normal);
                return Some(rec)
            }
        }
        None
    }
}