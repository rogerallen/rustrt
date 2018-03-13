// cargo run > out.ppm
extern crate rand;

mod camera;
mod hitable;
mod hitable_list;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use hitable::{HitRecord, Hitable};
use hitable_list::HitableList;
use ray::Ray;
use sphere::Sphere;
use vec3::{unit_vector, Vec3};
use std::f64;
use rand::Rng;

fn color(r: &Ray, world: &HitableList) -> Vec3 {
    let mut rec = HitRecord::new();
    if world.hit(r, 0.0, f64::MAX, &mut rec) {
        0.5
            * Vec3::new(
                rec.normal.x() + 1.0,
                rec.normal.y() + 1.0,
                rec.normal.z() + 1.0,
            )
    } else {
        let unit_direction = unit_vector(r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let nx = 200;
    let ny = 100;
    let ns = 100;
    println!("P3\n{0} {1} 255", nx, ny);
    let mut world = HitableList::new();
    world.push(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    world.push(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));
    let cam = Camera::new();
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _s in 0..ns {
                let u = (i as f64 + rng.gen::<f64>()) / (nx as f64);
                let v = (j as f64 + rng.gen::<f64>()) / (ny as f64);
                let r = cam.get_ray(u,v);
                // ??? let p = r.point_at_parameter(2.0);
                col += color(&r, &world);
            }
            col /= ns as f64;
            let ir = (255.99 * col[0]) as i32;
            let ig = (255.99 * col[1]) as i32;
            let ib = (255.99 * col[2]) as i32;
            println!("{0} {1} {2}", ir, ig, ib);
        }
    }
}
