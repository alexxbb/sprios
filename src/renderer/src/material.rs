use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec::{Color, Vec3};

pub trait Material: Sync + Send {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Ray>;
    fn color(&self) -> &Color;
}

pub struct Lambertian {
    pub color: Color,
}

pub struct Metal {
    pub color: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<Ray> {
        let scatter_direction = &rec.normal + Vec3::random_unit_vector();
        Some(Ray::new(&rec.p, &scatter_direction))
    }

    fn color(&self) -> &Color {
        &self.color
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Ray> {
        let reflected = r_in.direction.unit().reflect(&rec.normal);
        let scattered = Ray::new(&rec.p, &reflected);
        if scattered.direction.dot(&rec.normal) > 0.0 {
            return Some(scattered);
        }
        None
    }

    fn color(&self) -> &Color {
        &self.color
    }
}
