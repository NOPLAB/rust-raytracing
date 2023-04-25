use super::{ray::Ray, Color, Point3, Vec3};

pub struct Camera {
    pub origin: Point3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
}

impl Camera {
    pub fn new(u: Vec3, v: Vec3, w: Vec3) -> Self {
        Self {
            origin: Point3::zero(),
            u, // x
            v, // y
            w, // z
        }
    }

    pub fn from_lookat(
        origin: Vec3,
        lookat: Vec3,
        camera_up: Vec3,
        camera_fov: f64,
        aspect: f64,
    ) -> Self {
        // カメラのfovでスクリーンの幅を求める
        let half_h = (camera_fov.to_radians() * 0.5).tan();
        let half_w = aspect * half_h;

        // カメラとスクリーンの奥行き(手前)
        let w = (origin - lookat).normalize();

        // 奥行き(手前)とカメラの上方向を外積すると、スクリーンx軸プラス方向のベクトルが出てくる
        let u = camera_up.cross(w).normalize();
        // 奥行き(手前)とスクリーンx軸プラス方向のベクトルを外積すると、スクリーンy軸プラス方向のベクトルが出てくる
        let v = w.cross(u);

        // 正規化したスクリーンのベクトルをスクリーン幅でスカラー倍する
        let uw = u * half_w;
        let vh = v * half_h;

        Self {
            origin: origin,
            u: uw * 2.0,
            v: vh * 2.0,
            w: origin - uw - vh - w, // 原点位置から引いていくことでz軸方向のベクトル(位置)が出る
        }
    }

    pub fn ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.w + self.u * u + self.v * v - self.origin,
        }
    }
}
