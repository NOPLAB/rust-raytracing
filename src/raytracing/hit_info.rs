use super::{Point3, Vec3};

pub struct HitInfo {
    pub length: f64,
    pub position: Point3,
    pub direction: Vec3,
}

impl HitInfo {
    pub const fn new(length: f64, position: Point3, direction: Vec3) -> Self {
        Self {
            length,
            position,
            direction,
        }
    }
}
