use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub,
               SubAssign};
use rand;
use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    #[inline(always)]
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }
    #[inline(always)]
    pub fn x(&self) -> f64 {
        self.e[0]
    }
    #[inline(always)]
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    #[inline(always)]
    pub fn z(&self) -> f64 {
        self.e[2]
    }
    #[inline(always)]
    pub fn r(&self) -> f64 {
        self.e[0]
    }
    #[inline(always)]
    pub fn g(&self) -> f64 {
        self.e[1]
    }
    #[inline(always)]
    pub fn b(&self) -> f64 {
        self.e[2]
    }
    #[inline(always)]
    pub fn length(&self) -> f64 {
        (self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]).sqrt()
    }
    #[inline(always)]
    pub fn squared_length(&self) -> f64 {
        (self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2])
    }
    #[inline(always)]
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
    #[inline(always)]
    fn neg(self) -> Vec3 {
        Vec3::new(-self.e[0], -self.e[1], -self.e[2])
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;
    #[inline(always)]
    fn index(&self, i: usize) -> &f64 {
        &self.e[i]
    }
}

impl IndexMut<usize> for Vec3 {
    #[inline(always)]
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        &mut self.e[i]
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
    fn mul(self, t: f64) -> Vec3 {
        Vec3::new(self.e[0] * t, self.e[1] * t, self.e[2] * t)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    #[inline(always)]
    fn mul(self: f64, v2: Vec3) -> Vec3 {
        Vec3::new(self * v2.e[0], self * v2.e[1], self * v2.e[2])
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;
    #[inline(always)]
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
    #[inline(always)]
    fn div(self, t: f64) -> Vec3 {
        Vec3::new(self.e[0] / t, self.e[1] / t, self.e[2] / t)
    }
}

impl AddAssign<Vec3> for Vec3 {
    #[inline(always)]
    fn add_assign(&mut self, v2: Vec3) {
        self.e[0] += v2.e[0];
        self.e[1] += v2.e[1];
        self.e[2] += v2.e[2];
    }
}

impl SubAssign<Vec3> for Vec3 {
    #[inline(always)]
    fn sub_assign(&mut self, v2: Vec3) {
        self.e[0] -= v2.e[0];
        self.e[1] -= v2.e[1];
        self.e[2] -= v2.e[2];
    }
}

impl MulAssign<Vec3> for Vec3 {
    #[inline(always)]
    fn mul_assign(&mut self, v2: Vec3) {
        self.e[0] *= v2.e[0];
        self.e[1] *= v2.e[1];
        self.e[2] *= v2.e[2];
    }
}

impl MulAssign<f64> for Vec3 {
    #[inline(always)]
    fn mul_assign(&mut self, t: f64) {
        self.e[0] *= t;
        self.e[1] *= t;
        self.e[2] *= t;
    }
}

impl DivAssign<Vec3> for Vec3 {
    #[inline(always)]
    fn div_assign(&mut self, v2: Vec3) {
        self.e[0] /= v2.e[0];
        self.e[1] /= v2.e[1];
        self.e[2] /= v2.e[2];
    }
}

impl DivAssign<f64> for Vec3 {
    #[inline(always)]
    fn div_assign(&mut self, t: f64) {
        self.e[0] /= t;
        self.e[1] /= t;
        self.e[2] /= t;
    }
}

impl PartialEq for Vec3 {
    #[inline(always)]
    fn eq(&self, v2: &Vec3) -> bool {
        self.e[0] == v2.e[0] && self.e[1] == v2.e[1] && self.e[2] == v2.e[2]
    }
}

#[inline(always)]
pub fn dot(v1: &Vec3, v2: &Vec3) -> f64 {
    v1.e[0] * v2.e[0] + v1.e[1] * v2.e[1] + v1.e[2] * v2.e[2]
}

#[inline(always)]
pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
    Vec3::new(
        v1.e[1] * v2.e[2] - v1.e[2] * v2.e[1],
        -(v1.e[0] * v2.e[2] - v1.e[2] * v2.e[0]),
        v1.e[0] * v2.e[1] - v1.e[1] * v2.e[0],
    )
}

#[inline(always)]
pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

#[inline(always)]
pub fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng(); // FIXME for perf
                                      // no do..while in rust.  :(
    let mut p = 2.0 * Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>())
        - Vec3::new(1.0, 1.0, 1.0);
    while p.squared_length() >= 1.0 {
        p = 2.0 * Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>())
            - Vec3::new(1.0, 1.0, 1.0);
    }
    p
}

#[inline(always)]
pub fn random_in_unit_disk() -> Vec3 {
    let mut rng = rand::thread_rng(); // FIXME for perf
    let mut p = 2.0 * Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), 0.) - Vec3::new(1.0, 1.0, 0.0);
    while p.squared_length() >= 1.0 {
        p = 2.0 * Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), 0.) - Vec3::new(1.0, 1.0, 0.0);
    }
    p
}

#[inline(always)]
pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    (*v) - 2.0 * dot(v, n) * (*n)
}

#[inline(always)]
pub fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f64, refracted: &mut Vec3) -> bool {
    let uv = unit_vector(*v);
    let dt = dot(&uv, n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        *refracted = ni_over_nt * (uv - (*n) * dt) - (*n) * discriminant.sqrt();
        return true;
    }
    false
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
