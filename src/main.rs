// cargo run > out.ppm
extern crate rand;

mod camera;
mod hitable;
mod hitable_list;
mod material;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use hitable::{HitRecord, Hitable};
use hitable_list::HitableList;
use material::{scatter, Material};
use rand::Rng;
use ray::Ray;
use sphere::Sphere;
use std::f64;
use vec3::{unit_vector, Vec3};

fn color(r: &Ray, world: &HitableList, depth: i32) -> Vec3 {
    let mut rec = HitRecord::new();
    if world.hit(r, 0.001, f64::MAX, &mut rec) {
        let mut scattered = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
        let mut attenuation = Vec3::new(0.0, 0.0, 0.0);
        if depth < 50 && scatter(r, &mut rec, &mut attenuation, &mut scattered) {
            attenuation * color(&scattered, world, depth + 1)
        } else {
            Vec3::new(0.0, 0.0, 0.0)
        }
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
    /* original world */
    world.push(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        Material::Lambertian {
            albedo: Vec3::new(0.1, 0.3, 0.5),
        },
    ));
    world.push(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Material::Lambertian {
            albedo: Vec3::new(0.8, 0.8, 0.0),
        },
    ));
    world.push(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        Material::Metal {
            albedo: Vec3::new(0.8, 0.6, 0.2),
            fuzz: 1.0,
        },
    ));
    world.push(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        //Material::Metal {
        //    albedo: Vec3::new(0.8, 0.8, 0.8),
        //    fuzz: 0.3,
        //},
        Material::Dielectric { ref_idx: 1.5 },
    ));
    world.push(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        -0.45, // makes a bubble inside sphere
        Material::Dielectric { ref_idx: 1.5 },
    ));
    /* red/blue ball world
    let R = (std::f64::consts::PI/4.0).cos();
    world.push(Sphere::new(
        Vec3::new(-R, 0.0, -1.0),
        R,
        Material::Lambertian {
            albedo: Vec3::new(0.0, 0.0, 1.0),
        },
    ));
    world.push(Sphere::new(
        Vec3::new(R, 0.0, -1.0),
        R,
        Material::Lambertian {
            albedo: Vec3::new(1.0, 0.0, 0.0),
        },
    ));
     */
    let lookfrom = Vec3::new(3., 3., 2.);
    let lookat = Vec3::new(0., 0., -1.);
    let dist_to_focus = (lookfrom - lookat).length();
    let aperture = 2.0;
    let cam = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0., 1., 0.),
        20.0,
        nx as f64 / ny as f64,
        aperture,
        dist_to_focus,
    );
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _s in 0..ns {
                let u = (i as f64 + rng.gen::<f64>()) / (nx as f64);
                let v = (j as f64 + rng.gen::<f64>()) / (ny as f64);
                let r = cam.get_ray(u, v);
                // ??? let p = r.point_at_parameter(2.0);
                col += color(&r, &world, 0);
            }
            col /= ns as f64;
            col = Vec3::new(col[0].sqrt(), col[1].sqrt(), col[2].sqrt());
            let ir = (255.99 * col[0]) as i32;
            let ig = (255.99 * col[1]) as i32;
            let ib = (255.99 * col[2]) as i32;
            println!("{0} {1} {2}", ir, ig, ib);
        }
    }
}
