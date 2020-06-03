use rand;
use crate::vec::{Vec3, Point3};
use rand::{SeedableRng, thread_rng, Rng};
use rand::distributions::{Uniform};

pub trait Sampler {
    fn samples(&self) -> SamplesIter<'_>;
}

struct SamplerData {
    num_sets: usize,
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
            samples: Vec::with_capacity(total_num),
            shuffle_indices,
        }
    }
}

pub enum Distribution {
    Random,
    Jittered,
}

pub fn sampler(num: usize, stype: Distribution) -> Box<dyn Sampler> {
    match stype {
        Distribution::Random => Box::new(PureRandom::new(num, 83)),
        Distribution::Jittered => Box::new(Jittered::new(num, 83))
    }
}


pub struct SamplesIter<'a> {
    inner: &'a SamplerData,
    distr: Distribution,
    current: usize,
}

impl<'a> Iterator for SamplesIter<'a> {
    type Item = &'a Point3;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.inner.samples.len() {
            None
        } else {
            self.current += 1;
            self.inner.samples.get(self.current - 1)
        }
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
        SamplesIter { inner: &self.data, distr: Distribution::Random, current: 0 }
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
        SamplesIter { inner: &self.data, distr: Distribution::Jittered, current: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = sampler(9, Distribution::Random);
        assert_eq!(s.samples().count(), 9 * 83);
    }
}