use crate::hittable::{HitRecord, Hittable};
use crate::{Ray, Camera};
use std::sync::Arc;
use crate::bbox::AaBb;
use std::path::Path;
use std::io::Read;

trait Foo: Send + Sync {}

pub struct World {
    pub(crate) objects: Vec<Arc<dyn Hittable>>,
    pub(crate) camera: Camera,
}

impl Hittable for World {
    fn hit<'obj>(&'obj self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord<'obj>) -> bool {
        let mut temp_rec = rec.clone();
        let mut been_hit = false;
        let mut closest_so_far = t_max;

        for obj in &self.objects {
            if obj.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                been_hit = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }
        been_hit
    }

    fn bbox(&self, t0: f32, t1: f32) -> Option<AaBb> {
        if self.objects.is_empty() {
            return None;
        }
        let mut first_bbox = true;
        let mut out_box = None;

        for obj in &self.objects {
            if let Some(bbox) = obj.bbox(t0, t1) {
                out_box = if first_bbox {
                    Some(bbox)
                } else {
                    Some(AaBb::surrounding_box(&out_box.unwrap(), &bbox))
                };
                first_bbox = false;
            } else {
                return None;
            }
        }
        out_box
    }
}

impl World {
    pub fn new() -> World {
        World { objects: vec![], camera: Camera::default()}
    }
    /*
        Why 'static is needed here??
        Because for rustc, a concrete object (that implements Hittable) could as well potentially
        contain references. But we're taking the ownership here and storing the object in Arc,
        which requires the data to be 'static
     */
    pub fn add(&mut self, object: impl Hittable + 'static) {
        self.objects.push(Arc::new(object))
    }

    pub fn from_file(path: impl AsRef<Path>) -> Result<World, String> {
        match std::fs::File::open(path)
        {
            Err(e) => return Err(e.to_string()),
            Ok(mut file) => {
                let mut content= String::new();
                file.read_to_string(&mut content);

                for line in content.lines() {
                    if line.starts_with('#') || line.is_empty() {
                        continue
                    }
                    if let Ok(cam) = line.parse::<Camera>() {
                        dbg!(cam);
                    }
                    else {
                        eprintln!("Could not parse line: {}", line);
                    }
                }
            }

        }
        Ok(World::new())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Sphere, Lambertian, Color, Point3};
    use std::path::Path;

    #[test]
    fn test_1() {
        let mut world = World::new();
        world.add(
            Sphere::new((0.0, 0.0, 0.0), 0.5, Box::new(Lambertian { color: Color::ONE })),
        );
        world.add(
            Sphere::new((1.0, 0.0, 0.0), 0.5, Box::new(Lambertian { color: Color::ONE })),
        );
        let bbox = world.bbox(0.0, 0.0).unwrap();
        assert_eq!(&bbox.min, &Point3::new(-0.5, -0.5, -0.5));
        assert_eq!(&bbox.max, &Point3::new(1.5, 0.5, 0.5));
        world.add(
            Sphere::new((1.0, 1.0, 0.0), 0.5, Box::new(Lambertian { color: Color::ONE })),
        );
        let bbox = world.bbox(0.0, 0.0).unwrap();
        assert_eq!(&bbox.max, &Point3::new(1.5, 1.5, 0.5));
    }

    #[test]
    fn test_read_file() {
        assert!(World::from_file(Path::new("Foo")).is_err());
        let sc_1 = Path::new("../scene_1.rsc");
        assert!(World::from_file(sc_1).is_ok());
    }
}
