use rays::ray;
use rays::vec::Vec3;
use std::io::{self, Write};

fn ray_color(r: &ray::Ray) -> Vec3 {
    let direction = r.direction.unit();
    let t = 0.5 * (direction.y() + 1.0);
    Vec3::ones() * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn write_ppm<W: Write>(out: &mut W, nx: i32, ny: i32) -> std::io::Result<()> {
    write!(out, "P3\n{} {}\n255\n", nx, ny)?;
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = (i as f32) / (nx as f32);
            let v = (j as f32) / (ny as f32);
            let r = ray::Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);
            let c = ray_color(&r);

            let ir = (255.99 * c.r()) as u8;
            let ig = (255.99 * c.g()) as u8;
            let ib = (255.99 * c.b()) as u8;

            write!(out, "{} {} {}\n", ir, ig, ib)?;
        }
    }
    Ok(())
}
fn main() -> std::io::Result<()> {
    let mut out = io::stdout();
    let nx = 200;
    let ny = 100;

    write_ppm(&mut out, nx, ny)?;

    Ok(())
}
