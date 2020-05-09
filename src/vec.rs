use std::ops::{Add, Sub, AddAssign, MulAssign, DivAssign, Div, Mul};

pub type Point3 = Vec3;
pub type Color = Vec3;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl From<(f32, f32, f32)> for Vec3 {
    fn from(t: (f32, f32, f32)) -> Self {
        Vec3 { x: t.0, y: t.1, z: t.2 }
    }
}

impl From<&[f32; 3]> for Vec3 {
    fn from(a: &[f32; 3]) -> Self {
        Vec3 { x: a[0], y: a[1], z: a[2] }
    }
}

impl Vec3 {
    pub const ZERO: Vec3 = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    pub const ONE: Vec3 = Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }


    pub fn unit(vec: &Vec3) -> Vec3 {
        vec / vec.length()
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(l: &Vec3, r: &Vec3) -> f32 {
        l.x * r.x + l.y * r.y + l.z * r.z
    }
    pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
        Vec3 {
            x: u.y * v.z - u.z * v.y,
            y: u.z * v.x - u.x * v.z,
            z: u.x * v.y - u.y * v.x,
        }
    }
}

macro_rules! impl_ops {
    ($trait:ident $vec_type:ident $op_fn:ident $op:tt) => {
        // Base impl for &Vec3 and &Vec3
        impl<'a, 'b> $trait<&'a $vec_type> for &'b $vec_type {
            type Output = $vec_type;

            fn $op_fn(self, other: &'a $vec_type) -> Self::Output {
                $vec_type {
                    x: self.x $op other.x,
                    y: self.y $op other.y,
                    z: self.z $op other.z }
            }
        }

        // &Vec3 and &ec3
        impl $trait<$vec_type> for $vec_type {
            type Output = $vec_type;
            fn $op_fn(self, other: $vec_type) -> Self::Output {
                &self $op &other
            }
        }

        // &Vec3 and Vec3
        impl<'a> $trait<&'a $vec_type> for $vec_type {
            type Output = $vec_type;
            fn $op_fn(self, other: &'a $vec_type) -> Self::Output {
                &self $op other
            }
        }

        // Vec3 and &Vec3
        impl<'a> $trait<$vec_type> for &$vec_type {
            type Output = $vec_type;
            fn $op_fn(self, other: $vec_type) -> Self::Output {
                self $op &other
            }
        }

        // &Vec * f32
        impl<'a> $trait<f32> for &'a $vec_type {
            type Output = $vec_type;

            fn $op_fn(self, other: f32) -> Self::Output {
                $vec_type {
                    x: self.x $op other,
                    y: self.y $op other,
                    z: self.z $op other }
            }
        }
        // Vec * f32
        impl $trait<f32> for $vec_type {
            type Output = $vec_type;
            fn $op_fn(self, other: f32) -> Self::Output {
                &self $op other
            }
        }

    }
}

impl_ops!(Add Vec3 add +);
impl_ops!(Sub Vec3 sub -);
impl_ops!(Mul Vec3 mul *);
impl_ops!(Div Vec3 div /);


impl<'a> AddAssign<&'a Vec3> for Vec3 {
    fn add_assign(&mut self, other: &'a Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl<'a> MulAssign<&'a Vec3> for Vec3 {
    fn mul_assign(&mut self, other: &'a Vec3) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl<'a> DivAssign<&'a Vec3> for Vec3 {
    fn div_assign(&mut self, other: &'a Vec3) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
}

pub fn write_color(writer: &mut impl std::fmt::Write, clr: &Color) -> std::fmt::Result {
    writeln!(writer, "{} {} {}",
             (255.999 * clr.x) as u32,
             (255.999 * clr.y) as u32,
             (255.999 * clr.z) as u32,
    )
}
