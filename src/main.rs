// cargo run > out.ppm
mod vec3;
use vec3::{Vec3,unit_vector};
mod ray;
use ray::Ray;

fn color(r: &Ray) -> Vec3 {
    let unit_direction = unit_vector(r.direction());
    let t = 0.5*(unit_direction.y() + 1.0);
    (1.0-t)*Vec3::new(1.0, 1.0, 1.0) + t*Vec3::new(0.5, 0.7, 1.0)
}

fn main() {
    let nx = 200;
    let ny = 100;
    println!("P3\n{0} {1} 255",nx,ny);
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);
    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = (i as f64)/(nx as f64);
            let v = (j as f64)/(ny as f64);
            let r = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical);
            let col = color(&r);
            let ir = (255.99*col[0]) as i32;
            let ig = (255.99*col[1]) as i32;
            let ib = (255.99*col[2]) as i32;
            println!("{0} {1} {2}",ir,ig,ib);
        }
    }
}
