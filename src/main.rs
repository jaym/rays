use std::io::{self, Write};

fn write_ppm<W: Write>(out: &mut W, nx: i32, ny: i32) -> std::io::Result<()> {
    write!(out, "P3\n{} {}\n255\n", nx, ny)?;

    for j in (0..ny).rev() {
        for i in 0..nx {
            let r = (i as f64) / (nx as f64);
            let g = (j as f64) / (ny as f64);
            let b = 0.2;

            let ir = (255.99 * r) as u8;
            let ig = (255.99 * g) as u8;
            let ib = (255.99 * b) as u8;

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
