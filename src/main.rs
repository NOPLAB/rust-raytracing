pub struct Color([f64; 3]);
mod rayt;
mod save_ppm;
use rayt::*;
use save_ppm::save_ppm;

const IMAGE_WIDTH: u32 = 200;
const IMAGE_HEIGHT: u32 = 100;

fn main() {
    println!("Hello, world!");

    let mut pixels: Vec<Color> = Vec::with_capacity(IMAGE_WIDTH as usize * IMAGE_HEIGHT as usize);
    for j in 0..IMAGE_HEIGHT {
        let par_iter = (0..IMAGE_WIDTH).into_iter().map(|i| {
            Color([
                i as f64 / IMAGE_WIDTH as f64,
                j as f64 / IMAGE_HEIGHT as f64,
                0.5,
            ])
        });
        let mut line_pixels: Vec<_> = par_iter.collect();
        pixels.append(&mut line_pixels);
    }
    save_ppm(String::from("render.ppm"), &pixels).unwrap();
}
