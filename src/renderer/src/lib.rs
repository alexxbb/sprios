mod bbox;
mod buckets;
mod bvh;
mod camera;
mod hittable;
mod imagebuffer;
mod material;
mod ray;
mod sampler;
mod settings;
mod sphere;
mod utils;
mod vec;
mod world;
mod errors;

use crate::buckets::Bucket;
use crate::buckets::BucketGrid;
use crate::utils::Clip;
pub use camera::Camera;
pub use material::*;
pub use ray::Ray;
pub use sampler::{create_sampler, Distribution, PureRandom};
pub use settings::{RenderSettings, SettingsBuilder};
pub use sphere::Sphere;
pub use vec::{Color, Point3, Vec3};
pub use world::World;

use crate::hittable::HitRecord;
use rand::SeedableRng;
use rand::Rng;
use std::collections::VecDeque;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use threadpool::ThreadPool;

#[derive(Copy, Clone, Debug)]
pub struct RenderStats {
    pub render_time: f64,
    pub mrays: f64,
    pub fps: f64,
    pub num_ray_shot: u64,
    pub num_ray_hits: u64,
}

#[derive(Copy, Clone, Debug)]
pub struct SampleStat {
    pub sample: u32,
}

#[derive(Copy, Clone, Debug)]
pub enum RenderEvent {
    Completed(RenderStats),
    SampleDone(SampleStat),
    Percent(u8),
}

pub struct RayStat {
    pub num_ray_hits: AtomicU64,
}

impl RayStat {
    #[inline]
    pub fn add_hit(&self) {
        self.num_ray_hits.fetch_add(1, Ordering::Relaxed);
    }
}

#[allow(non_upper_case_globals)]
static ray_stat: RayStat = RayStat {
    num_ray_hits: AtomicU64::new(0),
};

fn ray_color(ray: &Ray, world: &World, depth: u32, rng: &mut rand::rngs::SmallRng) -> Color {
    if depth == 0 {
        return Color::ZERO;
    }
    let tmp_mat = Lambertian { color: Color::ZERO };
    let mut rec = HitRecord::new(&tmp_mat);

    use crate::hittable::Hittable;
    if world.hit(ray, 0.001, f32::INFINITY, &mut rec) {
        ray_stat.add_hit();
        if let Some(ray) = rec.mat.scatter(ray, &rec, Some(rng)) {
            return rec.mat.color() * ray_color(&ray, world, depth - 1, rng);
        }
        return Color::ZERO;
    }
    let dir = ray.direction.unit();
    let t = 0.5 * (dir.y + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + &world.background * t
}

pub fn render<EV>(
    settings: RenderSettings,
    image_ptr: Arc<AtomicPtr<f32>>,
    num_threads: usize,
    world: Arc<World>,
    event: EV,
) -> RenderStats
where
    EV: Fn(RenderEvent) + Send + Sync + 'static,
{
    const MAX_DEPTH: u32 = 10;
    let num_samples = settings.samples.pow(2) as usize;
    let timer = Instant::now();
    let pool = ThreadPool::new(num_threads);
    let event = Arc::new(event);
    let grid = BucketGrid::new(settings.width, settings.height, settings.bucket);
    let broker = Arc::new(Mutex::new(VecDeque::new()));
    for s in 1..=num_samples {
        let total_buckets = {
            let mut b = broker.lock().unwrap();
            b.clear();
            b.extend(grid.buckets());
            b.len() as u32
        };
        for _ in 0..pool.max_count() {
            let event = Arc::clone(&event);
            let broker = Arc::clone(&broker);
            let image_ptr = Arc::clone(&image_ptr);
            let world = Arc::clone(&world);
            pool.execute(move || loop {
                let mut broker = broker.lock().unwrap();
                let bucket = broker.pop_front();
                let buckets_left = broker.len() as u32;
                drop(broker);
                if bucket.is_none() {
                    break;
                }
                let bucket = bucket.unwrap();
                event(RenderEvent::Percent(
                    ((1.0 - buckets_left as f32 / total_buckets as f32) * 100.0) as u8,
                ));
                let rng = rand::rngs::SmallRng::from_entropy();
                // let sampler = create_sampler(num_samples, settings.distribution, rng);
                let mut rng = rand::rngs::SmallRng::from_entropy();
                // let mut samples_iter = sampler.samples();
                for (y, x) in bucket.pixels() {
                    // let s = samples_iter.next().unwrap();
                    let (sx, sy) = rng.gen::<(f32, f32)>();
                    let u = (x as f32 + sx) / (settings.width - 1) as f32;
                    let v = ((settings.height - y) as f32 + sy) / (settings.height - 1) as f32;
                    let ray = world.camera.get_ray(u, v, &mut rng);
                    let clr = ray_color(&ray, &world, MAX_DEPTH, &mut rng);
                    let idx = ((y * settings.width + x) * 3) as usize;
                    let ptr = image_ptr.load(Ordering::Relaxed);
                    unsafe {
                        let ptr = ptr.add(idx);
                        ptr.write(ptr.read() + clr.x);
                        let ptr = ptr.add(1);
                        ptr.write(ptr.read() + clr.y);
                        let ptr = ptr.add(1);
                        ptr.write(ptr.read() + clr.z);
                    }
                }
            });
        }
        std::thread::sleep(std::time::Duration::from_millis(50));
        pool.join();
        event(RenderEvent::SampleDone(SampleStat { sample: s as u32 }));
    }
    let render_time = timer.elapsed().as_secs_f64();
    let fps = 1.0 / render_time;
    let num_ray_shot = settings.width as u128 * settings.height as u128 * num_samples as u128;
    let mrays = (num_ray_shot as f64 * fps) / 1.0e6;
    RenderStats {
        render_time,
        mrays,
        fps,
        num_ray_shot: num_ray_shot as u64,
        num_ray_hits: ray_stat.num_ray_hits.load(Ordering::Relaxed),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{render, Arc};
    pub use settings::*;
    use std::sync::atomic::AtomicPtr;

    #[test]
    fn test_render() {
        let mut buf = Vec::<f32>::new();
        buf.resize(300 * 200 * 3, 0.0);
        let img_ptr = Arc::new(AtomicPtr::new(buf.as_mut_ptr()));
        let mut world = World::new();
        world.add(Arc::new(Sphere::new(
            (0.0, -100.5, -1.0),
            100.0,
            Some(Box::new(Lambertian {
                color: (0.5, 0.5, 0.5).into(),
            }),
        ))));
        world.camera = Camera::new(
            Point3::new(0.0, 0.0, 2.0),
            Point3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 1.0, 0.0),
            40,
            300 as f32 / 200 as f32,
            0.0,
            f32::INFINITY,
        );
        let world = Arc::new(world);
        let set = SettingsBuilder::new().samples(1).size(300, None).build();
        render(set, img_ptr, 2, world, |_| {});
        assert_eq!(buf.len(), 300 * 200 * 3);
    }
}
