use ray::Ray;
use hitable;
use sphere::Sphere;

// Ideally, HitableList is just Vec<Box<Hitable>>
// But, I don't know how to do that in Rust yet
pub struct HitableList {
    spheres: Vec<Sphere>, // FIXME generalize, but that's another book
}

impl HitableList {
    pub fn new() -> HitableList {
        HitableList {
            spheres: Vec::new(),
        }
    }
    pub fn push(&mut self, sphere: Sphere) {
        self.spheres.push(sphere);
    }
}

impl hitable::Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut hitable::HitRecord) -> bool {
        let mut temp_rec = hitable::HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for i in 0..self.spheres.len() {
            if self.spheres[i].hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                // Hmm, seems like something to encapsulate (FIXME)
                // could not just do rec = temp_rec (or variants)
                rec.t = temp_rec.t;
                rec.p = temp_rec.p;
                rec.normal = temp_rec.normal;
            }
        }
        hit_anything
    }
}
