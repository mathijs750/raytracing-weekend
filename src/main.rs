mod color;
mod ray;
mod vec3;

fn ray_color(r: ray::Ray) -> vec3::Color {
    let unit_direction = r.direction.normlised();
    let t = 0.5 * (unit_direction.y + 1.0);
    return (vec3::Vec3::one() * (1.0 - t)) + vec3::Color::new(0.5, 0.7, 1.0) * t;
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: i32 = 400;
    let image_height: i32 = (image_width as f64 / aspect_ratio) as i32;

    //Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = vec3::Point3::zero();
    let horizontal = vec3::Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = vec3::Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - vec3::Vec3::new(0.0, 0.0, focal_length);

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
