use core::fmt;
use rand::prelude::Rng;
use std::ops::Range;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub type Point3 = Vec3;
pub type Color = Vec3;

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

    pub fn length_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn normlised(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn unit_vector(self) -> Vec3 {
        self.normlised()
    }

    pub fn sqrt(self) -> Vec3 {
        Vec3::new(self.x.sqrt(), self.y.sqrt(), self.z.sqrt())
    }

    pub fn reflect(self, other: Vec3) -> Vec3 {
        self - 2.0 * self.dot(other) * other
    }

    pub fn to_rgba(self) -> [u8; 4] {
        fn f(num: f64) -> u8 {
            if num < 0.0 {
                0
            } else if num >= 1.0 {
                255
            } else {
                (num * 255.99) as u8
            }
        }
        [f(self.x), f(self.y), f(self.z), 0xff]
    }

    pub fn random(r: Range<f64>) -> Vec3 {
        let mut rng = rand::thread_rng();

        Vec3::new(
            rng.gen_range(r.clone()),
            rng.gen_range(r.clone()),
            rng.gen_range(r.clone()),
        )
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let v = Vec3::random(-1.0..1.0);
            if v.length() < 1.0 {
                return v;
            }
        }
    }

    pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
        let in_unit_sphere = Self::random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0.0 {
            // In the same hemisphere as the normal
            in_unit_sphere
        } else {
            (-1.0) * in_unit_sphere
        }
    }

    pub fn random_in_unit_disk() -> Vec3 {
        let mut rng = rand::thread_rng();

        loop {
            let p = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
            if p.length() < 1.0 {
                return p;
            }
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

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
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
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::*;

    macro_rules! assert_vec3_equal {
        ($expected:expr, $actual:expr) => {
            let tolerance = 0.0001;
            assert_approx_eq!($expected, $actual, tolerance);
        };
    }

    #[test]
    fn vector_new() {
        let result = Vec3::new(3.0, 5.5, 1.22);
        assert_vec3_equal!(3.0, result.x);
        assert_vec3_equal!(5.5, result.y);
        assert_vec3_equal!(1.22, result.z);
    }

    #[test]
    fn vector_one() {
        let result = Vec3::one();
        assert_vec3_equal!(1.0, result.x);
        assert_vec3_equal!(1.0, result.y);
        assert_vec3_equal!(1.0, result.z);
    }

    #[test]
    fn vector_zero() {
        let result = Vec3::zero();
        assert_vec3_equal!(0.0, result.x);
        assert_vec3_equal!(0.0, result.y);
        assert_vec3_equal!(0.0, result.z);
    }

    #[test]
    fn vector_neg() {
        let result = -Vec3::one();

        assert_vec3_equal!(1.0, -result.x);
        assert_vec3_equal!(1.0, -result.y);
        assert_vec3_equal!(1.0, -result.z);
    }

    #[test]
    fn vector_add() {
        let value = Vec3::new(2.0, 2.0, 2.0);
        let result = Vec3::one() + value;

        assert_vec3_equal!(3.0, result.x);
        assert_vec3_equal!(3.0, result.y);
        assert_vec3_equal!(3.0, result.z);
    }

    #[test]
    fn vector_add_assign() {
        let value = Vec3::new(2.0, 2.0, 2.0);
        let mut result = Vec3::one();
        result += value;

        assert_vec3_equal!(3.0, result.x);
        assert_vec3_equal!(3.0, result.y);
        assert_vec3_equal!(3.0, result.z);
    }

    #[test]
    fn vector_sub() {
        let value = Vec3::new(2.0, 2.0, 2.0);
        let result = Vec3::one() - value;

        assert_vec3_equal!(-1.0, result.x);
        assert_vec3_equal!(-1.0, result.y);
        assert_vec3_equal!(-1.0, result.z);
    }

    #[test]
    fn vector_sub_assign() {
        let value = Vec3::new(2.0, 2.0, 2.0);
        let mut result = Vec3::one();
        result -= value;

        assert_vec3_equal!(-1.0, result.x);
        assert_vec3_equal!(-1.0, result.y);
        assert_vec3_equal!(-1.0, result.z);
    }

    #[test]
    fn vector_div() {
        let result = Vec3::one() / 2.0;

        assert_vec3_equal!(0.5, result.x);
        assert_vec3_equal!(0.5, result.y);
        assert_vec3_equal!(0.5, result.z);
    }

    #[test]
    fn vector_div_assign() {
        let mut result = Vec3::one();
        result /= 2.0;

        assert_vec3_equal!(0.5, result.x);
        assert_vec3_equal!(0.5, result.y);
        assert_vec3_equal!(0.5, result.z);
    }

    #[test]
    fn vector_mul_vec3_with_float() {
        let result = Vec3::one() * 2.0;

        assert_vec3_equal!(2.0, result.x);
        assert_vec3_equal!(2.0, result.y);
        assert_vec3_equal!(2.0, result.z);
    }

    #[test]
    fn vector_mul_float_with_vec3() {
        let result = 2.0 * Vec3::one();

        assert_vec3_equal!(2.0, result.x);
        assert_vec3_equal!(2.0, result.y);
        assert_vec3_equal!(2.0, result.z);
    }

    #[test]
    fn vector_mul_vec3_with_vec3() {
        let result = Vec3::one() * Vec3::new(2.0, 2.0, 2.0);

        assert_vec3_equal!(2.0, result.x);
        assert_vec3_equal!(2.0, result.y);
        assert_vec3_equal!(2.0, result.z);
    }

    #[test]
    fn vector_mul_assign() {
        let mut result = Vec3::one();
        result *= 2.0;

        assert_vec3_equal!(2.0, result.x);
        assert_vec3_equal!(2.0, result.y);
        assert_vec3_equal!(2.0, result.z);
    }

    #[test]
    fn vector_format() {
        let input = Vec3::new(1.0, 2.5, 3.33333333333);
        let result = format!("{}", input);

        assert_eq!("(1, 2.5, 3.33333333333)", result);
    }

    #[test]
    fn vector_length() {
        let input = Vec3::new(5.0, 12.0, 0.0).length();

        assert_vec3_equal!(input, 13.0);
    }

    #[test]
    fn vector_length_squared() {
        let input = Vec3::new(5.0, 12.0, 0.0).length_squared();

        assert_vec3_equal!(input, 169.0);
    }

    #[test]
    fn vector_dot_product() {
        let a = Vec3::new(9.0, 2.0, 7.0);
        let b = Vec3::new(4.0, 8.0, 10.0);

        let result = a.dot(b);

        assert_vec3_equal!(122.0, result);
    }

    #[test]
    fn vector_cross_product1() {
        let a = Vec3::new(1.0, 0.0, 0.0);
        let b = Vec3::new(0.0, 1.0, 0.0);

        let result = a.cross(b);

        assert_vec3_equal!(0.0, result.x);
        assert_vec3_equal!(0.0, result.y);
        assert_vec3_equal!(1.0, result.z);
    }

    #[test]
    fn vector_cross_product2() {
        let a = Vec3::new(2.0, 3.0, 4.0);
        let b = Vec3::new(5.0, 6.0, 7.0);

        let result = a.cross(b);

        assert_vec3_equal!(-3.0, result.x);
        assert_vec3_equal!(6.0, result.y);
        assert_vec3_equal!(-3.0, result.z);
    }

    #[test]
    fn vector_hadamard_product() {
        let vector1 = Vec3::new(1.0, 2.0, 3.0);
        let vector2 = Vec3::new(4.0, 5.0, 6.0);

        let result = vector1 * vector2;
        let expected = Vec3::new(4.0, 10.0, 18.0);

        assert_vec3_equal!(expected.x, result.x);
        assert_vec3_equal!(expected.y, result.y);
        assert_vec3_equal!(expected.z, result.z);
    }

    #[test]
    fn vector_normalised_big_vector() {
        let input = Vec3::new(3.0, 12.0, 24.0).normlised();
        let expected = Vec3::new(1.0 / 9.0, 4.0 / 9.0, 8.0 / 9.0);
        let length = expected.length();

        assert_vec3_equal!(expected.x, input.x);
        assert_vec3_equal!(expected.y, input.y);
        assert_vec3_equal!(expected.z, input.z);
        assert_vec3_equal!(1.0, length);
    }

    #[test]
    fn vector_normalised_small_vector() {
        let input = Vec3::new(3.0 / 10.0, 3.0 / 5.0, 1.0).normlised();
        let expected = Vec3::new(0.24913643956122, 0.49827287912244, 0.8304547985374);
        let length = expected.length();

        assert_vec3_equal!(expected.x, input.x);
        assert_vec3_equal!(expected.y, input.y);
        assert_vec3_equal!(expected.z, input.z);
        assert_vec3_equal!(1.0, length);
    }

    #[test]
    fn vector_unitvector() {
        let input = Vec3::new(3.0, 12.0, 24.0).unit_vector();
        let expected = Vec3::new(1.0 / 9.0, 4.0 / 9.0, 8.0 / 9.0);

        assert_vec3_equal!(expected.x, input.x);
        assert_vec3_equal!(expected.y, input.y);
        assert_vec3_equal!(expected.z, input.z);
    }

    #[test]
    fn vector_to_rbga() {
        let result = Vec3::new(-1.0, 0.5, 1.4).to_rgba();
        let expected = [0u8, 127u8, 255u8, 0xff];

        assert_eq!(expected, result);
    }

    #[test]
    fn vector_random_unit_sphere() {
        let input = Vec3::random_in_unit_sphere();
        let result = input.length();

        assert_ne!(1.0, result);
    }

    #[test]
    fn vector_random_unit_hemisphere() {
        let input1 = Vec3::random_in_hemisphere(Vec3::new(0.0, 1.0, 0.0));
        let input2 = Vec3::random_in_hemisphere(Vec3::new(0.0, -1.0, 0.0));
        let result1 = input1.y > 0.0;
        let result2 = input2.y < 0.0;

        assert!(result1);
        assert!(result2);
        assert_ne!(1.0, input1.length());
        assert_ne!(1.0, input2.length());
    }

    #[test]
    fn vector_random_unit_disc() {
        let input = Vec3::random_in_unit_disk();
        let result = input.length();

        assert_ne!(1.0, result);
        assert_vec3_equal!(0.0, input.z);
    }

    #[test]
    fn vector_square_root() {
        let input = Vec3::new(144.0, 144.0, 144.0);
        let result = input.sqrt();
        let expected = Vec3::new(12.0, 12.0, 12.0);

        assert_vec3_equal!(expected.x, result.x);
        assert_vec3_equal!(expected.y, result.y);
        assert_vec3_equal!(expected.z, result.z);
    }
}
