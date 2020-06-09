use crate::hittable::{Hittable};
use std::rc::Rc;


pub struct BVH {
    pub left: Rc<dyn Hittable>,
    pub right: Rc<dyn Hittable>,
}

impl BVH {
    pub fn new() -> BVH {
        unimplemented!()

    }
}

// impl Hittable for BVH {
//     fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>> {
//         unimplemented!()
//     }
//
//     fn bbox(&self, t0: f32, t1: f32) -> Option<AaBb> {
//         unimplemented!()
//     }
// }
