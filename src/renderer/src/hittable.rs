use crate::errors::{SpriosError, SpriosError::WorldParseError};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec::{Point3, Vec3};
use crate::bbox::AaBb;
use std::str::FromStr;
use crate::Sphere;
use std::sync::Arc;

#[derive(Clone)]
pub struct HitRecord<'obj> {
    pub normal: Vec3,
    pub front_face: bool,
    pub mat: &'obj dyn Material,
    pub p: Point3,
    pub t: f32,
}

impl<'obj> HitRecord<'obj> {
    pub fn new(mat: &'obj dyn Material) -> HitRecord<'obj> {
        HitRecord {
            normal: Vec3::ZERO,
            front_face: true,
            mat,
            p: Point3::ZERO,
            t: 0.0,
        }
    }
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal.clone()
        } else {
            -outward_normal.clone()
        };
    }
}

pub trait Hittable: Send + Sync {
    fn hit<'obj>(&'obj self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord<'obj>) -> bool;
    fn bbox(&self, t0: f32, t1: f32) -> Option<AaBb>;
    fn material(&self) -> Option<&dyn Material>;
    fn set_material(&mut self, mat: Box<dyn Material>);
    fn name(&self) -> &'static str;
}

pub fn from_string(s: &str) -> Result<Arc<dyn Hittable>, SpriosError> {
    let mut split = s.split_whitespace();
    let shape = split.next().ok_or(WorldParseError("empty object".to_string()))?;
    let parms = split
        .map(|v| v.parse::<f32>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_|WorldParseError("Could not parse object parms".to_string()))?;
    match shape {
        "sphere" => {
            let center = Point3::new(parms[0], parms[1], parms[2]);
            return Ok(Arc::new(Sphere::new(center, parms[3], None)));
        }
        _ => {}
    }
    Err(WorldParseError("Not a Hittable".to_string()))
}
