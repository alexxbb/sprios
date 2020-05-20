use std::fmt::{Display, Formatter};

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
    pub width: u32,
    pub height: u32,
    pub bucket_size: u32,
    cursor: (u32, u32),
}

impl BucketGrid {
    pub fn new(grid_width: u32, grid_height: u32, bucket_size: u32) -> BucketGrid {
        BucketGrid {
            width: grid_width,
            height: grid_height,
            bucket_size,
            cursor: (0, 0),
        }
    }
}


impl Iterator for BucketGrid {
    type Item = Bucket;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor.1  == self.height{
           return None
        }
        let mut bucket = Bucket {
            top_left: (self.cursor.0, self.cursor.1),
            bottom_right: (self.cursor.0 + self.bucket_size, self.cursor.1 + self.bucket_size),
        };
        if self.cursor.0 < self.width {
            self.cursor.0 += self.bucket_size;
            if self.cursor.0 >= self.width {
                self.cursor.0 = 0;
                self.cursor.1 += self.bucket_size;
                if self.cursor.1 > self.height {
                    self.cursor.1 = self.height
                }
                bucket.bottom_right.0 = self.width;
            }
        }
        Some(bucket)
    }
}

fn main() {}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(BucketGrid::new(9, 5, 3).count(), 6);
        assert_eq!(BucketGrid::new(10, 5, 4).count(), 6);
        assert_eq!(BucketGrid::new(10, 5, 5).count(), 2);
        assert_eq!(BucketGrid::new(25, 5, 5).count(), 5);
        assert_eq!(BucketGrid::new(20, 8, 4).count(), 10);
        assert_eq!(BucketGrid::new(20, 8, 8).count(), 3);
    }
}