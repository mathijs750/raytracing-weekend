mod color;
mod ray;
mod vec3;

use crate::vec3::*;

fn hit_sphere(center: Point3, radius: f64, ray: ray::Ray) -> f64 {
    let oc = ray.origin - center;
    let a = ray.direction.length_squared();
    let half_b = Vec3::dot(oc, ray.direction);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - discriminant.sqrt()) /  a;
    }
}

fn ray_color(r: ray::Ray) -> Color {
    let mut t = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let normal = (r.at(t) - Vec3::new(0.0, 0.0, -1.0)).normlised();
        return Color::new(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0) * 0.5;
    }

    let unit_direction = r.direction.normlised();
    t = 0.5 * (unit_direction.y + 1.0);
    return (Vec3::one() * (1.0 - t)) + Color::new(0.5, 0.7, 1.0) * t;
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    //Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::zero();
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    //Render
    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height - 1).rev() {
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let r = ray::Ray::new(
                origin,
                lower_left_corner + horizontal * u + vertical * v - origin,
            );
            let pixel_color = ray_color(r);
            pixel_color.write_color();
        }
    }
}
