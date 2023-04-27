use std::{
    sync::{mpsc::Sender, Arc, Mutex},
    thread::{self, JoinHandle},
    time::Duration,
};

use image::{ImageBuffer, Rgb, RgbImage};

use super::{camera::Camera, ray::Ray, Color};

// const IMAGE_WIDTH: u32 = 200;
// const IMAGE_HEIGHT: u32 = 100;

pub const IMAGE_WIDTH: u32 = 1920;
pub const IMAGE_HEIGHT: u32 = 1080;

const BACKUP_FILENAME: &str = "render.png";

pub trait Scene: Sync + Send {
    fn camera(&self) -> Camera;
    fn trace(&self, ray: Ray) -> Color;
    fn width(&self) -> u32 {
        IMAGE_WIDTH
    }
    fn height(&self) -> u32 {
        IMAGE_HEIGHT
    }
    fn aspect(&self) -> f64 {
        self.width() as f64 / self.height() as f64
    }
}

pub fn render(scene: impl Scene + Sync) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let camera = scene.camera();

    let mut img = RgbImage::new(scene.width(), scene.height());

    img.enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        let u = x as f64 / (scene.width() - 1) as f64;
        let v = (scene.height() - y - 1) as f64 / (scene.height() - 1) as f64;

        let ray = camera.ray(u, v);

        let rgb = scene.trace(ray).to_rgb();

        pixel[0] = rgb[0];
        pixel[1] = rgb[1];
        pixel[2] = rgb[2];
    });

    img
}

pub fn render_async(
    scene: impl Scene + Sync + Send + 'static,
    tx: Arc<Mutex<Sender<RgbImage>>>,
) -> JoinHandle<ImageBuffer<Rgb<u8>, Vec<u8>>> {
    thread::spawn(move || {
        let mut img = RgbImage::new(scene.width(), scene.height());

        let camera = scene.camera();

        for y in 0..scene.height() {
            for x in 0..scene.width() {
                let u = x as f64 / (scene.width() - 1) as f64;
                let v = (scene.height() - y - 1) as f64 / (scene.height() - 1) as f64;

                let ray = camera.ray(u, v);

                let rgb = scene.trace(ray).to_rgb();

                img.put_pixel(x, y, Rgb(rgb));
                // thread::sleep(Duration::from_millis(10));
            }

            let locked_tx = tx.lock().unwrap();
            drop(&locked_tx);
            locked_tx.send(img.clone()).unwrap();
            // println!("tx");
        }

        img
    })
}
