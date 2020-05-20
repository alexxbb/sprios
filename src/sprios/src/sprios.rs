use std::fmt::{Display, Formatter};
use std::sync::{mpsc, Mutex, Arc};

// use rayon::prelude::*;
use threadpool::ThreadPool;
use std::collections::VecDeque;

//
#[derive(Copy, Clone)]
struct Bucket {
    top_left: (u32, u32),
    bottom_right: (u32, u32),
}


impl Display for Bucket {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Bucket[{:?},{:?}]", self.top_left, self.bottom_right)
    }
}

struct BucketGrid {
    pub height: u32,
    pub width: u32,
    pub bucket_size: u32,
    cursor: (u32, u32),
    index: u32,
    num_buckets: u32,
}

impl BucketGrid {
    pub fn new(grid_height: u32, grid_width: u32, bucket_size: u32) -> BucketGrid {
        let num_buckets = (grid_height * grid_width) as f32 / bucket_size.pow(2) as f32;
        let num_buckets = num_buckets.ceil() as u32;
        BucketGrid {
            height: grid_height,
            width: grid_width,
            bucket_size,
            cursor: (0, 0),
            index: 0,
            num_buckets,
        }
    }
}


impl Iterator for BucketGrid {
    type Item = Bucket;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.num_buckets {
            return None;
        }
        let mut bucket = Bucket {
            top_left: (self.cursor.0, self.cursor.1),
            bottom_right: (self.cursor.0 + self.bucket_size, self.cursor.1 + self.bucket_size),
        };
        if self.cursor.0 + self.bucket_size >= self.width {
            self.cursor.0 = 0;
            self.cursor.1 += self.bucket_size;
            bucket.bottom_right.0 = self.width;
        } else {
            self.cursor.0 += self.bucket_size;
        }
        self.index += 1;
        Some(bucket)
    }
}

fn main() {
    const W: u32 = 50;
    const H: u32 = 50;

    let buffer = Arc::new(Mutex::new(Vec::<u8>::with_capacity((W * H) as usize)));
    let buckets = BucketGrid::new(50, 50, 10);
    let mut broker:VecDeque<Bucket> = std::collections::VecDeque::new();
    broker.extend(buckets);
    let broker = Arc::new(Mutex::new(broker));

    let mut pool = ThreadPool::new(2);
    for i in 0..2 {
        let broker = Arc::clone(&broker);
        let buffer = Arc::clone(&buffer);
        pool.execute(move || {
            use std::thread::current;
            loop {
                let mut broker = broker.lock().unwrap();
                match broker.pop_front() {
                    Some(bucket) => {
                        let mut buffer = buffer.lock().unwrap();
                        for w in bucket.top_left.0..bucket.bottom_right.0 {
                            for h in bucket.top_left.1..bucket.bottom_right.1 {
                                // calc
                                let v = (w * h) as u8;
                                buffer.push(v);
                            }
                        }
                    }
                    None => break
                }
            }
        })
    }
    pool.join();
    println!("{:?}", buffer.lock().unwrap())


    // let res: Vec<u32> = (0..w).into_par_iter().zip(0..h).map(|t| {
    //     t.0 * t.1
    // }).collect();


    // println!("{:?}", buf);
}