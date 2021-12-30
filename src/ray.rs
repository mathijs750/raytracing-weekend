use crate::vector::*;
use crate::hittable::*;
use crate::hittable_list::*;

const T_MIN: f64 = 0.0001;
const T_MAX: f64 = f64::MAX;

const COLOR_WHITE: Color = Color { x: 1.0, y: 1.0, z: 1.0};
const COLOR_SKYBLUE: Color = Color { x: 0.5, y: 0.7, z: 1.0 };

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray {
            origin: origin,
            direction: direction,
        }
    }

    pub fn at(&self, t: f64) -> Point3 {
        return self.origin + self.direction * t;
    }

    pub fn color(&self, world: &HittableList) -> Color {
        let t = world.hit(&self, T_MIN, T_MAX);
        match t {
            Some(record) => {
                0.5 * (COLOR_WHITE + record.normal.unit_vector())
            }

            None => {
                let unit_direction = self.direction.unit_vector();
                let t = 0.5 * (unit_direction.y + 1.0);
                (1.0 - t) * COLOR_WHITE + t * COLOR_SKYBLUE
            }
        }
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
    fn ray_new() {
        let result = Ray::new(Point3::new(3.5, 1.5, 0.0), Vec3::one());

        assert_vec3_equal!(3.5, result.origin.x);
        assert_vec3_equal!(1.5, result.origin.y);
        assert_vec3_equal!(0.0, result.origin.z);

        assert_vec3_equal!(1.0, result.direction.x);
        assert_vec3_equal!(1.0, result.direction.y);
        assert_vec3_equal!(1.0, result.direction.z);
    }

    #[test]
    fn ray_at() {
        let ray = Ray::new(Point3::new(1.0, 1.0, 1.0), Vec3::new(3.0, 4.0, 0.0));
        let position = ray.at(5.0);
        let expected = Vec3::new(16.0, 21.0, 1.0);
        
        assert_vec3_equal!(expected.x, position.x);
        assert_vec3_equal!(expected.y, position.y);
        assert_vec3_equal!(expected.z, position.z);
    }
}
