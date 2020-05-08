use std::ops::{Add, AddAssign, MulAssign, DivAssign, Div, Mul};
use std::fmt::{Display, Formatter, Result, Write};

pub type Point3 = Vec3;
pub type Color = Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn zero() -> Vec3 {
        Vec3 { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn one() -> Vec3 {
        Vec3 { x: 1.0, y: 1.0, z: 1.0 }
    }
    pub fn unit(&self) -> Vec3 {
        self / self.length()
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

impl<'a, 'b> Add<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn add(self, other: &'b Vec3) -> Self::Output {
        Vec3 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl<'a> AddAssign<&'a Vec3> for Vec3 {
    fn add_assign(&mut self, other: &'a Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Mul<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn mul(self, other: &Vec3) -> Self::Output {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<f32> for &Vec3 {
    type Output = Vec3;

    fn mul(self, mul: f32) -> Self::Output {
        Vec3 { x: self.x * mul, y: self.y * mul, z: self.z * mul }
    }
}

impl<'a> MulAssign<&'a Vec3> for Vec3 {
    fn mul_assign(&mut self, other: &'a Vec3) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}


impl Div<f32> for &Vec3 {
    type Output = Vec3;

    fn div(self, div: f32) -> Self::Output {
        Vec3 { x: self.x / div, y: self.y / div, z: self.z / div }
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
