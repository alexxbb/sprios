use renderer::{render, Camera, Lambertian, Sphere, Vec3, Point3};
pub use renderer::World;
use rand;
use rand::Rng;

pub fn world_ivan(loc: &Vec3, rad: f32, splits: u32, recur: u32) -> World {
    fn recurse(world: &mut World, loc: &Vec3, rad: f32, splits: u32, recur: u32) {
        let mut rng = rand::thread_rng();
        for _ in 0..splits {
            let center = Vec3::random_in_unit_sphere().unit() * rad * 1.5 + loc;
            world.add(Sphere::new(
                center.clone(),
                rad * 0.5,
                Box::new(Lambertian { color: (rng.gen(), rng.gen(), rng.gen()).into() }),
            ));
            if recur > 0 {
                recurse(world, &center, rad * 0.5, splits, recur - 1);
            }
        }
    }
    let mut world = World::new();
    // // Globe sphere
    world.add(Sphere::new(
        (0.0, -100.5, -1.0),
        100.0,
        Box::new(Lambertian {
            color: (0.5, 0.5, 0.5).into(),
        }),
    ));
    recurse(&mut world, loc, rad, splits, recur);
    world
}

pub fn world_book() -> World {
    let mut world = World::new();
    // // Globe sphere
    world.add(Sphere::new(
        (0.0, -100.5, -1.0),
        100.0,
        Box::new(Lambertian {
            color: (0.5, 0.5, 0.5).into(),
        }),
    ));
    // Red
    world.add(Sphere::new(
        (-1.0, 0.0, -1.0),
        0.5,
        Box::new(Lambertian {
            color: (0.9, 0.1, 0.1).into(),
        }),
    ));
    // Green
    world.add(Sphere::new(
        (0.0, 0.0, -1.0),
        0.5,
        Box::new(Lambertian {
            color: (0.1, 0.9, 0.1).into(),
        }),
    ));
    // Blue
    world.add(Sphere::new(
        (1.0, 0.0, -1.0),
        0.5,
        Box::new(Lambertian {
            color: (0.1, 0.1, 0.9).into(),
        }),
    ));
    world
}
