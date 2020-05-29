mod buckets;
mod camera;
mod hittable;
mod imagebuffer;
mod material;
mod ray;
mod sphere;
mod utils;
mod vec;
mod world;

use crate::buckets::BucketGrid;
use crate::utils::Clip;
use buckets::Bucket;
pub use camera::Camera;
pub use imagebuffer::ImageBuffer;
pub use material::*;
use rand::{Rng, SeedableRng};
pub use ray::Ray;
use std::collections::VecDeque;


use std::sync::atomic::{AtomicPtr, Ordering};
use std::sync::{Arc, Mutex};

use std::time::{Instant};
use threadpool::ThreadPool;
pub use vec::{Color, Vec3, Point3};
pub use world::World;
pub use sphere::Sphere;
use std::borrow::Cow;

// type Buffer = Arc<Mutex<ImageBuffer>>;

#[derive(Copy, Clone, Debug)]
pub struct RenderStats {
    pub render_time: f64,
    pub mrays: f64,
    pub fps: f64,
}

fn ray_color(ray: &Ray, world: &World, depth: u32, rng: &mut rand::rngs::SmallRng) -> Color {
    if depth == 0 {
        // Max recursion depth reached
        return Color::ZERO;
    }

    if let Some(rec) = world.hit(ray, 0.001, f32::INFINITY) {
        if let Some(ray) = rec.mat.scatter(ray, &rec, Some(rng)) {
            return rec.mat.color() * ray_color(&ray, world, depth - 1, rng);
        }
        return Color::ZERO;
    }
    let dir = ray.direction.unit();
    let t = 0.5 * (dir.y + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}


pub fn render<F>(
    width: u32,
    height: u32,
    samples: u32,
    bucket: u32,
    image_ptr: Arc<AtomicPtr<u8>>,
    pool: Option<&threadpool::ThreadPool>,
    world: Arc<World>,
    camera: Arc<Camera>,
    progress: F,
) -> RenderStats
    where
        F: Fn(u32) + Send + Sync + 'static,
{
    const MAX_DEPTH: u32 = 10;
    let buckets = BucketGrid::new(width, height, bucket);
    let mut broker: VecDeque<Bucket> = std::collections::VecDeque::new();
    broker.extend(buckets);
    let total_buckets = broker.len() as u32;
    let samples = samples.pow(2);
    let samples_scale = 1.0 / samples as f32;
    let broker = Arc::new(Mutex::new(broker));
    let progress = Arc::new(progress);
    let timer = Instant::now();
    let pool = pool.map_or_else(|| Cow::Owned(ThreadPool::new(10)), |v| Cow::Borrowed(v));
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
                        progress(
                            ((1.0 - buckets_left as f32 / total_buckets as f32) * 100.0) as u32,
                        );
                        let ptr = image_ptr.load(Ordering::Relaxed);
                        for (y, x) in bucket.pixels() {
                            let mut pixel_color = Color::ZERO;
                            for _ in 0..samples {
                                let u = (x as f32 + rng.gen::<f32>()) / (width - 1) as f32;
                                let v =
                                    ((height - y) as f32 + rng.gen::<f32>()) / (height - 1) as f32;
                                let ray = camera.get_ray(u, v);
                                pixel_color += &ray_color(&ray, &world, MAX_DEPTH, &mut rng);
                            }
                            let idx = ((y * width + x) * 3) as usize;
                            let r = (pixel_color.x * samples_scale).sqrt();
                            let g = (pixel_color.y * samples_scale).sqrt();
                            let b = (pixel_color.z * samples_scale).sqrt();

                            unsafe {
                                ptr.add(idx + 0).write((256.0 * r.clip(0.0, 0.999)) as u8);
                                ptr.add(idx + 1).write((256.0 * g.clip(0.0, 0.999)) as u8);
                                ptr.add(idx + 2).write((256.0 * b.clip(0.0, 0.999)) as u8);
                            }
                        }
                    }
                    None => break,
                }
            }
        });
    }
    pool.join();
    let render_time = timer.elapsed().as_secs_f64();
    let fps = 1.0 / render_time;
    let mrays = ((width as u128 * height as u128 * samples as u128) as f64 * fps) / 1.0e6;
    RenderStats {
        render_time,
        mrays,
        fps,
    }
}

#[cfg(test)]
mod tests {
    use crate::{render, Arc};
    use super::*;
    use std::sync::atomic::AtomicPtr;

    #[test]
    fn test_render() {
        let mut buf = Vec::<u8>::new();
        buf.resize(300 * 200 * 3, 0);
        let img_ptr = Arc::new(AtomicPtr::new(buf.as_mut_ptr()));
        let mut world = World::new();
        world.add(Sphere::new(
            (0.0, -100.5, -1.0),
            100.0,
            Box::new(Lambertian {
                color: (0.5, 0.5, 0.5).into(),
            }),
        ));
        let world = Arc::new(world);
        let camera = Arc::new(Camera::new(
            Point3::new(0.0, 0.0, 2.0),
            Point3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 1.0, 0.0),
            40,
            300 as f32 / 200 as f32));
        render(300, 200, 1, 10, img_ptr, None, world, camera, |_| {});
        assert_eq!(buf.len(), 300 * 200 * 3);
    }
}
