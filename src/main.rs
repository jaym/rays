use rays::camera::{self, Camera};
use rays::geom;
use rays::ray;
use rays::vec::Vec3;
use std::io::{self, Write};

type HittableList = Vec<Box<dyn geom::Hittable>>;

fn rand(u: f32, v: f32) -> f32 {
    let u = u + 0.39;
    let v = v + 0.73;
    return ((u * 1031.1031 * v * 369.11369 + v * 369.11369 * u * 1031.1031) + 19.19 / 7.33)
        .fract();
}

fn ray_color(r: &ray::Ray, hittables: &HittableList) -> Vec3 {
    hittables
        .iter()
        .fold(None, |last_hit: Option<geom::HitRecord>, hittable| {
            let t_max = last_hit.map_or(f32::INFINITY, |h| h.t);
            hittable.hit(r, t_max).or(last_hit)
        })
        .map(|h| (h.normal + 1.0) * 0.5)
        .unwrap_or_else(|| {
            let direction = r.direction.unit();
            let t = 0.5 * (direction.y() + 1.0);
            Vec3::ones() * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
        })

    //Vec3::zeros()
}

fn write_ppm<W: Write>(
    out: &mut W,
    nx: i32,
    ny: i32,
    hittables: &HittableList,
) -> std::io::Result<()> {
    write!(out, "P3\n{} {}\n255\n", nx, ny)?;
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let cam = camera::Simple::new(origin, 4.0, 2.0, -1.0);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = (i as f32) / (nx as f32);
            let v = (j as f32) / (ny as f32);
            let r = cam.get_ray(u, v);
            let c = ray_color(&r, hittables);

            let c = Vec3::new(rand(u, v), rand(u, v), rand(u, v));
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
    let nx = 400;
    let ny = 200;
    let objects: HittableList = vec![
        Box::new(geom::Sphere {
            center: Vec3::new(0.0, 0.0, -1.5),
            radius: 0.5,
        }),
        Box::new(geom::Sphere {
            center: Vec3::new(0.0, 0.0, -0.5),
            radius: 0.1,
        }),
        Box::new(geom::Sphere {
            center: Vec3::new(0.0, 100.5, -1.0),
            radius: 100.0,
        }),
        Box::new(geom::Sphere {
            center: Vec3::new(0.0, -100.5, -1.0),
            radius: 100.0,
        }),
    ];

    write_ppm(&mut out, nx, ny, &objects)?;

    Ok(())
}
