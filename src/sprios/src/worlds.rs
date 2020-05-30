use renderer::{Lambertian, Metal, Sphere, Vec3, Material, Color, Point3};
pub use renderer::World;
use rand;
use rand::Rng;

pub fn world_ivan(loc: &Vec3, rad: f32, splits: u32, recur: u32) -> World {
    fn recurse(world: &mut World, loc: &Vec3, rad: f32, splits: u32, recur: u32) {
        let mut rng = rand::thread_rng();
        for _ in 0..splits {
            let center = Vec3::random_in_unit_sphere(&mut rng).unit() * rad * 1.5 + loc;
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

pub fn final_world() -> World {
    use rand::seq::{SliceRandom, IteratorRandom};
    enum Mats {
        Lambert,
        Metal,
    }
    let mut world = World::new();
    let mut rng = rand::thread_rng();
    world.add(Sphere::new(
        (0.0, -1000.0, 0.0),
        1000.0,
        Box::new(Lambertian {
            color: (0.5, 0.5, 0.5).into(),
        }),
    ));
    for a in -11..11 {
        for b in -11..11 {
            let center = Vec3::new(a as f32 + 0.9 * rng.gen::<f32>(), 0.2, b as f32 + 0.9f32 * rng.gen::<f32>());
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let mat = match [Mats::Lambert, Mats::Metal].choose(&mut rng).unwrap() {
                    Mats::Lambert => Box::new(Lambertian { color: Color::random(&mut rng) }) as Box<dyn Material>,
                    Mats::Metal => Box::new(Metal { color: Color::random_in(0.5, 1.0, &mut rng), fuzz: rng.gen_range(0.0, 0.5) }) as Box<dyn Material>,
                };
                world.add(Sphere { center, radius: 0.2, material: mat });
            }
        }
    }
    world.add(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, Box::new(Lambertian { color: (0.4, 0.2, 0.1).into() })));
    world.add(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, Box::new(Metal { color: (0.7, 0.6, 0.5).into(), fuzz: 0.0 })));
    world
}
