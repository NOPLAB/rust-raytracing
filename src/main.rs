mod raytracing;
mod window;

use crate::{
    raytracing::{
        render::{render, Scene},
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

    let img = render(SimpleScene::new());

    img.save(String::from("render.png")).unwrap();

    let drawer = Draw::new();
    drawer.setup_window(img).unwrap();
}
