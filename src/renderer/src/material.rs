use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec::{Color, Vec3};
use std::str::FromStr;
use std::convert::TryInto;
use crate::errors::SpriosError::WorldParseError;

pub trait Material: Sync + Send {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, rng: Option<&mut dyn rand::RngCore>) -> Option<Ray>;
    fn color(&self) -> &Color;
}


impl FromStr for Box<dyn Material> {
    type Err = crate::errors::SpriosError;

    fn from_str(s: &str) -> Result<Box<dyn Material>, Self::Err> {
        let parts: Vec<&str> = s.split(" ").collect();
        let mat = *parts.get(0).ok_or_else(||WorldParseError("Missing material type".to_string()))?;
        let parms = parts.iter()
            .skip(1)
            .map(|v| v.parse::<f32>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_|WorldParseError("Could not parse material parms".to_string()))?;
        match mat
        {
            "diffuse" => {
                if parms.len() < 3 {
                    return Err(WorldParseError("Color must have 3 components".to_string()));
                }
                Ok(Box::new(Lambertian { color: Color::from(&[parms[0], parms[1], parms[2]]) }))
            }
            "metal" => {
                if parms.len() < 3 {
                    return Err(WorldParseError("Color must have 3 components".to_string()));
                }
                Ok(Box::new(Metal {
                    color: Color::from(&[parms[0], parms[1], parms[2]]),
                    fuzz: *parms.last().ok_or(WorldParseError("Missing fuzz parm".to_string()))?,
                }))
            }
            "glass" => {
                return Err(WorldParseError("Glass not supported".to_string()));
            }
            m => {
                return Err(WorldParseError(format!("Unknown material {}", m)));
            }
        }
    }
}

#[derive(Debug)]
pub struct Lambertian {
    pub color: Color,
}


pub struct Metal {
    pub color: Color,
    pub fuzz: f32,
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, rng: Option<&mut dyn rand::RngCore>) -> Option<Ray> {
        let mut trng: rand::rngs::ThreadRng;
        let mut rng = match rng {
            Some(r) => r,
            None => {
                trng = rand::thread_rng();
                &mut trng
            }
        };
        let scatter_direction = &rec.normal + Vec3::random_unit_vector(&mut rng);
        Some(Ray::new(&rec.p, &scatter_direction))
    }

    fn color(&self) -> &Color {
        &self.color
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, rng: Option<&mut dyn rand::RngCore>) -> Option<Ray> {
        let reflected = r_in.direction.unit().reflect(&rec.normal);
        let mut trng: rand::rngs::ThreadRng;
        let mut rng = match rng {
            Some(r) => r,
            None => {
                trng = rand::thread_rng();
                &mut trng
            }
        };
        let scattered = Ray::new(&rec.p, &(reflected + Vec3::random_in_unit_sphere(&mut rng) * self.fuzz));
        if scattered.direction.dot(&rec.normal) > 0.0 {
            return Some(scattered);
        }
        None
    }

    fn color(&self) -> &Color {
        &self.color
    }
}
