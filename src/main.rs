// cargo run > out.ppm
// Chapter 1 - done
mod vec3;
use vec3::Vec3;

fn main() {
    let nx = 200;
    let ny = 100;
    println!("P3\n{0} {1} 255",nx,ny);
    for j in (0..ny).rev() {
        for i in 0..nx {
            let col = Vec3::new( (i as f32)/(nx as f32), (j as f32)/(ny as f32), 0.2 as f32 );
            let ir = (255.99*col[0]) as i32;
            let ig = (255.99*col[1]) as i32;
            let ib = (255.99*col[2]) as i32;
            println!("{0} {1} {2}",ir,ig,ib);
        }
    }
}
