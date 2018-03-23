use hitable::HitRecord;
use vec3::{dot, random_in_unit_sphere, reflect, refract, unit_vector, Vec3};
use ray::Ray;
use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub enum Material {
    Lambertian { albedo: Vec3 },
    Metal { albedo: Vec3, fuzz: f64 },
    Dielectric { ref_idx: f64 },
}

pub fn scatter<R: Rng>(
    r_in: &Ray,
    rec: &HitRecord,
    attenuation: &mut Vec3,
    scattered: &mut Ray,
    rng: &mut R,
) -> bool {
    match rec.material {
        Material::Lambertian { ref albedo } => {
            let target = rec.p + rec.normal + random_in_unit_sphere(rng);
            *attenuation = *albedo;
            *scattered = Ray::new(rec.p, target - rec.p);
            true
        }

        Material::Metal {
            ref albedo,
            ref fuzz,
        } => {
            let reflected = reflect(&unit_vector(r_in.direction), &rec.normal);
            *attenuation = *albedo;
            *scattered = Ray::new(rec.p, reflected + *fuzz * random_in_unit_sphere(rng));
            dot(&scattered.direction, &rec.normal) > 0.0
        }

        Material::Dielectric { ref ref_idx } => {
            // are we entering or exiting the material?
            let entering = dot(&r_in.direction, &rec.normal) > 0.0;
            let mut outward_normal = if entering {
                -rec.normal
            } else {
                rec.normal
            };
            let mut ni_over_nt = if entering { *ref_idx } else { 1.0 / *ref_idx };
            let mut cosine = if entering {
                ref_idx * dot(&r_in.direction, &rec.normal) / r_in.direction.length()
            } else {
                -dot(&r_in.direction, &rec.normal) / r_in.direction.length()
            };

            let mut refracted = Vec3::new(0.0, 0.0, 0.0);
            let mut reflect_prob = if refract(
                &r_in.direction,
                &outward_normal,
                ni_over_nt,
                &mut refracted,
            ) {
                schlick(cosine, *ref_idx)
            } else {
                1.0
            };

            *attenuation = Vec3::new(1.0, 1.0, 1.0);
            *scattered = if rng.gen::<f64>() < reflect_prob {
                let reflected = reflect(&r_in.direction, &rec.normal);
                Ray::new(rec.p, reflected)
            } else {
                Ray::new(rec.p, refracted)
            };
            true
        }
    }
}

// for dielectric calc
fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}
