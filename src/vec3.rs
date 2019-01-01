#![allow(dead_code)]
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    pub e: [f32; 3],
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }

    pub fn dot(&self, other: Vec3) -> f32 {
        self.e[0] * other.e[0] + self.e[1] * other.e[1] + self.e[2] * other.e[2]
    }

    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[1] * other.e[2] - other.e[1] * self.e[2],
                self.e[2] * other.e[0] - other.e[2] * self.e[0],
                self.e[0] * other.e[1] - other.e[0] * self.e[1],
            ],
        }
    }

    pub fn squared_lenght(&self) -> f32 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn lenght(&self) -> f32 {
        self.squared_lenght().sqrt()
    }

    pub fn normalize(&mut self) {
        let length = self.lenght();

        *self /= length;
    }

    pub fn normalized(&self) -> Vec3 {
        *self / self.lenght()
    }
}

/**
 * Data acessors
 */
impl Vec3 {
    pub fn x(&self) -> f32 {
        self.e[0]
    }

    pub fn y(&self) -> f32 {
        self.e[1]
    }

    pub fn z(&self) -> f32 {
        self.e[2]
    }

    pub fn r(&self) -> f32 {
        self.e[0]
    }

    pub fn g(&self) -> f32 {
        self.e[1]
    }

    pub fn b(&self) -> f32 {
        self.e[2]
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self {
        Vec3 {
            e: [
                self.e[0] + rhs.e[0],
                self.e[1] + rhs.e[1],
                self.e[2] + rhs.e[2],
            ],
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Self) -> Self {
        Vec3 {
            e: [
                self.e[0] - other.e[0],
                self.e[1] - other.e[1],
                self.e[2] - other.e[2],
            ],
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.e[0] -= rhs.e[0];
        self.e[1] -= rhs.e[1];
        self.e[2] -= rhs.e[2];
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Self) -> Self {
        Vec3 {
            e: [
                self.e[0] * other.e[0],
                self.e[1] * other.e[1],
                self.e[2] * other.e[2],
            ],
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, n: f32) -> Self {
        Vec3 {
            e: [self.e[0] * n, self.e[1] * n, self.e[2] * n],
        }
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.e[0] *= rhs.e[0];
        self.e[1] *= rhs.e[1];
        self.e[2] *= rhs.e[2];
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}

impl Div for Vec3 {
    type Output = Vec3;
    fn div(self, other: Self) -> Self {
        Vec3 {
            e: [
                self.e[0] / other.e[0],
                self.e[1] / other.e[1],
                self.e[2] / other.e[2],
            ],
        }
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self.e[0] /= rhs.e[0];
        self.e[1] /= rhs.e[1];
        self.e[2] /= rhs.e[2];
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, n: f32) -> Self {
        Vec3 {
            e: [self.e[0] / n, self.e[1] / n, self.e[2] / n],
        }
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self.e[0] /= rhs;
        self.e[1] /= rhs;
        self.e[2] /= rhs;
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;
    fn index(&self, i: usize) -> &f32 {
        &self.e[i]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut f32 {
        &mut self.e[i]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partial_eq() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(1.0, 2.0, 3.0);
        let v3 = Vec3::new(1.0, 2.0, 3.5);

        assert_eq!(v1, v2);
        assert_eq!(v2, v1);
        assert_ne!(v1, v3);
    }

    #[test]
    fn add_operator() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(1.5, 2.0, 3.0);
        let v3 = Vec3::new(0.0, 0.0, 0.0);

        assert_eq!(v1 + v2, Vec3 { e: [2.5, 4.0, 6.0] });
        assert_eq!(v1 + v3, v1);
    }

    #[test]
    fn add_assign_operator() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(1.0, 2.0, 3.0);
        let v3 = Vec3::new(0.0, 0.0, 0.0);

        v1 += v3;
        assert_eq!(v1, v1 + v3);

        v1 += v2;
        assert_eq!(v1, Vec3 { e: [2.0, 4.0, 6.0] });
    }

    #[test]
    fn sub_operator() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(1.5, 2.0, 3.0);
        let v3 = Vec3::new(0.0, 0.0, 0.0);

        assert_eq!(
            v1 - v2,
            Vec3 {
                e: [-0.5, 0.0, 0.0]
            }
        );
        assert_eq!(v1 - v3, v1);
    }

    #[test]
    fn sub_assign_operator() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(1.0, 4.0, 3.0);
        let v3 = Vec3::new(0.0, 0.0, 0.0);

        v1 -= v3;
        assert_eq!(v1, v1 - v3);

        v1 -= v2;
        assert_eq!(
            v1,
            Vec3 {
                e: [0.0, -2.0, 0.0]
            }
        );
    }

    #[test]
    fn mul_operator() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(1.5, 2.0, 3.0);
        let v3 = Vec3::new(0.0, 0.0, 0.0);

        assert_eq!(v1 * v2, Vec3 { e: [1.5, 4.0, 9.0] });
        assert_eq!(v1 * v3, v3);
    }

    #[test]
    fn mul_assign_operator() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(1.0, 4.0, 3.0);
        let v3 = Vec3::new(0.0, 0.0, 0.0);

        v1 *= v3;
        assert_eq!(v1, v1 * v3);

        v1 = Vec3::new(1.0, 2.0, 3.0);
        v1 *= v2;
        assert_eq!(v1, Vec3 { e: [1.0, 8.0, 9.0] });
    }

    #[test]
    fn div_operator() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(1.5, 2.0, 3.0);

        assert_eq!(
            v1 / v2,
            Vec3 {
                e: [1.0 / 1.5, 1.0, 1.0]
            }
        );
    }

