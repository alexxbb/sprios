use crate::ray::Ray;
use crate::vec::{Point3, Vec3};

#[derive(Clone)]
pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lens_radius: f32,
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl Camera {
    pub fn new(lookfrom: Point3, lookat: Point3, vup: Vec3, vfov: u32, aspect_ratio: f32, aperture: f32, focus_dist: f32) -> Camera {
        let theta = vfov as f32 * std::f32::consts::PI / 180.0;
        let viewport_height = (theta/2.0).tan() * 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let w = (lookfrom - lookat).unit();
        let u = Vec3::cross(&vup, &w).unit();
        let v = Vec3::cross(&w, &u);
        let origin = lookfrom;
        let horizontal = u * focus_dist * viewport_width;
        let vertical = v * focus_dist * viewport_height;
        let lower_left_corner = &origin - horizontal/2.0 - vertical/2.0 - w * focus_dist;
        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            lens_radius: aperture / 2.0,
            u,
            v,
            w
        }
    }

    pub fn get_ray(&self, s: f32, t: f32, rng: &mut impl rand::RngCore) -> Ray {
        let rd = Vec3::random_unit_vector(rng) * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            &(self.origin + offset),
            &(self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset),
        )
    }
}
