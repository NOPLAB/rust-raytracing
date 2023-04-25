use super::EPS;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Float3([f64; 3]);

pub type Color = Float3;
pub type Vec3 = Float3;
pub type Point3 = Float3;

impl Float3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self([x, z, y])
    }

    pub const fn zero() -> Self {
        Self([0.0; 3])
    }

    pub const fn one() -> Self {
        Self([1.0; 3])
    }

    pub const fn full(value: f64) -> Self {
        Self([value; 3])
    }

    pub fn sqrt(&self) -> Self {
        Self::from_iter(self.0.iter().map(|x| x.sqrt()))
    }

    pub fn near_zero(&self) -> bool {
        self.0.iter().all(|x| x.abs() < EPS)
    }

    pub fn saturate(&self) -> Self {
        Self::from_iter(self.0.iter().map(|x| x.min(1.0).max(0.0)))
    }

    pub fn to_array(&self) -> [f64; 3] {
        self.0
    }

    pub fn iter(&self) -> std::slice::Iter<'_, f64> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, f64> {
        self.0.iter_mut()
    }
}

impl Float3 {
    /// Compute the dot product of two vectors
    pub fn dot(&self, rhs: Self) -> f64 {
        self.0
            .iter()
            .zip(rhs.0.iter())
            .fold(0.0, |acc, (l, r)| acc + l * r)
    }

    /// Compute the cross product of two vectors
    pub fn cross(&self, rhs: Self) -> Self {
        Self([
            self.0[1] * rhs.0[2] - self.0[2] * rhs.0[1],
            self.0[2] * rhs.0[0] - self.0[0] * rhs.0[2],
            self.0[0] * rhs.0[1] - self.0[1] * rhs.0[0],
        ])
    }

    /// Compute the length of vector
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    /// Compute the squared length of vector
    pub fn length_squared(&self) -> f64 {
        self.0.iter().fold(0.0, |acc, x| acc + x * x)
    }

    /// Returns normalized this vector
    pub fn normalize(&self) -> Self {
        *self / self.length()
    }

    /// Compute a reflect vector
    pub fn reflect(&self, normal: Self) -> Self {
        *self - 2.0 * self.dot(normal) * normal
    }

    /// Compute a refract vector
    pub fn refract(&self, normal: Self, ni_over_nt: f64) -> Option<Float3> {
        let uv = self.normalize();
        let dt = uv.dot(normal);
        let d = 1.0 - ni_over_nt.powi(2) * (1.0 - dt.powi(2));
        if d > 0.0 {
            Some(-ni_over_nt * (uv - normal * dt) - normal * d.sqrt())
        } else {
            None
        }
    }

    /// Compute linear interpolation between two vectors
    pub fn lerp(&self, v: Self, t: f64) -> Self {
        *self + (v - *self) * t
    }

    pub fn x(&self) -> f64 {
        self.0[0]
    }
    pub fn y(&self) -> f64 {
        self.0[1]
    }
    pub fn z(&self) -> f64 {
        self.0[2]
    }

    /// Returns a x-axis vector
    pub const fn xaxis() -> Self {
        Self::new(1.0, 0.0, 0.0)
    }
    /// Returns a y-axis vector
    pub const fn yaxis() -> Self {
        Self::new(0.0, 1.0, 0.0)
    }
    /// Returns a z-axis vector
    pub const fn zaxis() -> Self {
        Self::new(0.0, 0.0, 1.0)
    }
}

/// implements color utilities
impl Float3 {
    /// Construct from a hex slice, ex. b"ffffff"
    pub fn from_hex(hex: &[u8; 6]) -> Self {
        if let Ok(hex_str) = std::str::from_utf8(hex) {
            let r = u8::from_str_radix(&hex_str[0..2], 16).unwrap();
            let g = u8::from_str_radix(&hex_str[2..4], 16).unwrap();
            let b = u8::from_str_radix(&hex_str[4..6], 16).unwrap();
            Self::from_rgb(r, g, b)
        } else {
            panic!();
        }
    }

    /// Construct from r,g,b components. [0..255]
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self::new(r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0)
    }

    /// Returns the array that is represented by r,g,b components
    pub fn to_rgb(&self) -> [u8; 3] {
        [self.r(), self.g(), self.b()]
    }

    pub fn r(&self) -> u8 {
        (255.99 * self.0[0].min(1.0).max(0.0)) as u8
    }
    pub fn g(&self) -> u8 {
        (255.99 * self.0[1].min(1.0).max(0.0)) as u8
    }
    pub fn b(&self) -> u8 {
        (255.99 * self.0[2].min(1.0).max(0.0)) as u8
    }

    /// Convert linear space to gamma space
    pub fn gamma(&self, factor: f64) -> Self {
        let recip = factor.recip();
        Self::from_iter(self.0.iter().map(|x| x.powf(recip)))
    }

    /// Convert gamma space to linear space
    pub fn degamma(&self, factor: f64) -> Self {
        Self::from_iter(self.0.iter().map(|x| x.powf(factor)))
    }
}

impl FromIterator<f64> for Float3 {
    fn from_iter<T: IntoIterator<Item = f64>>(iter: T) -> Self {
        let mut initer = iter.into_iter();
        Float3([
            initer.next().unwrap(),
            initer.next().unwrap(),
            initer.next().unwrap(),
        ])
    }
}

/// Add: Float3 + Float3
impl std::ops::Add<Float3> for Float3 {
    type Output = Float3;
    fn add(self, rhs: Self) -> Self::Output {
        Float3([
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
        ])
    }
}

/// Sub: Float3 - Float3
impl std::ops::Sub<Float3> for Float3 {
    type Output = Float3;
    fn sub(self, rhs: Float3) -> Self::Output {
        Float3([
            self.0[0] - rhs.0[0],
            self.0[1] - rhs.0[1],
            self.0[2] - rhs.0[2],
        ])
    }
}

/// Mul Scalar: Float3 * f64
impl std::ops::Mul<f64> for Float3 {
    type Output = Float3;
    fn mul(self, rhs: f64) -> Float3 {
        Float3([self.0[0] * rhs, self.0[1] * rhs, self.0[2] * rhs])
    }
}

/// Mul Scalar: f64 * Float3
impl std::ops::Mul<Float3> for f64 {
    type Output = Float3;
    fn mul(self, rhs: Float3) -> Self::Output {
        Float3([rhs.0[0] * self, rhs.0[1] * self, rhs.0[2] * self])
    }
}

/// Div: Float3 / f64
impl std::ops::Div<f64> for Float3 {
    type Output = Float3;
    fn div(self, rhs: f64) -> Float3 {
        Float3([self.0[0] / rhs, self.0[1] / rhs, self.0[2] / rhs])
    }
}

/// Div: Float3 / Float3
impl std::ops::Div<Float3> for Float3 {
    type Output = Float3;
    fn div(self, rhs: Float3) -> Float3 {
        Float3([
            self.0[0] / rhs.0[0],
            self.0[1] / rhs.0[1],
            self.0[2] / rhs.0[2],
        ])
    }
}

/// Neg: -Float3
impl std::ops::Neg for Float3 {
    type Output = Float3;
    fn neg(self) -> Self::Output {
        Float3([-self.0[0], -self.0[1], -self.0[2]])
    }
}
