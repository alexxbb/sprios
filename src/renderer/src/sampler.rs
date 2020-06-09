use rand;
use crate::vec::{Point3};
use rand::{Rng};
use rand::distributions::{Uniform};

pub trait Sampler<R: Rng> {
    fn samples(&self) -> SamplesIter<R>;
}

struct SamplerData {
    num_sets: usize,
    num_samples: usize,
    samples: Vec<Point3>,
    shuffle_indices: Vec<usize>,
}

impl SamplerData {
    fn new(num_samples: usize, num_sets: usize, rng: &mut impl Rng) -> SamplerData {
        let total_num = num_sets * num_samples;
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

#[derive(Copy, Clone)]
pub enum Distribution {
    Random,
    Jittered,
}

pub fn create_sampler<R: 'static + Rng + Clone>(num: usize, stype: Distribution, rng: R) -> Box<dyn Sampler<R>> {
    match stype {
        Distribution::Random => Box::new(PureRandom::new(num, 83, rng)),
        Distribution::Jittered => Box::new(Jittered::new(num, 83, rng))
    }
}


pub struct SamplesIter<'a, R: Rng> {
    inner: &'a SamplerData,
    distr: Distribution,
    rng: R,
    current: usize,
    jump: usize,
}

impl<'a, R> Iterator for SamplesIter<'a, R> where R: Rng{
    type Item = &'a Point3;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current % self.inner.num_samples == 0 { // next pixel
            self.jump = (self.rng.gen::<usize>() % self.inner.num_sets) * self.inner.num_samples
        }
        self.current += 1;
        let shuffled = self.inner.shuffle_indices[self.jump + self.current % self.inner.num_samples];
        Some(&self.inner.samples[shuffled])
    }
}

pub struct PureRandom<R: Rng> {
    data: SamplerData,
    rng: R
}

pub struct Jittered<R: Rng> {
    data: SamplerData,
    rng: R
}

impl<R> PureRandom<R> where R: Rng {
    pub fn new(num_samples: usize, num_sets: usize, mut rng: R) -> PureRandom<R> {
        let mut data = SamplerData::new(num_samples, num_sets, &mut rng);
        for _ in 0..num_sets {
            for _ in 0..num_samples {
                data.samples.push(Point3::random(&mut rng))
            }
        }
        PureRandom { data, rng }
    }
}

impl<R> Sampler<R> for PureRandom<R> where R: Rng + Clone {
    fn samples(&self) -> SamplesIter<R> {
        SamplesIter { inner: &self.data, distr: Distribution::Random, current: 0, jump: 0, rng: self.rng.clone() }
    }
}

impl<R> Jittered<R> where R: Rng {
    pub fn new(num_samples: usize, num_sets: usize, mut rng: R) -> Jittered<R> {
        let mut data = SamplerData::new(num_samples, num_sets, &mut rng);
        let n = (num_samples as f32).sqrt();
        for _ in 0..num_sets {
            for j in 0..n as usize {
                for k in 0..n as usize {
                    data.samples.push(
                        Point3::new(((k as f32) + rng.gen::<f32>()) / n, ((j as f32) + rng.gen::<f32>()) / n, 0.0));
                }
            }
        }
        Jittered {
            data,
            rng
        }
    }
}

impl<R: Rng + Clone> Sampler<R> for Jittered<R> {
    fn samples(&self) -> SamplesIter<'_, R> {
        SamplesIter { inner: &self.data, distr: Distribution::Jittered, current: 0, jump: 0, rng: self.rng.clone() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let rng = rand::rngs::SmallRng::from_entropy();
        let s = create_sampler(9, Distribution::Random, rng);
        // assert_eq!(s.samples().take(9 * 83).count(), 9 * 83);
        let i: Vec<&Point3> = s.samples().take(2000).collect();
        // dbg!(&i);
    }
}