use super::{
    hit_info::{self, HitInfo},
    ray::Ray,
    Point3,
};

pub trait Shape: Sync + Send {
    fn hit(&self, ray: &Ray, t0: f64, t1: f64) -> Option<HitInfo>;
}

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub const fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Shape for Sphere {
    fn hit(&self, ray: &Ray, t0: f64, t1: f64) -> Option<HitInfo> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(oc);
        let c = oc.dot(oc) - self.radius.powi(2);
        let d = b * b - 4.0 * a * c;

        // 判別式
        if d > 0.0 {
            let root = d.sqrt();
            let temp = (-b - root) / (2.0 * a);
            if t0 < temp && temp < t1 {
                let p = ray.at(temp);
                return Some(HitInfo::new(temp, p, (p - self.center) / self.radius));
            }
            let temp = (-b + root) / (2.0 * a);
            if t0 < temp && temp < t1 {
                let p = ray.at(temp);
                return Some(HitInfo::new(temp, p, (p - self.center) / self.radius));
            }
        }

        None
    }
}

pub struct ShapeList {
    pub objects: Vec<Box<dyn Shape>>,
}

impl ShapeList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn push(&mut self, object: Box<dyn Shape>) {
        self.objects.push(object)
    }
}

impl Shape for ShapeList {
    fn hit(&self, ray: &Ray, t0: f64, t1: f64) -> Option<HitInfo> {
        let mut hit_info: Option<HitInfo> = None;
        let mut closet_so_far = t1;
        for objects in &self.objects {
            if let Some(info) = objects.hit(ray, t0, closet_so_far) {
                closet_so_far = info.length;
                hit_info = Some(info);
            }
        }

        hit_info
    }
}

impl std::ops::Index<usize> for ShapeList {
    type Output = Box<dyn Shape>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.objects[index]
    }
}

impl std::ops::IndexMut<usize> for ShapeList {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.objects[index]
    }
}
