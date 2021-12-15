fn main() {
    // Image
    let image_width: i32 = 256;
    let image_height: i32 = 256;

    // Render
    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in 0..image_height - 1 {
        for i in 0..image_width - 1 {
            let r = (i as f32 / (image_width - 1) as f32) * 255.999;
            let g = (j as f32 / (image_height - 1) as f32) * 255.999;
            let b = (0.25 as f32) * 255.999;


            print!("{} {} {}\n", r.round(),g.round(),b.round())
        }
    }
}