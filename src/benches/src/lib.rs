
use rand::{thread_rng, Rng, SeedableRng};
use rand::rngs::SmallRng;

pub fn small_rng() {
    let mut sum = 0.0;
    let mut rng = SmallRng::from_entropy();
    for _ in 0..100000 {
        sum += rng.gen::<f32>();
    }
}

pub fn thread_rng_() {
    let mut sum = 0.0;
    let mut rng = thread_rng();
    for _ in 0..100000 {
        sum += rng.gen::<f32>();
    }
}

