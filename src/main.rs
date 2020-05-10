mod vec;
mod ray;
mod sphere;
mod hittable;
mod camera;
mod color;
mod utils;

use std::fmt::Write as FmWrite;
use std::io::Write;
use std::error::Error;
use std::time::{Instant};
use std::rc::Rc;
use rand::Rng;

use utils::*;
use vec::*;
use ray::Ray;
use sphere::Sphere;
use color::write_color;
use camera::Camera;
use hittable::{Hittable, HitRecord};


type World = Vec<Rc<dyn Hittable>>;

fn ray_color(ray: &Ray, world: &World) -> Color {
    let mut rec = HitRecord::default();

    if world.hit(ray, 0.0, f32::INFINITY, &mut rec) {
        return (rec.normal + Color::ONE) * 0.5;
    }
    let dir = Vec3::unit(&ray.direction);
    let t = 0.5 * (dir.y + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

#[allow(non_upper_case_globals)]
fn main() -> Result<(), Box<dyn Error>> {
    const aspect_ratio: f32 = 16.0 / 9.0;
    const image_width: u32 = 384;
    const image_height: u32 = (image_width as f32 / aspect_ratio) as u32;
    const samples_per_pixel: u32 = 10;
    let cap = image_height * image_width * (std::mem::size_of::<u32>() * 3) as u32;
    let mut buf = String::with_capacity(cap as usize);
    writeln!(&mut buf, "P3\n{} {}\n255", image_width, image_height)?;

    let start_time = Instant::now();

    let camera = Camera::new();
    let mut world = World::new();
    world.push(Rc::new(Sphere::new((0.0, 0.0, -1.0), 0.5)));
    world.push(Rc::new(Sphere::new((0.0, -100.5, -1.0), 100.0)));

    let mut rng = rand::thread_rng();
    for i in (0..image_height).rev() {
        eprint!("\rLines remaining: {} ", i);
        std::io::stderr().flush()?;
        for j in 0..image_width {
            let mut pixel_color = Color::ZERO;
            for _ in 0..samples_per_pixel {
                let u = (j as f32 + rng.gen::<f32>() ) / (image_width - 1) as f32;
                let v = (i as f32 + rng.gen::<f32>() )/ (image_height - 1) as f32;
                let ray = camera.get_ray(u, v);
                pixel_color += &ray_color(&ray, &world);
            }
            write_color(&mut buf, &pixel_color, samples_per_pixel)?;
        }
    }
    std::fs::write("image.ppm", &buf).unwrap();
    eprintln!("\nDone in {}ms", start_time.elapsed().as_millis());
    // Command::new("nomacs")
    //     .arg("--mode").arg("frameless")
    //     .arg("image.ppm").spawn()?;
    Ok(())
}