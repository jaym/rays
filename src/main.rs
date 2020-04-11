use rays::ray;
use rays::vec::Vec3;
use std::io::{self, Write};

fn hit_sphere(center: Vec3, radius: f32, r: &ray::Ray) -> bool {
    // The point P given by (x,y,z) is on a sphere with radius R
    // and center C given by (c_x, c_y, c_z)
    //   (x - c_x)^2 + (y - c_y)^2 + (z - c_z)^2 = R^2
    // In vector form:
    //   (p - c) * (p-c) = R^2
    // Our ray is described by p(t)
    // We can substitue in p(t):
    //  (p(t) - c) * (p(t) - c) = R^2
    //  => (a + tb - c) * (a + tb - c) - R^2 = 0
    //  => [b*b]t^2 + [2b*(a-c)]t +[(a-c)*(a-c)-R^2] = 0
    // The discrimant is [2b*(a-c)]-4[(b*b)((a-c)*(a-c)-R^2)]
    // If the discrimant is less than 0, there are no solutions. The ray
    // does not intersect the sphere.
    // If the discriminant is greater 0, there are 2 solutions
    // If the discriminant is zero, there is one solution

    let oc = r.origin - center;
    let a = r.direction.dot(r.direction);
    let b = 2.0 * r.direction.dot(oc);
    let c = oc.dot(oc) - radius * radius;
    let discrimant = b * b - 4.0 * a * c;

    return discrimant > 0.0;
}

fn ray_color(r: &ray::Ray) -> Vec3 {
    if hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, r) {
        return Vec3::new(1.0, 0.0, 0.0);
    }
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
