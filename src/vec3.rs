use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    #[allow(dead_code)]
    pub const fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
    #[allow(dead_code)]
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    #[allow(dead_code)]
    pub const fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    #[allow(dead_code)]
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    #[allow(dead_code)]
    pub fn unit(&self) -> Self {
        *self / self.length()
    }
    #[allow(dead_code)]
    pub const fn dot(&self, b: Self) -> f64 {
        self.x * b.x + self.y * b.y + self.z * b.z
    }
    #[allow(dead_code)]
    pub const fn cross(&self, b: Self) -> Self {
        Self::new(
            self.y * b.z - self.z * b.y,
            self.z * b.x - self.x * b.z,
            self.x * b.y - self.y * b.x,
        )
    }
    #[allow(dead_code)]
    pub fn random(min: f64, max: f64) -> Self {
        Self::new(
            min + (max - min) * rand::random::<f64>(),
            min + (max - min) * rand::random::<f64>(),
            min + (max - min) * rand::random::<f64>(),
        )
    }
    #[allow(dead_code)]
    pub fn random_unit_vector() -> Self {
        loop {
            let p = Self::random(-1.0, 1.0);
            let lensq = p.length_squared();
            if 1e-160 < lensq && lensq <= 1.0 {
                return p / lensq.sqrt();
            }
        }
    }
    #[allow(dead_code)]
    pub fn random_on_hemisphere(normal: Vec3) -> Self {
        loop {
            let p = Self::random_unit_vector();
            if normal.dot(p) >= 0.0 {
                return p;
            } else {
                return -p;
            }
        }
    }
    #[allow(dead_code)]
    pub fn near_zero(&self) -> bool {
        const S: f64 = 1e-8;
        self.x.abs() < S && self.y.abs() < S && self.z.abs() < S
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}
impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, b: Self) -> Self::Output {
        Self::new(self.x + b.x, self.y + b.y, self.z + b.z)
    }
}
impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, b: Self) -> Self::Output {
        Self::new(self.x - b.x, self.y - b.y, self.z - b.z)
    }
}
impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, b: Self) -> Self::Output {
        Self::new(self.x * b.x, self.y * b.y, self.z * b.z)
    }
}
impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, b: f64) -> Self::Output {
        Self::new(self.x * b, self.y * b, self.z * b)
    }
}
impl Div for Vec3 {
    type Output = Vec3;
    fn div(self, b: Self) -> Self::Output {
        Self::new(self.x / b.x, self.y / b.y, self.z / b.z)
    }
}
impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, b: f64) -> Self::Output {
        Self::new(self.x / b, self.y / b, self.z / b)
    }
}
impl AddAssign for Vec3 {
    fn add_assign(&mut self, b: Self) {
        *self = *self + b;
    }
}
impl SubAssign for Vec3 {
    fn sub_assign(&mut self, b: Self) {
        *self = *self - b;
    }
}
impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, b: f64) {
        *self = *self * b;
    }
}
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, b: f64) {
        *self = *self / b;
    }
}
impl Sum for Vec3 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Vec3::zero(), |acc, x| acc + x)
    }
}
