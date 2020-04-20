use rand::Rng;
use rays::camera::{self, Camera};
use rays::geom;
use rays::ray;
use rays::vec::Vec3;
use std::io::{self, Write};

type HittableList = Vec<Box<dyn geom::Hittable>>;

fn rand() -> (f32, f32) {
    rand::thread_rng().gen()
}

fn ray_color(r: &ray::Ray, hittables: &HittableList, depth: i32) -> Vec3 {
    if depth <= 0 {
        Vec3::zeros()
    } else {
        hittables
            .iter()
            .fold(None, |last_hit: Option<geom::HitRecord>, hittable| {
                let t_max = last_hit.map_or(f32::INFINITY, |h| h.t);
                hittable.hit(r, t_max).or(last_hit)
            })
            //.map(|h| (h.normal + 1.0) * 0.5)
            .map(|h| {
                let (albedo, next_ray) = h.mat.scatter(r, &h.intersection, &h.normal);
                ray_color(&next_ray, hittables, depth - 1) * albedo
            })
            .unwrap_or_else(|| {
                let direction = r.direction.unit();
                let t = 0.5 * (direction.y() + 1.0);
                Vec3::ones() * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
            })
    }

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
    let samples_per_pixel = 50;
    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = (i as f32) / (nx as f32);
            let v = (j as f32) / (ny as f32);

            let mut c = Vec3::zeros();

            for k in 0..samples_per_pixel {
                let (ur, vr) = rand();
                let r = cam.get_ray(u + ur / (nx as f32), v + vr / (ny as f32));
                c = c + ray_color(&r, hittables, 50);
            }
            c = c / (samples_per_pixel as f32);

            let ir = (255.99 * c.r().sqrt()) as u8;
            let ig = (255.99 * c.g().sqrt()) as u8;
            let ib = (255.99 * c.b().sqrt()) as u8;

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
            mat: geom::Lambertian {
                albedo: Vec3::new(0.8, 0.1, 0.1),
            },
        }),
        Box::new(geom::Sphere {
            center: Vec3::new(0.7, 0.0, -1.5),
            radius: 0.3,
            mat: geom::Lambertian {
                albedo: Vec3::new(1.0, 1.0, 1.0),
            },
        }),
        Box::new(geom::Sphere {
            center: Vec3::new(0.0, -100.5, -1.0),
            radius: 100.0,
            mat: geom::Lambertian {
                albedo: Vec3::new(0.3, 0.8, 0.1),
            },
        }),
    ];

    write_ppm(&mut out, nx, ny, &objects)?;

    Ok(())
}