    #[test]
    fn div_assign_operator() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(1.5, 4.0, 3.0);

        v1 /= v2;
        assert_eq!(
            v1,
            Vec3 {
                e: [1.0 / 1.5, 0.5, 1.0]
            }
        );
    }

    #[test]
    fn index_operator() {
        let v = Vec3::new(1.0, 2.0, 3.0);

        assert!((v[0] - 1.0).abs() < std::f32::EPSILON);
        assert!((v[1] - 2.0).abs() < std::f32::EPSILON);
        assert!((v[2] - 3.0).abs() < std::f32::EPSILON);
    }
    #[test]
    fn index_mut_operator() {
        let mut v = Vec3::new(1.0, 2.0, 3.0);

        v[0] = 2.0;

        assert!((v[0] - 2.0).abs() < std::f32::EPSILON);
        assert!((v[1] - 2.0).abs() < std::f32::EPSILON);
        assert!((v[2] - 3.0).abs() < std::f32::EPSILON);
    }

    #[test]
    fn dot_product() {
        let v1 = Vec3::new(1.0, 0.0, 0.0);
        let v2 = Vec3::new(0.0, 1.0, 0.0);
        let v3 = Vec3::new(0.0, 0.0, 0.0);

        assert!(v1.dot(v2) == 0.0);
        assert!(v2.dot(v1) == 0.0);
        assert!(v1.dot(v3) == 0.0);
        assert!((v1.dot(v1) - 1.0).abs() < std::f32::EPSILON);
    }

    #[test]
    fn cross_product() {
        let v1 = Vec3::new(1.0, 0.0, 0.0);
        let v2 = Vec3::new(0.0, 1.0, 0.0);

        let c1 = v1.cross(v2);

        assert!(c1.x() == 0.0);
        assert!(c1.y() == 0.0);
        assert!((c1.z() - 1.0).abs() < std::f32::EPSILON);

        let v3 = Vec3::new(0.0, -1.0, 0.0);
        let c2 = v3.cross(v1);
        assert!((c1.x() - c2.x()).abs() < std::f32::EPSILON);
        assert!((c1.y() - c2.y()).abs() < std::f32::EPSILON);
        assert!((c1.z() - c2.z()).abs() < std::f32::EPSILON);
    }

    #[test]
    fn length() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);

        assert!((v1.lenght() - (14.0f32).sqrt()).abs() < std::f32::EPSILON);
    }

    #[test]
    fn length_squared() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);

        assert!((v1.squared_lenght() - 14.0).abs() < std::f32::EPSILON);
    }

    #[test]
    fn normalization() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);

        assert!((v1.normalized().lenght() - 1.0).abs() < std::f32::EPSILON);

        let v2 = v1.normalized();
        v1.normalize();
        assert_eq!(v1, v2);
    }
}
