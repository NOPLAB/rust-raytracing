mod raytracing;
mod window;

use std::{
    sync::{mpsc::channel, Arc, Mutex},
    thread,
};

use crate::{
    raytracing::{
        render::{render, render_async, Scene, IMAGE_HEIGHT, IMAGE_WIDTH},
        simple_scene::SimpleScene,
    },
    window::Draw,
};

// const IMAGE_WIDTH: u32 = 200;
// const IMAGE_HEIGHT: u32 = 100;

// fn color(ray: &Ray) -> Color {
//     let d = ray.direction.normalize();
//     let t = 0.5 * (d.y() + 1.0);
//     Color::new(0.5, 0.7, 1.0).lerp(Color::one(), t)
// }

// fn hit_sphere(center: Point3, radius: f64, ray: &Ray) -> bool {
//     let oc = ray.origin - center;
//     let a = ray.direction.dot(ray.direction);
//     let b = 2.0 * ray.direction.dot(oc);
//     let c = oc.dot(oc) - radius.powi(2);
//     let d = b * b - 4.0 * a * c;
//     d > 0.0
// }

// fn color(ray: &Ray) -> Color {
//     if hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, &ray) {
//         return Color::new(1.0, 0.0, 0.0);
//     }

//     Color::new(1.0, 1.0, 1.0)
// }

fn main() {
    println!("Hello! Ray tracing world!");

    // let img = render(SimpleScene::new());

    let (tx, rx) = channel();

    let scene = SimpleScene::new();

    let output = render_async(scene, Arc::new(Mutex::new(tx)));

    // rx.recv().unwrap().save(String::from("render.png")).unwrap();

    let drawer = Draw::new();
    drawer
        .setup_async_window(rx, IMAGE_WIDTH as usize, IMAGE_HEIGHT as usize)
        .unwrap();
}
