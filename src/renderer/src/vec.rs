use rand;
use rand::Rng;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, Index};

pub type Point3 = Vec3;
pub type Color = Vec3;

#[derive(Debug, Clone, PartialOrd, PartialEq, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl From<(f32, f32, f32)> for Vec3 {
    fn from(t: (f32, f32, f32)) -> Self {
        Vec3 {
            x: t.0,
            y: t.1,
            z: t.2,
        }
    }
}

impl From<&[f32; 3]> for Vec3 {
    fn from(a: &[f32; 3]) -> Self {
        Vec3 {
            x: a[0],
            y: a[1],
            z: a[2],
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Vec3 out of bounds!")
        }
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

    pub fn unit(&self) -> Vec3 {
        self / self.length()
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(&self, r: &Vec3) -> f32 {
        self.x * r.x + self.y * r.y + self.z * r.z
    }
    pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
        Vec3 {
            x: u.y * v.z - u.z * v.y,
            y: u.z * v.x - u.x * v.z,
            z: u.x * v.y - u.y * v.x,
        }
    }
    pub fn reflect(&self, other: &Vec3) -> Vec3 {
        self - other * 2.0 * self.dot(other)
    }
    pub fn random(rng: &mut impl rand::RngCore) -> Self {
        Self::random_in(0.0, 1.0, rng)
    }

    pub fn random_in(min: f32, max: f32, rng: &mut impl rand::RngCore) -> Self {
        Vec3 {
            x: rng.gen_range(min, max),
            y: rng.gen_range(min, max),
            z: rng.gen_range(min, max),
        }
    }

    pub fn random_in_unit_sphere(rng: &mut impl rand::RngCore) -> Self {
        loop {
            let p = Self::random(rng);
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn random_in_hemisphere(normal: &Vec3, rng: &mut impl rand::RngCore) -> Self {
        let in_unit_sphere = Vec3::random_in_unit_sphere(rng);
        if Vec3::dot(&in_unit_sphere, normal) > 0.0 {
            return in_unit_sphere;
        }
        return -in_unit_sphere;
    }

    pub fn random_unit_vector(rng: &mut impl rand::RngCore) -> Self {
        let a = rng.gen_range(0.0, 2.0 * std::f32::consts::PI);
        let z = rng.gen_range(-1.0f32, 1.0f32);
        let r = (1.0 - z * z).sqrt();
        return Vec3 {
            x: r * f32::cos(a),
            y: r * f32::sin(a),
            z,
        };
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

        // &Vec3 and &Vec3
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

impl Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

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

impl<'a> DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, other: f32) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
    }
}
