use image::{ImageBuffer, Rgb, RgbImage};

mod ray;
mod vector;
mod sphere;
mod hittable;
mod hittable_list;
mod camera;
mod render;

use ray::Ray;
use vector::Vec3;
use hittable_list::*;
use camera::Camera;
use render::render;

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 384;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

    let camera = Camera::new();
    let world = HittableList::new();

    match buffer.save("two_spheres.png") {
        Err(e) => eprintln!("Error writing file: {}", e),
        Ok(()) => println!("Done."),
    };
}