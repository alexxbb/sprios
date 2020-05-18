mod camera;
mod hittable;
mod imagebuffer;
mod material;
mod ray;
mod sphere;
mod utils;
mod vec;

pub use camera::Camera;
pub use hittable::{HitRecord, Hittable};
pub use imagebuffer::ImageBuffer;
pub use material::{Lambertian, Metal};
use rand::Rng;
pub use ray::Ray;
pub use sphere::Sphere;
use std::ops::DerefMut;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
pub use vec::{Color, Vec3};

type World = Vec<Rc<dyn Hittable>>;

fn ray_color(ray: &Ray, world: &World, depth: u32) -> Color {
    if depth == 0 {
        // Max recursion depth reached
        return Color::ZERO;
    }

    if let Some(rec) = world.hit(ray, 0.001, f32::INFINITY) {
        if let Some(ray) = rec.mat.scatter(ray, &rec) {
            return rec.mat.color() * ray_color(&ray, world, depth - 1);
        }
        return Color::ZERO;
    }
    let dir = ray.direction.unit();
    let t = 0.5 * (dir.y + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

pub fn render<F>(width: u32, height: u32, samples: u32, buf: Arc<Mutex<ImageBuffer>>, progress: F)
where
    F: Fn(u32)
{
    const MAX_DEPTH: u32 = 10;
    let camera = Camera::new();
    let mut world = World::new();
    // Globe sphere
    world.push(Rc::new(Sphere::new(
        (0.0, -100.5, -1.0),
        100.0,
        Rc::new(Lambertian {
            color: (0.5, 0.5, 0.5).into(),
        }),
    )));
    // Red
    world.push(Rc::new(Sphere::new(
        (-1.0, 0.0, -1.0),
        0.5,
        Rc::new(Metal {
            color: (0.9, 0.1, 0.1).into(),
        }),
    )));
    // Green
    world.push(Rc::new(Sphere::new(
        (1.0, 0.0, -1.0),
        0.5,
        Rc::new(Metal {
            color: (0.1, 0.9, 0.1).into(),
        }),
    )));
    // Blue
    world.push(Rc::new(Sphere::new(
        (0.0, 0.0, -1.0),
        0.5,
        Rc::new(Metal {
            color: (0.1, 0.1, 0.9).into(),
        }),
    )));

    let mut buf = buf.lock().unwrap();
    let buf = buf.deref_mut();
    let mut rng = rand::thread_rng();
    for i in (0..height).rev() {
        let cur_line = height - i;
        let _progress = (cur_line as f32 / height as f32) * 100.0;
        progress(_progress as u32);
        for j in 0..width {
            let mut pixel_color = Color::ZERO;
            for _ in 0..samples {
                let u = (j as f32 + rng.gen::<f32>()) / (width - 1) as f32;
                let v = (i as f32 + rng.gen::<f32>()) / (height - 1) as f32;
                let ray = camera.get_ray(u, v);
                pixel_color += &ray_color(&ray, &world, MAX_DEPTH);
            }
            buf.write_color(&pixel_color, samples);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{render, Arc, ImageBuffer, Mutex};

    #[test]
    fn test_render() {
        let buf = Vec::<u8>::new();
        let mut buf = ImageBuffer::new(300, 200, buf);
        let buf = Arc::new(Mutex::new(buf));
        render(300, 200, 1, buf.clone());
        assert_eq!(buf.lock().unwrap().len(), 300 * 200 * 3);
    }
}
