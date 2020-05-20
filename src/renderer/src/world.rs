use crate::hittable::{Hittable, HitRecord};
use std::sync::Arc;
use crate::Ray;

trait Foo: Send + Sync {}

pub struct World {
    pub(crate) objects: Vec<Arc<dyn Hittable>>
}

impl World {
    pub fn new() -> World {
        World { objects: vec![] }
    }
    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut temp_rec:Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for obj in &self.objects {
            if let Some(hit) = obj.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit.t;
                temp_rec = Some(hit);
            }
        }
        temp_rec
    }

    // Why 'static is needed here??
    pub fn add(&mut self, object: impl Hittable + 'static) {
        self.objects.push(Arc::new(object))
    }
}