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
use getopts;

use vec::*;
use ray::Ray;
use sphere::Sphere;
use color::write_color;
use camera::Camera;
use hittable::{Hittable, HitRecord};


type World = Vec<Rc<dyn Hittable>>;

fn ray_color(ray: &Ray, world: &World, depth: u32) -> Color {
    let mut rec = HitRecord::default();
    if depth == 0 {
        // Max recursion depth reached
        return Color::ZERO
    }

    if world.hit(ray, 0.001, f32::INFINITY, &mut rec) {
        let target = rec.p + rec.normal + Vec3::random_unit_vector();
        return ray_color(&Ray::new(&rec.p, &(target - rec.p)), world, depth - 1) * 0.5
        // return (rec.normal + Color::ONE) * 0.5;
    }
    let dir = Vec3::unit(&ray.direction);
    let t = 0.5 * (dir.y + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

#[allow(non_upper_case_globals)]
fn main() -> Result<(), Box<dyn Error>> {
    let args:Vec<String> = std::env::args().collect();
    const aspect_ratio: f32 = 16.0 / 9.0;
    const max_depth:u32 = 10;

    let mut opts = getopts::Options::new();
    opts.optopt("w", "width", "Image width", "WIDTH");
    opts.optopt("s", "samples", "Pixel samples", "SAMPLES");
    opts.optflag("h", "help", "print help");

    let args = match opts.parse(args) {
        Ok(m) => m,
        Err(e) => {panic!(e.to_string())}
    };

    let image_width: u32 = match args.opt_str("w") {
        Some(s) => {s.parse().unwrap()},
        None => 386
    };
    let image_height: u32 = (image_width as f32 / aspect_ratio) as u32;
    let samples_per_pixel: u32 = match args.opt_str("s") {
        Some(s) => {s.parse().unwrap()},
        None=> 10
    };

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
        let cur_line = image_height - i;
        let prog =  (cur_line as f32 / image_height as f32) * 100.0;
        eprint!("\rRendering: {}%", prog as u32);
        std::io::stderr().flush()?;
        for j in 0..image_width {
            let mut pixel_color = Color::ZERO;
            for _ in 0..samples_per_pixel {
                let u = (j as f32 + rng.gen::<f32>() ) / (image_width - 1) as f32;
                let v = (i as f32 + rng.gen::<f32>() )/ (image_height - 1) as f32;
                let ray = camera.get_ray(u, v);
                pixel_color += &ray_color(&ray, &world, max_depth);
            }
            write_color(&mut buf, &pixel_color, samples_per_pixel)?;
        }
    }
    std::fs::write("image.ppm", &buf).unwrap();
    eprintln!("\nDone in {}ms", start_time.elapsed().as_millis());
    Ok(())
}