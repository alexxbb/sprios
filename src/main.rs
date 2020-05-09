mod vec;
mod ray;

use std::fmt::Write as FmWrite;
use std::io::Write;
use std::error::Error;
use vec::*;
use ray::Ray;
use std::process::Command;
use std::time::{Duration, Instant};


fn hit_sphere(center: Point3, radius: f32, ray: &Ray) -> f32 {
    let oc = ray.origin - center;
    let a = Vec3::dot(&ray.direction, &ray.direction);
    let b = 2.0 * Vec3::dot(&oc, &ray.direction);
    let c = Vec3::dot(&oc, &oc) - radius * radius;
    let descr = b * b - 4.0 * a * c;
    return if descr < 0.0 {
        -1.0
    } else {
        (-b - descr.sqrt()) / (2.0 * a)
    };
}

fn ray_color(ray: &Ray) -> Color {
    let hit = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, ray);
    if hit > 0.0
    {
        let n = Vec3::unit(&(ray.at(hit) - Vec3::new(0.0, 0.0, -1.0)));
        return Color::new(n.x + 1., n.y + 1., n.z + 1.) * 0.5;
    }
    let dir = Vec3::unit(&ray.direction);
    let t = 0.5 * (dir.y + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn main() -> Result<(), Box<dyn Error>> {
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 384;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;
    let cap = IMAGE_HEIGHT * IMAGE_WIDTH * (std::mem::size_of::<u32>() * 3) as u32;
    let mut buf = String::with_capacity(cap as usize);
    writeln!(&mut buf, "P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT)?;

    let origin = Point3::ZERO;
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.25, 0.0);
    let lower_left = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, 1.0);
    let start_time = Instant::now();
    for i in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rLines remaining: {} ", i);
        std::io::stderr().flush()?;
        for j in 0..IMAGE_WIDTH {
            let u = j as f32 / (IMAGE_WIDTH - 1) as f32;
            let v = i as f32 / (IMAGE_HEIGHT - 1) as f32;
            let dir = lower_left + horizontal * u + vertical * v;
            let ray = Ray::new(&origin, &dir);
            let color = ray_color(&ray);
            write_color(&mut buf, &color)?;
        }
    }
    std::fs::write("image.ppm", &buf).unwrap();
    eprintln!("\nDone in {}ms", start_time.elapsed().as_millis());
    Command::new("nomacs")
        .arg("--mode").arg("frameless")
        .arg("image.ppm").spawn()?;
    Ok(())
}