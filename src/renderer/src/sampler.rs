use rand;
use crate::vec::{Vec3, Point3};
use rand::{SeedableRng, thread_rng, Rng};
use rand::distributions::{Uniform};

pub trait Sampler {
    fn samples(&self) -> SamplesIter<'_>;
}

struct SamplerData {
    num_sets: usize,
    num_samples: usize,
    samples: Vec<Point3>,
    shuffle_indices: Vec<usize>,
}

impl SamplerData {
    fn new(num_samples: usize, num_sets: usize) -> SamplerData {
        let total_num = num_sets * num_samples;
        let mut rng = rand::thread_rng();
        let shuffle_indices: Vec<usize> = rng.sample_iter(
            Uniform::new(0, total_num)).take(total_num).collect();
        SamplerData {
            num_sets,
            num_samples,
            samples: Vec::with_capacity(total_num),
            shuffle_indices,
        }
    }
}

pub enum Distribution {
    Random,
    Jittered,
}

pub fn create_sampler(num: usize, stype: Distribution) -> Box<dyn Sampler> {
    match stype {
        Distribution::Random => Box::new(PureRandom::new(num, 83)),
        Distribution::Jittered => Box::new(Jittered::new(num, 83))
    }
}


pub struct SamplesIter<'a> {
    inner: &'a SamplerData,
    distr: Distribution,
    current: usize,
    jump: usize,
}

impl<'a> Iterator for SamplesIter<'a> {
    type Item = &'a Point3;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO Pass rng along?
        if self.current % self.inner.num_samples == 0 { // next pixel
            self.jump = (thread_rng().gen::<usize>() % self.inner.num_sets) * self.inner.num_samples
        }
        self.current += 1;
        let shuffled = self.inner.shuffle_indices[self.jump + self.current % self.inner.num_samples];
        Some(&self.inner.samples[shuffled])
    }
}

pub struct PureRandom {
    data: SamplerData,
}

pub struct Jittered {
    data: SamplerData,
}

impl PureRandom {
    pub fn new(num_samples: usize, num_sets: usize) -> PureRandom {
        let mut rng = rand::rngs::SmallRng::from_entropy();
        let mut data = SamplerData::new(num_samples, num_sets);
        for _ in 0..num_sets {
            for _ in 0..num_samples {
                data.samples.push(Point3::random(&mut rng))
            }
        }
        PureRandom { data }
    }
}

impl Sampler for PureRandom {
    fn samples(&self) -> SamplesIter<'_> {
        SamplesIter { inner: &self.data, distr: Distribution::Random, current: 0, jump: 0 }
    }
}

impl Jittered {
    pub fn new(num_samples: usize, num_sets: usize) -> Jittered {
        let mut rng = rand::rngs::SmallRng::from_entropy();
        let mut data = SamplerData::new(num_samples, num_sets);
        for _ in 0..(num_samples * num_sets) {
            data.samples.push(Point3::random(&mut rng))
        }
        Jittered {
            data
        }
    }
}

impl Sampler for Jittered {
    fn samples(&self) -> SamplesIter<'_> {
        SamplesIter { inner: &self.data, distr: Distribution::Jittered, current: 0, jump: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = create_sampler(9, Distribution::Random);
        // assert_eq!(s.samples().take(9 * 83).count(), 9 * 83);
        let i: Vec<&Point3> = s.samples().take(2000).collect();
        // dbg!(&i);
    }
}