use crate::vec::{Point3, Vec3};
use crate::material::Material;
use crate::ray::Ray;
use std::sync::Arc;

#[derive(Clone)]
pub struct HitRecord {
    pub normal: Vec3,
    pub front_face: bool,
    pub mat: Arc<dyn Material>,
    pub p: Point3,
    pub t: f32,
}

impl HitRecord {
    pub fn new(mat: Arc<dyn Material>) -> HitRecord {
        HitRecord{
            normal: Vec3::ZERO,
            front_face: true,
            mat,
            p: Point3::ZERO,
            t: 0.0
        }
    }
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face { *outward_normal } else { -*outward_normal };
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
