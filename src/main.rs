use std::io::{self, Write};

fn main() -> std::io::Result<()> {
    let mut out = io::stdout();
    let nx = 200;
    let ny = 100;

    write!(out, "P3\n{} {}\n255\n", nx, ny)?;

    for j in (0..ny).rev() {
        for i in 0..nx {
            let r = (i as f64) / (nx as f64);
            let g = (j as f64) / (ny as f64);
            let b = 0.2;

            let ir: u8 = (256. * r) as u8;
            let ig = (256. * g) as u8;
            let ib = (256. * b) as u8;

            write!(out, "{} {} {}\n", ir, ig, ib)?
        }
    }
    Ok(())
}
