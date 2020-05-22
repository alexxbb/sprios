mod camera;
mod hittable;
mod imagebuffer;
mod material;
mod ray;
mod sphere;
mod utils;
mod buckets;
mod world;
mod vec;

pub use camera::Camera;
pub use imagebuffer::ImageBuffer;
pub use material::{Lambertian, Metal};
use rand::{Rng, SeedableRng};
pub use ray::Ray;
use sphere::Sphere;
use buckets::Bucket;
use std::ops::DerefMut;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
pub use vec::{Color, Vec3};
use std::time::{Duration, Instant};
use crate::buckets::BucketGrid;
use std::collections::VecDeque;
use threadpool::ThreadPool;
use world::World;
use std::sync::atomic::{Ordering, AtomicPtr};
use std::thread;
use crate::utils::Clip;

// type Buffer = Arc<Mutex<ImageBuffer>>;

#[derive(Copy, Clone)]
pub struct RenderStats {
    pub render_time: f64,
    pub mrays: f64,
    pub fps: f64,
}

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

fn world() -> World {
    let mut world = World::new();
    // // Globe sphere
    world.add(Sphere::new(
        (0.0, -100.5, -1.0),
        100.0,
        Arc::new(Lambertian {
            color: (0.5, 0.5, 0.5).into(),
        }),
    ));
    // Red
    world.add(Sphere::new(
        (-1.0, 0.0, -1.0),
        0.5,
        Arc::new(Lambertian {
            color: (0.9, 0.1, 0.1).into(),
        }),
    ));
    // Green
    world.add(Sphere::new(
        (0.0, 0.0, -1.0),
        0.5,
        Arc::new(Lambertian {
            color: (0.1, 0.9, 0.1).into(),
        }),
    ));
    // Blue
    world.add(Sphere::new(
        (1.0, 0.0, -1.0),
        0.5,
        Arc::new(Lambertian {
            color: (0.1, 0.1, 0.9).into(),
        }),
    ));
    world
}

pub fn render<F>(width: u32, height: u32, samples: u32, bucket: u32, image_ptr: Arc<AtomicPtr<u8>>, progress: F) -> RenderStats
    where
        F: Fn(u32) + Send + Sync + 'static
{
    const MAX_DEPTH: u32 = 10;
    let world = Arc::new(world());
    let camera = Arc::new(Camera::new());
    let buckets = BucketGrid::new(width, height, bucket);
    let mut broker: VecDeque<Bucket> = std::collections::VecDeque::new();
    broker.extend(buckets);
    let total_buckets = broker.len() as u32;
    let broker = Arc::new(Mutex::new(broker));
    let progress = Arc::new(progress);
    let timer = Instant::now();
    let pool = ThreadPool::new(4);
    for _ in 0..pool.max_count() {
        let broker = Arc::clone(&broker);
        let image_ptr = Arc::clone(&image_ptr);
        let world = Arc::clone(&world);
        let camera = Arc::clone(&camera);
        let progress = Arc::clone(&progress);
        pool.execute(move || {
            let mut rng = rand::rngs::SmallRng::from_entropy();
            loop {
                let mut broker = broker.lock().unwrap();
                let bucket = broker.pop_front();
                let buckets_left = broker.len() as u32;
                drop(broker);
                match bucket {
                    Some(bucket) => {
                        // eprintln!("{:?} with bucket {}", current(), &bucket);
                        progress(((1.0 - buckets_left as f32 / total_buckets as f32) * 100.0) as u32);
                        // This is not right! The buffer is locked until this bucket finished.
                        let ptr = image_ptr.load(Ordering::Relaxed);
                        for (y, x) in bucket.pixels() {
                            let mut pixel_color = Color::ZERO;
                            for _ in 0..samples {
                                let u = (x as f32 + rng.gen::<f32>()) / (width - 1) as f32;
                                let v = ((height - y) as f32 + rng.gen::<f32>()) / (height - 1) as f32;
                                let ray = camera.get_ray(u, v);
                                pixel_color += &ray_color(&ray, &world, MAX_DEPTH);
                            }
                            let idx = ((y * width + x) * 3) as usize;
                            let scale = 1.0 / samples as f32;
                            let r = (pixel_color.x * scale).sqrt();
                            let g = (pixel_color.y * scale).sqrt();
                            let b = (pixel_color.z * scale).sqrt();

                            unsafe {
                                ptr.add(idx + 0).write((256.0 * r.clip(0.0, 0.999)) as u8);
                                ptr.add(idx + 1).write((256.0 * g.clip(0.0, 0.999)) as u8);
                                ptr.add(idx + 2).write((256.0 * b.clip(0.0, 0.999)) as u8);
                            }
                        }
                    }
                    None => break
                }
            }
        });
    }
    pool.join();
    let render_time = timer.elapsed().as_secs_f64();
    let fps = 1.0 / render_time;
    let mrays = ((width as u128 * height as u128 * samples as u128) as f64 * fps) / 1.0e6;
    RenderStats{render_time, mrays, fps }
}

#[cfg(test)]
mod tests {
    use crate::{render, Arc, ImageBuffer, Mutex};

    #[test]
    fn test_render() {
        let buf = Vec::<u8>::new();
        let mut buf = Arc::new(Vec::new());
        buf.resize(300 * 200 * 3, 0);
        render(300, 200, 1, 10, buf.clone(), |_, _| {});
        assert_eq!(buf.len(), 300 * 200 * 3);
    }
}
