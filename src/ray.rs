use vec3::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    a: Vec3,
    b: Vec3
}

impl Ray {
    pub fn new(a: Vec3, b: Vec3) -> Ray { Ray { a, b } }
    //pub fn origin(&self) -> Vec3 { self.a.clone() }
    pub fn direction(&self) -> Vec3 { self.b.clone() }
    //pub fn point_at_parameter(&self, t: f64) -> Vec3 { return self.a + t * self.b; }
}
