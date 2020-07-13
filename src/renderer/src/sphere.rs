use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec::*;
use crate::bbox::AaBb;
use crate::Lambertian;

pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
    pub material: Box<dyn Material>,
}

impl Sphere {
    pub fn new<P: Into<Point3>>(center: P, radius: f32, mat: Option<Box<dyn Material>>) -> Sphere {
        Sphere {
            center: center.into(),
            radius,
            material: mat.unwrap_or(Box::new(Lambertian{color: Color::new(0.8, 0.8, 0.8)})),
        }
    }
}

impl Hittable for Sphere {
    fn hit<'obj>(&'obj self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord<'obj>) -> bool {
        let oc = &ray.origin - &self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                rec.mat = self.material.as_ref();
                rec.t = temp;
                rec.p = ray.at(temp);
                let outward_normal = (&rec.p - &self.center) / self.radius;
                rec.set_face_normal(ray, &outward_normal);
                return true;
            }
            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                rec.mat = self.material.as_ref();
                rec.t = temp;
                rec.p = ray.at(temp);
                let outward_normal = (&rec.p - &self.center) / self.radius;
                rec.set_face_normal(ray, &outward_normal);
                return false;
            }
        }
        false
    }

    fn bbox(&self, _t0: f32, _t1: f32) -> Option<AaBb> {
        let rad = Vec3::new(self.radius, self.radius, self.radius);
        Some(AaBb::new(&self.center - &rad, &self.center + &rad))
    }

    fn material(&self) -> Option<&dyn Material> {
        Some(self.material.as_ref())
    }

    fn set_material(&mut self, mat: Box<dyn Material>) {
        self.material = mat;
    }

    #[inline]
    fn name(&self) -> &'static str {
        "Sphere"
    }
}
