use ray::Ray;
use rand::Rng;
use vec3::{cross, random_in_unit_disk, unit_vector, Vec3};
use std;

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f64,
        aspect: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let theta = vfov * std::f64::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = unit_vector(lookfrom - lookat);
        let u = unit_vector(cross(&vup, &w));
        let v = cross(&w, &u);
        Camera {
            origin: lookfrom,
            lower_left_corner: lookfrom - half_width * focus_dist * u - half_height * focus_dist * v
                - focus_dist * w,
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            u: u,
            v: v,
            w: w,
            lens_radius: aperture / 2.0,
        }
    }
    pub fn get_ray<R: Rng>(&self, s: f64, t: f64, rng: &mut R) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk(rng);
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}
