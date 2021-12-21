mod color;
mod vec3;
mod ray;

fn main() {
    // Image
    let image_width: i32 = 256;
    let image_height: i32 = 256;

    //Render
    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height - 1).rev() {
        for i in 0..image_width {
            let r = (i as f64 / (image_width - 1) as f64) * 255.999;
            let g = (j as f64 / (image_height - 1) as f64) * 255.999;
            let b = (0.25 as f64) * 255.999;

            let color = vec3::Color::new(r, g, b);
            color.write_color();
        }
    }
}
