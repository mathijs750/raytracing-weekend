mod vector3;

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

            let vector = vector3::Vector3::new(r, g, b);

            print!(
                "{} {} {}\n",
                vector.x.round(),
                vector.y.round(),
                vector.z.round()
            );
        }
    }

    // let kaas = vector3::Vector3::new(1.0, 2.0, 3.0);

    // print!("{}", kaas);
}
