use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub,
               SubAssign};
use rand;
use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }
    pub fn x(&self) -> f64 {
        self.e[0]
    }
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    pub fn z(&self) -> f64 {
        self.e[2]
    }
    pub fn r(&self) -> f64 {
        self.e[0]
    }
    pub fn g(&self) -> f64 {
        self.e[1]
    }
    pub fn b(&self) -> f64 {
        self.e[2]
    }
    pub fn length(&self) -> f64 {
        (self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]).sqrt()
    }
    pub fn squared_length(&self) -> f64 {
        (self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2])
    }
    pub fn make_unit_vector(&mut self) {
        let k =
            1.0 / (self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]).sqrt();
        self.e[0] *= k;
        self.e[1] *= k;
        self.e[2] *= k;
    }
}

// FIXME? Missing operator?  Rust doesn't have a unary + operator
impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3::new(-self.e[0], -self.e[1], -self.e[2])
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, i: usize) -> &f64 {
        &self.e[i]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        &mut self.e[i]
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, v2: Vec3) -> Vec3 {
        Vec3::new(
            self.e[0] + v2.e[0],
            self.e[1] + v2.e[1],
            self.e[2] + v2.e[2],
        )
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, v2: Vec3) -> Vec3 {
        Vec3::new(
            self.e[0] - v2.e[0],
            self.e[1] - v2.e[1],
            self.e[2] - v2.e[2],
        )
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, v2: Vec3) -> Vec3 {
        Vec3::new(
            self.e[0] * v2.e[0],
            self.e[1] * v2.e[1],
            self.e[2] * v2.e[2],
        )
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, t: f64) -> Vec3 {
        Vec3::new(self.e[0] * t, self.e[1] * t, self.e[2] * t)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self: f64, v2: Vec3) -> Vec3 {
        Vec3::new(self * v2.e[0], self * v2.e[1], self * v2.e[2])
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;
    fn div(self, v2: Vec3) -> Vec3 {
        Vec3::new(
            self.e[0] / v2.e[0],
            self.e[1] / v2.e[1],
            self.e[2] / v2.e[2],
        )
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, t: f64) -> Vec3 {
        Vec3::new(self.e[0] / t, self.e[1] / t, self.e[2] / t)
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, v2: Vec3) {
        self.e[0] += v2.e[0];
        self.e[1] += v2.e[1];
        self.e[2] += v2.e[2];
    }
}

impl SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, v2: Vec3) {
        self.e[0] -= v2.e[0];
        self.e[1] -= v2.e[1];
        self.e[2] -= v2.e[2];
    }
}

impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, v2: Vec3) {
        self.e[0] *= v2.e[0];
        self.e[1] *= v2.e[1];
        self.e[2] *= v2.e[2];
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        self.e[0] *= t;
        self.e[1] *= t;
        self.e[2] *= t;
    }
}

impl DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, v2: Vec3) {
        self.e[0] /= v2.e[0];
        self.e[1] /= v2.e[1];
        self.e[2] /= v2.e[2];
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        self.e[0] /= t;
        self.e[1] /= t;
        self.e[2] /= t;
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, v2: &Vec3) -> bool {
        self.e[0] == v2.e[0] && self.e[1] == v2.e[1] && self.e[2] == v2.e[2]
    }
}

pub fn dot(v1: &Vec3, v2: &Vec3) -> f64 {
    v1.e[0] * v2.e[0] + v1.e[1] * v2.e[1] + v1.e[2] * v2.e[2]
}

pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
    Vec3::new(
        v1.e[1] * v2.e[2] - v1.e[2] * v2.e[1],
        -(v1.e[0] * v2.e[2] - v1.e[2] * v2.e[0]),
        v1.e[0] * v2.e[1] - v1.e[1] * v2.e[0],
    )
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

pub fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    // no do..while in rust.  :(
    let mut p = 2.0 * Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>())
        - Vec3::new(1.0, 1.0, 1.0);
    while p.squared_length() >= 1.0 {
        p = 2.0 * Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>())
            - Vec3::new(1.0, 1.0, 1.0);
    }
    p
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    (*v) - 2.0 * dot(v, n) * (*n)
}

// ======================================================================
// Unit testing
// ======================================================================
#[test]
fn test_neg() {
    assert_eq!(-Vec3::new(1.0, 2.0, 3.0), Vec3::new(-1.0, -2.0, -3.0));
}
#[test]
fn test_add() {
    assert_eq!(
        Vec3::new(1.0, 2.0, 3.0) + Vec3::new(1.0, 2.0, 3.0),
        Vec3::new(2.0, 4.0, 6.0)
    );
}
#[test]
fn test_sub() {
    assert_eq!(
        Vec3::new(1.0, 2.0, 3.0) - Vec3::new(1.0, 2.0, 3.0),
        Vec3::new(0.0, 0.0, 0.0)
    );
}
#[test]
fn test_mul() {
    assert_eq!(
        Vec3::new(1.0, 2.0, 3.0) * Vec3::new(1.0, 2.0, 3.0),
        Vec3::new(1.0, 4.0, 9.0)
    );
}
#[test]
fn test_div() {
    assert_eq!(
        Vec3::new(1.0, 2.0, 3.0) / Vec3::new(1.0, 2.0, 3.0),
        Vec3::new(1.0, 1.0, 1.0)
    );
}
#[test]
fn test_add_assign() {
    let mut t = Vec3::new(1.0, 2.0, 3.0);
    t += Vec3::new(1.0, 2.0, 3.0);
    assert_eq!(t, Vec3::new(2.0, 4.0, 6.0));
}
#[test]
fn test_dot() {
    assert_eq!(
        dot(&Vec3::new(1.0, 2.0, 3.0), &Vec3::new(1.0, 2.0, 3.0)),
        1.0 * 1.0 + 2.0 * 2.0 + 3.0 * 3.0
    );
}
#[test]
fn test_cross() {
    assert_eq!(
        cross(&Vec3::new(1.0, 2.0, 3.0), &Vec3::new(1.0, 2.0, 3.0)),
        Vec3::new(
            1.0 * 2.0 - 2.0 * 1.0,
            -(1.0 * 2.0 - 2.0 * 1.0),
            1.0 * 2.0 - 2.0 * 1.0
        )
    );
}
