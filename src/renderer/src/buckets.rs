use std::fmt::{Display, Formatter};

#[derive(Copy, Clone)]
pub struct Bucket {
    pub(crate) top_left: (u32, u32),
    pub(crate) bottom_right: (u32, u32),
}


impl Display for Bucket {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Bucket[{:?},{:?}]", self.top_left, self.bottom_right)
    }
}

pub struct BucketGrid {
    pub height: u32,
    pub width: u32,
    pub bucket_size: u32,
    cursor: (u32, u32),
}

impl BucketGrid {
    pub fn new(grid_width: u32, grid_height: u32, bucket_size: u32) -> BucketGrid {
        BucketGrid {
            height: grid_height,
            width: grid_width,
            bucket_size,
            cursor: (0, 0),
        }
    }
}


impl Iterator for BucketGrid {
    type Item = Bucket;

    fn next(&mut self) -> Option<Self::Item> {
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
        Some(bucket)
    }
}


mod tests {
    use super::*;
    #[test]
    fn test_grid_uniform() {
        let grid = BucketGrid::new(10, 5, 5);
        assert_eq!(grid.count(), 2);
    }

    #[test]
    fn test_grid() {
        // let grid = BucketGrid::new(9, 5, 3);
        // assert_eq!(grid.count(), 6);
        let grid = BucketGrid::new(9, 5, 4);
        assert_eq!(grid.count(), 6);
        let grid = BucketGrid::new(9, 5, 5);
        assert_eq!(grid.count(), 2);
    }

}
