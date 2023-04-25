use super::{
    camera::Camera,
    ray::Ray,
    render::Scene,
    shapes::{Shape, ShapeList, Sphere},
    Color, Float3, Point3, Vec3,
};

pub struct SimpleScene {
    objects: ShapeList,
}

impl SimpleScene {
    pub fn new() -> Self {
        let mut objects = ShapeList::new();
        objects.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
        objects.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));
        Self { objects }
    }

    fn hit_sphere(&self, center: Point3, radius: f64, ray: &Ray) -> f64 {
        let oc = ray.origin - center;
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(oc);
        let c = oc.dot(oc) - radius.powi(2);
        let d = b * b - 4.0 * a * c;

        // 判別式
        if d < 0.0 {
            -1.0
        } else {
            return (-b - -d.sqrt()) / (2.0 * a);
        }
    }

    fn background(&self, d: Vec3) -> Color {
        let t = 0.5 * (d.normalize().y() + 1.0);
        Color::one().lerp(Color::new(0.5, 0.7, 1.0), t)
    }
}

impl Scene for SimpleScene {
    fn camera(&self) -> Camera {
        Camera::new(
            Vec3::new(4.0, 0.0, 0.0),
            Vec3::new(0.0, 2.0, 0.0),
            Vec3::new(-2.0, -1.0, -1.0),
        )
    }

    fn trace(&self, ray: Ray) -> Color {
        let hit_info = self.objects.hit(&ray, 0.0, f64::MAX);
        if let Some(hit) = hit_info {
            0.8 * (hit.length * Vec3::one())
            // Color::new(1.0, 0.0, 0.0)
        } else {
            self.background(ray.direction)
        }
    }
}
