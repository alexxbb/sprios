use crate::vec::{Point3, Vec3};
use crate::material::Material;
use crate::ray::Ray;
use std::rc::Rc;

#[derive(Clone)]
pub struct HitRecord {
    pub normal: Vec3,
    pub front_face: bool,
    pub mat: Rc<dyn Material>,
    pub p: Point3,
    pub t: f32,
}

impl HitRecord {
    pub fn new(mat: Rc<dyn Material>) -> HitRecord {
        HitRecord{
            normal: Vec3::ZERO,
            front_face: true,
            mat,
            p: Point3::ZERO,
            t: 0.0
        }
    }
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(&ray.direction, outward_normal) < 0.0;
        self.normal = if self.front_face { *outward_normal } else { -*outward_normal };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

impl Hittable for Vec<Rc<dyn Hittable>> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut temp_rec:Option<HitRecord> = None;
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for obj in self {
            if let Some(hit) = obj.hit(ray, t_min, closest_so_far) {
                hit_anything = true;
                closest_so_far = hit.t;
                temp_rec = Some(hit);
            }
        }
        temp_rec
    }
}

