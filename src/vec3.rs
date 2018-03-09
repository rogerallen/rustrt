use std::ops::{Add,Div,Index,Mul,Sub};

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    e: [f64; 3]
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }
    pub fn x(&self) -> f64 { self.e[0] }
    pub fn y(&self) -> f64 { self.e[1] }
    pub fn z(&self) -> f64 { self.e[2] }
    pub fn r(&self) -> f64 { self.e[0] }
    pub fn g(&self) -> f64 { self.e[1] }
    pub fn b(&self) -> f64 { self.e[2] }
    pub fn length(&self) -> f64 {
        (self.e[0] * self.e[0] + self.e[1]*self.e[1] + self.e[2]*self.e[2]).sqrt()
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, i: usize) -> &f64 { &self.e[i] }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, v2: Vec3) -> Vec3 {
        Vec3::new(self.e[0] + v2.e[0], self.e[1] + v2.e[1], self.e[2] + v2.e[2])
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, v2: Vec3) -> Vec3 {
        Vec3::new(self.e[0] - v2.e[0], self.e[1] - v2.e[1], self.e[2] - v2.e[2])
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, v2: Vec3) -> Vec3 {
        Vec3::new(self.e[0] * v2.e[0], self.e[1] * v2.e[1], self.e[2] * v2.e[2])
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
        Vec3::new(self.e[0] / v2.e[0], self.e[1] / v2.e[1], self.e[2] / v2.e[2])
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, t: f64) -> Vec3 {
        Vec3::new(self.e[0] / t, self.e[1] / t, self.e[2] / t)
    }
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

// ======================================================================
// Unit testing
// ======================================================================
impl PartialEq for Vec3 {
    fn eq(&self, v2: &Vec3) -> bool {
        self.e[0] == v2.e[0] && self.e[1] == v2.e[1] && self.e[2] == v2.e[2]
    }
}

#[test]
fn test_add() {
    assert_eq!(Vec3::new(1.0,2.0,3.0) + Vec3::new(1.0,2.0,3.0),
               Vec3::new(2.0,4.0,6.0));
}
#[test]
fn test_sub() {
    assert_eq!(Vec3::new(1.0,2.0,3.0) - Vec3::new(1.0,2.0,3.0),
               Vec3::new(0.0,0.0,0.0));
}
#[test]
fn test_mul() {
    assert_eq!(Vec3::new(1.0,2.0,3.0) * Vec3::new(1.0,2.0,3.0),
               Vec3::new(1.0,4.0,9.0));
}
#[test]
fn test_div() {
    assert_eq!(Vec3::new(1.0,2.0,3.0) / Vec3::new(1.0,2.0,3.0),
               Vec3::new(1.0,1.0,1.0));
}
