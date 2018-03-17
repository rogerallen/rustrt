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
use rand::{Rng, SeedableRng, StdRng};
use ray::Ray;
use sphere::Sphere;
use std::f64;
use vec3::{unit_vector, Vec3};

fn color<R: Rng>(r: &Ray, world: &HitableList, depth: i32, rng: &mut R) -> Vec3 {
    let mut rec = HitRecord::new();
    if world.hit(r, 0.001, f64::MAX, &mut rec) {
        let mut scattered = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
        let mut attenuation = Vec3::new(0.0, 0.0, 0.0);
        if depth < 50 && scatter(r, &mut rec, &mut attenuation, &mut scattered, rng) {
            attenuation * color(&scattered, world, depth + 1, rng)
        } else {
            Vec3::new(0.0, 0.0, 0.0)
        }
    } else {
        let unit_direction = unit_vector(r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

fn original_scene(world: &mut HitableList) -> &HitableList {
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
    world
}

fn redblue_scene(world: &mut HitableList) -> &HitableList {
    let r = (std::f64::consts::PI / 4.0).cos();
    world.push(Sphere::new(
        Vec3::new(-r, 0.0, -1.0),
        r,
        Material::Lambertian {
            albedo: Vec3::new(0.0, 0.0, 1.0),
        },
    ));
    world.push(Sphere::new(
        Vec3::new(r, 0.0, -1.0),
        r,
        Material::Lambertian {
            albedo: Vec3::new(1.0, 0.0, 0.0),
        },
    ));
    world
}

fn final_scene<'a, R: Rng>(world: &'a mut HitableList, rng: &mut R) -> &'a HitableList {
    //let mut rng = rand::thread_rng();
    world.push(Sphere::new(
        Vec3::new(0., -1000., 0.),
        1000.,
        Material::Lambertian {
            albedo: Vec3::new(0.5, 0.5, 0.5),
        },
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f64>();
            let center = Vec3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );
            if (center - Vec3::new(4., 0.2, 0.)).length() > 0.9 {
                if choose_mat < 0.8 {
                    world.push(Sphere::new(
                        center,
                        0.2,
                        Material::Lambertian {
                            albedo: Vec3::new(
                                rng.gen::<f64>() * rng.gen::<f64>(),
                                rng.gen::<f64>() * rng.gen::<f64>(),
                                rng.gen::<f64>() * rng.gen::<f64>(),
                            ),
                        },
                    ));
                } else if choose_mat < 0.95 {
                    world.push(Sphere::new(
                        center,
                        0.2,
                        Material::Metal {
                            albedo: Vec3::new(
                                0.5 * (1. + rng.gen::<f64>()),
                                0.5 * (1. + rng.gen::<f64>()),
                                0.5 * (1. + rng.gen::<f64>()),
                            ),
                            fuzz: 0.5 * rng.gen::<f64>(),
                        },
                    ));
                } else {
                    world.push(Sphere::new(
                        center,
                        0.2,
                        Material::Dielectric { ref_idx: 1.5 },
                    ));
                }
            }
        }
    }

    world.push(Sphere::new(
        Vec3::new(0., 1., 0.),
        1.,
        Material::Dielectric { ref_idx: 1.5 },
    ));
    world.push(Sphere::new(
        Vec3::new(-4., 1., 0.),
        1.,
        Material::Lambertian {
            albedo: Vec3::new(0.4, 0.2, 0.1),
        },
    ));
    world.push(Sphere::new(
        Vec3::new(4., 1., 0.),
        1.,
        Material::Metal {
            albedo: Vec3::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        },
    ));

    world
}

fn main() {
    //let mut rng = rand::thread_rng();
    let seed: &[_] = &[1, 2, 3, 4];
    let mut rng: StdRng = SeedableRng::from_seed(seed);

    let nx = 1200;
    let ny = 800;
    let ns = 10;
    println!("P3\n{0} {1} 255", nx, ny);

    let mut the_world = HitableList::new();
    //let world = original_scene(&mut world);
    //let world = redblue_scene(&mut world);
    let world = final_scene(&mut the_world, &mut rng);

    let lookfrom = Vec3::new(13., 2., 3.);
    let lookat = Vec3::new(0., 0., 0.);
    let dist_to_focus = 10.0; //(lookfrom - lookat).length();
    let aperture = 0.1;

    let cam = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0., 1., 0.),
        30.0,
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
                let r = cam.get_ray(u, v, &mut rng);
                // ??? let p = r.point_at_parameter(2.0);
                col += color(&r, &world, 0, &mut rng);
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
