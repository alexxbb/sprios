use crate::vec::{Point3, Vec3};

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: &Point3, direction: &Vec3) -> Self {
        Ray {
            origin: origin.clone(),
            direction: direction.clone(),
        }
    }
    // pub fn zero() -> Self {
    //     Ray { origin: Point3::ZERO, direction: Vec3::ZERO }
    // }

    pub fn at(&self, t: f32) -> Point3 {
        &self.origin + &self.direction * t
    }
}
