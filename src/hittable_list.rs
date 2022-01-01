use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vector::*;

pub struct HittableList {
    pub objects: Vec<Sphere>,
}

impl HittableList {
    pub fn new() -> HittableList {
        let spheres = vec![
            Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5),
            Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0),
        ];
        HittableList { objects: spheres }
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut temp_rec = HitRecord::new(Point3::zero(), Vec3::zero(), t_max, true);

        for object in &self.objects {
            if let Some(record) = object.hit(ray, t_min, t_max) {
                if record.t < temp_rec.t {
                    temp_rec = record;
                }
            }
        }

        if temp_rec.t == t_max {
            None
        } else {
            Some(temp_rec)
        }
    }
}
