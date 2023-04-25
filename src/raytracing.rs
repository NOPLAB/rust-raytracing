pub mod camera;
pub mod float3;
pub mod hit_info;
pub mod quaternion;
pub mod ray;
pub mod render;
pub mod shapes;
pub mod simple_scene;

pub use self::float3::{Color, Float3, Point3, Vec3};
pub use std::f64::consts::FRAC_1_PI;
pub use std::f64::consts::PI;
pub const PI2: f64 = PI * 2.0;
pub const EPS: f64 = 1e-6;
