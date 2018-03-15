use hitable::HitRecord;
use vec3::{dot, random_in_unit_sphere, reflect, refract, unit_vector, Vec3};
use ray::Ray;
use rand;
use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub enum Material {
    Lambertian { albedo: Vec3 },
    Metal { albedo: Vec3, fuzz: f64 },
    Dielectric { ref_idx: f64 },
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

        Material::Dielectric { ref ref_idx } => {
            // initialize with first else clause
            let mut outward_normal = rec.normal;
            let mut ni_over_nt = 1.0 / *ref_idx;
            let mut cosine = -dot(&r_in.direction(), &rec.normal) / r_in.direction().length();
            let mut reflect_prob: f64;
            let reflected = reflect(&r_in.direction(), &rec.normal);
            *attenuation = Vec3::new(1.0, 1.0, 1.0); // intentional bug fixed
            let mut refracted = Vec3::new(0.0, 0.0, 0.0);
            if dot(&r_in.direction(), &rec.normal) > 0.0 {
                outward_normal = -rec.normal;
                ni_over_nt = *ref_idx;
                cosine = ref_idx * dot(&r_in.direction(), &rec.normal) / r_in.direction().length();
            } // else case in initialization
            if refract(
                &r_in.direction(),
                &outward_normal,
                ni_over_nt,
                &mut refracted,
            ) {
                reflect_prob = schlick(cosine, *ref_idx);
            } else {
                // no need for perf bug *scattered = Ray::new(rec.p, refracted);
                reflect_prob = 1.0;
            }
            let mut rng = rand::thread_rng(); // FIXME for perf
            if rng.gen::<f64>() < reflect_prob {
                *scattered = Ray::new(rec.p, reflected);
            } else {
                *scattered = Ray::new(rec.p, refracted);
            }
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
