use crate::vec;

pub struct Ray {
    origin: vec::Point3,
    direction: vec::Vec3
}

impl Ray {
    pub fn new() -> Self {
        Ray{origin: vec::Point3::ZERO, direction: vec::Vec3::ZERO}
    }

    pub fn at(&self, t: f32) -> vec::Point3 {
        self.origin + self.direction * t
    }
}