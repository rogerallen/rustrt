use std::ops::Index;

pub struct Vec3 {
    e: [f32; 3]
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }
    /* unused for the moment
    pub fn x(&self) -> f32 { self.e[0] }
    pub fn y(&self) -> f32 { self.e[1] }
    pub fn z(&self) -> f32 { self.e[2] }
    pub fn r(&self) -> f32 { self.e[0] }
    pub fn g(&self) -> f32 { self.e[1] }
    pub fn b(&self) -> f32 { self.e[2] }
     */
}

impl Index<usize> for Vec3 {
    type Output = f32;
    fn index(&self, i: usize) -> &f32 { &self.e[i] }
}
