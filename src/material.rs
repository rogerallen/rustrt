use hitable::HitRecord;
use vec3::{dot, random_in_unit_sphere, reflect, unit_vector, Vec3};
use ray::Ray;

#[derive(Clone, Copy, Debug)]
pub enum Material {
    Lambertian { albedo: Vec3 },
    Metal { albedo: Vec3, fuzz: f64 },
}

pub fn scatter(r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
    match rec.material {
        Material::Lambertian { ref albedo } => {
            let target = rec.p + rec.normal + random_in_unit_sphere();
            *scattered = Ray::new(rec.p, target - rec.p);
            *attenuation = *albedo;
            true
        }
        Material::Metal {
            ref albedo,
            ref fuzz,
        } => {
            let reflected = reflect(&unit_vector(r_in.direction()), &rec.normal);
            *scattered = Ray::new(rec.p, reflected + *fuzz * random_in_unit_sphere());
            *attenuation = *albedo;
            dot(&scattered.direction(), &rec.normal) > 0.0
        }
    }
}
