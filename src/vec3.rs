use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn zero() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn one() -> Vec3 {
        Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        };
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, factor: f64) -> Vec3 {
        Vec3 {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, factor: f64) {
        *self = Self {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
        };
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, factor: f64) -> Vec3 {
        Vec3 {
            x: (1.0 / factor) * self.x,
            y: (1.0 / factor) * self.y,
            z: (1.0 / factor) * self.z,
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, factor: f64) {
        *self = Self {
            x: (1.0 / factor) * self.x,
            y: (1.0 / factor) * self.y,
            z: (1.0 / factor) * self.z,
        };
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use crate::vec3::Vec3;

    #[test]
    fn vector_new() {
        let result = Vec3::new(3.0, 5.5, 1.22);
        assert_eq!(result.x, 3.0);
        assert_eq!(result.y, 5.5);
        assert_eq!(result.z, 1.22);
    }

    #[test]
    fn vector_one() {
        let result = Vec3::one();
        assert_eq!(result.x, 1.0);
        assert_eq!(result.y, 1.0);
        assert_eq!(result.z, 1.0);
    }

    #[test]
    fn vector_zero() {
        let result = Vec3::zero();
        assert_eq!(result.x, 0.0);
        assert_eq!(result.y, 0.0);
        assert_eq!(result.z, 0.0);
    }

    #[test]
    fn vector_neg() {
        let result = -Vec3::one();

        assert_eq!(result.x, -1.0);
        assert_eq!(result.y, -1.0);
        assert_eq!(result.z, -1.0);
    }

    #[test]
    fn vector_add() {
        let value = Vec3::new(2.0, 2.0, 2.0);
        let result = Vec3::one() + value;

        assert_eq!(result.x, 3.0);
        assert_eq!(result.y, 3.0);
        assert_eq!(result.z, 3.0);
    }

    #[test]
    fn vector_add_assign() {
        let value = Vec3::new(2.0, 2.0, 2.0);
        let mut result = Vec3::one();
        result += value;

        assert_eq!(result.x, 3.0);
        assert_eq!(result.y, 3.0);
        assert_eq!(result.z, 3.0);
    }

    #[test]
    fn vector_sub() {
        let value = Vec3::new(2.0, 2.0, 2.0);
        let result = Vec3::one() - value;

        assert_eq!(result.x, -1.0);
        assert_eq!(result.y, -1.0);
        assert_eq!(result.z, -1.0);
    }

    #[test]
    fn vector_sub_assign() {
        let value = Vec3::new(2.0, 2.0, 2.0);
        let mut result = Vec3::one();
        result -= value;

        assert_eq!(result.x, -1.0);
        assert_eq!(result.y, -1.0);
        assert_eq!(result.z, -1.0);
    }

    #[test]
    fn vector_div() {
        let result = Vec3::one() / 2.0;

        assert_eq!(result.x, 0.5);
        assert_eq!(result.y, 0.5);
        assert_eq!(result.z, 0.5);
    }

    #[test]
    fn vector_div_assign() {
        let mut result = Vec3::one();
        result /= 2.0;

        assert_eq!(result.x, 0.5);
        assert_eq!(result.y, 0.5);
        assert_eq!(result.z, 0.5);
    }

    #[test]
    fn vector_mul_float() {
        let result = Vec3::one() * 2.0;

        assert_eq!(result.x, 2.0);
        assert_eq!(result.y, 2.0);
        assert_eq!(result.z, 2.0);
    }

    #[test]
    fn vector_mul_vec3() {
        let result = Vec3::one() * Vec3::new(2.0, 2.0, 2.0);

        assert_eq!(result.x, 2.0);
        assert_eq!(result.y, 2.0);
        assert_eq!(result.z, 2.0);
    }

    #[test]
    fn vector_mul_assign() {
        let mut result = Vec3::one();
        result *= 2.0;

        assert_eq!(result.x, 2.0);
        assert_eq!(result.y, 2.0);
        assert_eq!(result.z, 2.0);
    }
}
