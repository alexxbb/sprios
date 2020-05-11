use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec::Color;
use std::fmt::Debug;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &Color, scattered: &Ray) -> bool;
}

pub struct DefaultMaterial;

impl Material for DefaultMaterial {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &Color, scattered: &Ray) -> bool {
        unimplemented!()
    }
}

