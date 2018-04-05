// cargo run > out.ppm
extern crate rand;
extern crate rayon;

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
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use std::time::Instant;

fn color<R: Rng>(
    r: &Ray,
    world: &HitableList,
    depth: i32,
    rng: &mut R,
    ray_count: &mut i32,
) -> Vec3 {
    let mut rec = HitRecord::new();
    if world.hit(r, 0.001, f64::MAX, &mut rec) {
        let mut scattered = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
        *ray_count += 1;
        let mut attenuation = Vec3::new(0.0, 0.0, 0.0);
        if depth < 50 && scatter(r, &rec, &mut attenuation, &mut scattered, rng) {
            attenuation * color(&scattered, world, depth + 1, rng, ray_count)
        } else {
            Vec3::new(0.0, 0.0, 0.0)
        }
    } else {
        let unit_direction = unit_vector(r.direction);
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[allow(dead_code)]
fn final_scene<'a, R: Rng>(world: &'a mut HitableList, rng: &mut R) -> &'a HitableList {
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
                f64::from(a) + 0.9 * rng.gen::<f64>(),
                0.2,
                f64::from(b) + 0.9 * rng.gen::<f64>(),
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
    let seed: &[_] = &[1984];
    let mut rng: StdRng = SeedableRng::from_seed(seed);

    const NX: usize = 1440;
    const NY: usize = 720;
    const NS: i32 = 32;

    eprintln!("rendering {}x{} image with {} samples/pixel", NX, NY, NS);
    let mut the_world = HitableList::new();
    //let world = original_scene(&mut the_world);
    //let world = redblue_scene(&mut the_world);
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
        NX as f64 / NY as f64,
        aperture,
        dist_to_focus,
    );

    let num_rays = Arc::new(Mutex::new(0));
    // use thread per row for concurrency.
    let mut framebuffer = vec![[[0.0f64; 3]; NX]; NY];
    let render_start = Instant::now();
    framebuffer
        .par_iter_mut() // rayon speedup here
        .enumerate()
        .map(|(j, framebuffer_row): (usize, &mut [[f64; 3]; NX])| {
            let seed2: &[_] = &[1984 + j];
            let mut rng2: StdRng = SeedableRng::from_seed(seed2);
            let num_rays = Arc::clone(&num_rays);
            let mut row_rays = 0;
            for i in 0..NX {
                let mut col = Vec3::new(0.0, 0.0, 0.0);
                for _s in 0..NS {
                    let u = (i as f64 + rng2.gen::<f64>()) / (NX as f64);
                    let v = (j as f64 + rng2.gen::<f64>()) / (NY as f64);
                    let r = cam.get_ray(u, v, &mut rng2);
                    row_rays += 1;
                    col += color(&r, world, 0, &mut rng2, &mut row_rays);
                }
                (*framebuffer_row)[i][0] = col[0];
                (*framebuffer_row)[i][1] = col[1];
                (*framebuffer_row)[i][2] = col[2];
            }
            let mut guard_num_rays = num_rays.lock().unwrap();
            *guard_num_rays += row_rays;
            framebuffer_row
        })
        .collect::<Vec<_>>();
    let render_dur = render_start.elapsed();
    let render_secs = render_dur.as_secs() as f64 + 1e-9 * (render_dur.subsec_nanos() as f64);
    let safe_num_rays = *num_rays.lock().unwrap() as f64;
    let rays_per_sec = safe_num_rays / render_secs;

    eprintln!(
        "rays = {}, time = {:.1}s rays/sec = {:.0}",
        safe_num_rays, render_secs, rays_per_sec
    );

    println!("P3\n{0} {1} 255", NX, NY);
    for j in (0..NY).rev() {
        for i in 0..NX {
            // final div by samples & gamma correction
            let ri = (255.99 * (framebuffer[j][i][0] / f64::from(NS)).sqrt()) as u8;
            let gi = (255.99 * (framebuffer[j][i][1] / f64::from(NS)).sqrt()) as u8;
            let bi = (255.99 * (framebuffer[j][i][2] / f64::from(NS)).sqrt()) as u8;
            println!("{0} {1} {2}", ri, gi, bi);
        }
    }
}
