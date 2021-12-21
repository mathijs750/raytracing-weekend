use crate::vec3::*;

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
}

#[cfg(test)]
mod tests {
    use crate::ray::*;

    #[test]
    fn ray_new() {
        let result = Ray::new(Point3::new(3.5, 1.5, 0.0), Vec3::one());

        assert_eq!(3.5, result.origin.x);
        assert_eq!(1.5, result.origin.y);
        assert_eq!(0.0, result.origin.z);

        assert_eq!(1.0, result.direction.x);
        assert_eq!(1.0, result.direction.y);
        assert_eq!(1.0, result.direction.z);
    }

    #[test]
    fn ray_at() {
        let input = Ray::new(Point3::zero(), Vec3::new(3.0, 1.0, 0.0));

        let result = input.at(5.0);

        assert_eq!(15.0, result.x);
        assert_eq!(5.0, result.y);
        assert_eq!(0.0, result.z);
    }
}