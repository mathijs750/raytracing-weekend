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

    pub fn length_squared(&self) -> f64 {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }

    pub fn length(&self) -> f64 {
        return self.length_squared().sqrt();
    }

    pub fn dot(u: Vec3, v: Vec3) -> f64 {
        return u.x * v.x + u.y * v.y + u.z * v.z;
    }

    pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
        return Vec3::new(
            u.y * v.z - u.z * v.y,
            u.z * v.x - u.x * v.z,
            u.x * v.y - u.y * v.x,
        );
    }

    pub fn normlised(&self) -> Vec3 {
        return *self / self.length();
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
        write!(
            f,
            "{} {} {}",
            self.x.round(),
            self.y.round(),
            self.z.round()
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::vec3::Vec3;

    #[test]
    fn vector_new() {
        let result = Vec3::new(3.0, 5.5, 1.22);
        assert_eq!(3.0, result.x);
        assert_eq!(5.5, result.y);
        assert_eq!(1.22, result.z);
    }

    #[test]
    fn vector_one() {
        let result = Vec3::one();
        assert_eq!(1.0, result.x);
        assert_eq!(1.0, result.y);
        assert_eq!(1.0, result.z);
    }

    #[test]
    fn vector_zero() {
        let result = Vec3::zero();
        assert_eq!(0.0, result.x);
        assert_eq!(0.0, result.y);
        assert_eq!(0.0, result.z);
    }

    #[test]
    fn vector_neg() {
        let result = -Vec3::one();

        assert_eq!(1.0, -result.x);
        assert_eq!(1.0, -result.y);
        assert_eq!(1.0, -result.z);
    }

    #[test]
    fn vector_add() {
        let value = Vec3::new(2.0, 2.0, 2.0);
        let result = Vec3::one() + value;

        assert_eq!(3.0, result.x);
        assert_eq!(3.0, result.y);
        assert_eq!(3.0, result.z);
    }

    #[test]
    fn vector_add_assign() {
        let value = Vec3::new(2.0, 2.0, 2.0);
        let mut result = Vec3::one();
        result += value;

        assert_eq!(3.0, result.x);
        assert_eq!(3.0, result.y);
        assert_eq!(3.0, result.z);
    }

    #[test]
    fn vector_sub() {
        let value = Vec3::new(2.0, 2.0, 2.0);
        let result = Vec3::one() - value;

        assert_eq!(-1.0, result.x);
        assert_eq!(-1.0, result.y);
        assert_eq!(-1.0, result.z);
    }

    #[test]
    fn vector_sub_assign() {
        let value = Vec3::new(2.0, 2.0, 2.0);
        let mut result = Vec3::one();
        result -= value;

        assert_eq!(-1.0, result.x);
        assert_eq!(-1.0, result.y);
        assert_eq!(-1.0, result.z);
    }

    #[test]
    fn vector_div() {
        let result = Vec3::one() / 2.0;

        assert_eq!(0.5, result.x);
        assert_eq!(0.5, result.y);
        assert_eq!(0.5, result.z);
    }

    #[test]
    fn vector_div_assign() {
        let mut result = Vec3::one();
        result /= 2.0;

        assert_eq!(0.5, result.x);
        assert_eq!(0.5, result.y);
        assert_eq!(0.5, result.z);
    }

    #[test]
    fn vector_mul_float() {
        let result = Vec3::one() * 2.0;

        assert_eq!(2.0, result.x);
        assert_eq!(2.0, result.y);
        assert_eq!(2.0, result.z);
    }

    #[test]
    fn vector_mul_vec3() {
        let result = Vec3::one() * Vec3::new(2.0, 2.0, 2.0);

        assert_eq!(2.0, result.x);
        assert_eq!(2.0, result.y);
        assert_eq!(2.0, result.z);
    }

    #[test]
    fn vector_mul_assign() {
        let mut result = Vec3::one();
        result *= 2.0;

        assert_eq!(2.0, result.x);
        assert_eq!(2.0, result.y);
        assert_eq!(2.0, result.z);
    }

    #[test]
    fn vector_fmt() {
        let input1 = Vec3::new((0.25 as f64) * 255.999, 0.0, (1.0 as f64) * 255.999);
        let input2 = Vec3::new(1.0 / 3.0, 2.0 / 3.0, 3.0 / 3.0);
        let result1 = format!("{}", input1);
        let result2 = format!("{}", input2);

        assert_eq!(result1, "64 0 256");
        assert_eq!(result2, "0 1 1");
    }
}
