use vec3::Vec3;
use ray::Ray;
use material::Material;

#[derive(Clone, Copy, Debug)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Material, // FIXME reference to avoid copy?
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            t: 0.0,
            p: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            material: Material::Lambertian {
                albedo: Vec3::new(0.0, 0.0, 0.0),
            },
        }
    }
}
