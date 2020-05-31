use crate::vec::{Vec3, Point3};
use crate::ray::Ray;

#[derive(Debug)]
pub struct AaBb {
    pub min: Point3,
    pub max: Point3,
}


impl AaBb {
    pub fn new(min: Point3, max: Point3) -> Self {
        AaBb { min, max }
    }

    pub fn hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> bool {
        for a in 0..3 {
            let invd = 1.0 / ray.direction[a];
            let mut t0 = (self.min[a] - ray.origin[a]) * invd;
            let mut t1 = (self.max[a] - ray.origin[a]) * invd;
            if invd < 0.0f32 {
                std::mem::swap(&mut t0, &mut t1);
            }
            let tmin = if t0 > tmin { t0 } else { tmin };
            let tmax = if t1 > tmax { t1 } else { tmax };
            if tmax <= tmin {
                return false;
            }
        }
        true
    }

    pub fn surrounding_box(box0: &AaBb, box1: &AaBb) -> AaBb {
        let small = Point3::new(f32::min(box0.min.x, box1.min.x),
                                f32::min(box0.min.y, box1.min.y),
                                f32::min(box0.min.z, box1.min.z));

        let big = Point3::new(f32::max(box0.max.x, box1.max.x),
                              f32::max(box0.max.y, box1.max.y),
                              f32::max(box0.max.z, box1.max.z));
        AaBb::new(small, big)
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hit() {
        let min = Point3::new(3.0, 2.0, 0.0);
        let max = Point3::new(5.0, 4.0, 0.0);
        let aabb = AaBb::new(min, max);
        let mut ray = Ray::new(&Point3::new(1.0, 1.0, 0.0), &Vec3::new(2.0, 2.0, 0.0));
        assert!(aabb.hit(&ray, 0.0001, f32::INFINITY));
        let mut ray = Ray::new(&Point3::new(1.0, 1.0, 0.0), &Vec3::new(2.0, 1.0, 0.0));
        assert!(!aabb.hit(&ray, 0.0001, f32::INFINITY));
    }
}