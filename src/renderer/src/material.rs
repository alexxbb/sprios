use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec::{Color, Vec3};
use std::str::FromStr;

pub trait Material: Sync + Send {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, rng: Option<&mut dyn rand::RngCore>) -> Option<Ray>;
    fn color(&self) -> &Color;
}

pub enum MaterialError {
    ParseError
}

impl FromStr for Box<dyn Material> {
    type Err = MaterialError;

    fn from_str(s: &str) -> Result<Box<dyn Material>, Self::Err> {
        let mut split = s.splitn(2, " ");
        match split.next().ok_or(MaterialError::ParseError)?
        {
            "diffuse" => {
                Ok(Box::new(Lambertian {
                    color: split.next().ok_or(MaterialError::ParseError)?
                        .parse().map_err(|_| MaterialError::ParseError)?
                }))
            }
            "metal" => {
                let parms = split.next().ok_or(MaterialError::ParseError)?
                    .split(" ")
                    .map(|v| v.parse::<f32>())
                    .collect::<Result<Vec<_>, _>>().map_err(|_| MaterialError::ParseError)?;
                if parms.len() < 4 {
                    return Err(MaterialError::ParseError);
                }
                Ok(Box::new(Metal {
                    color: Color::from((parms[0], parms[1], parms[2])),
                    fuzz: parms[3],
                }))
            }
            _ => {
                Err(MaterialError::ParseError)
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
