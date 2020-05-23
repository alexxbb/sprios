use std::fmt::{Display, Formatter};

#[derive(Copy, Clone)]
pub struct Bucket {
    pub top_left: (u32, u32),
    pub bottom_right: (u32, u32),
}

impl Bucket {
    pub fn pixels(self) -> BucketPixels {
        self.into_iter()
    }
}

pub struct BucketPixels {
    bucket: Bucket,
    cursor: (u32, u32),
}

impl Iterator for BucketPixels {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor == self.bucket.bottom_right {
            return None;
        }
        for y in self.cursor.1..self.bucket.bottom_right.1 {
            for x in self.cursor.0..self.bucket.bottom_right.0 {
                self.cursor.0 = x + 1;
                return Some((y, x));
            }
            self.cursor.1 = y + 1;
            self.cursor.0 = self.bucket.top_left.0;
        }
        None
    }
}

impl IntoIterator for Bucket {
    type Item = (u32, u32);
    type IntoIter = BucketPixels;

    fn into_iter(self) -> Self::IntoIter {
        BucketPixels {
            bucket: self,
            cursor: self.top_left,
        }
    }
}

impl Display for Bucket {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Bucket[{:?},{:?}]", self.top_left, self.bottom_right)
    }
}

pub struct BucketGrid {
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
        if self.cursor.1 == self.height {
            return None;
        }
        let top_left = self.cursor;
        let mut bottom_right = (
            self.cursor.0 + self.bucket_size,
            self.cursor.1 + self.bucket_size,
        );
        // Advance forward
        self.cursor.0 += self.bucket_size;
        // Overflow
        if self.cursor.0 >= self.width {
            // Advance down
            self.cursor.0 = 0;
            self.cursor.1 += self.bucket_size;
            // Clip the bucket to width
            bottom_right.0 = self.width;
        }
        // Check for overflow
        if self.cursor.1 > self.height {
            // bottom_right.1 = self.height;
            self.cursor.1 = self.height;
        }
        if bottom_right.1 > self.height {
            bottom_right.1 = self.height;
        }
        Some(Bucket {
            top_left,
            bottom_right,
        })
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
        assert_eq!(BucketGrid::new(20, 11, 3).count(), 28);
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Bucket {
                top_left: (0, 0),
                bottom_right: (5, 5)
            }
            .into_iter()
            .count(),
            25
        );
        assert_eq!(
            Bucket {
                top_left: (10, 10),
                bottom_right: (15, 15)
            }
            .into_iter()
            .count(),
            25
        );
    }
}
