use crate::ray::Ray;
use crate::vec::{Point3, Vec3};

#[derive(Clone)]
pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new(lookfrom: Point3, lookat: Point3, vup: Vec3, vfov: u32, aspect_ratio: f32) -> Camera {
        let theta = vfov as f32 * std::f32::consts::PI / 180.0;
        let half_height = (theta/2.0).tan();
        let half_width = aspect_ratio * half_height;
        let w = (lookfrom - lookat).unit();
        let u = Vec3::cross(&vup, &w).unit();
        let v = Vec3::cross(&w, &u);
        let origin = lookfrom;
        let lower_left_corner = &origin - u * half_width - v * half_height - w;
        Camera {
            lower_left_corner,
            horizontal: u * half_width * 2.0,
            vertical: v * half_height * 2.0,
            origin,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            &self.origin,
            &(self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin),
        )
    }
}
