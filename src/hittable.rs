use crate::vec::{Point3, Vec3};
use crate::ray::Ray;
use std::rc::Rc;

#[derive(Debug, Default, Copy, Clone)]
pub struct HitRecord {
    pub normal: Vec3,
    pub front_face: bool,
    pub p: Point3,
    pub t: f32,
}

impl HitRecord {
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
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for obj in self {
            if let Some(hit) = obj.hit(ray, t_min, closest_so_far) {
                hit_anything = true;
                closest_so_far = hit.t;
                temp_rec = hit
            }
        }
        if hit_anything {Some(temp_rec)} else {None}
    }
}

