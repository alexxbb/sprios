mod vec;
mod ray;

use std::fmt::Write as FmWrite;
use std::io::Write;
use std::thread::sleep;
use std::error::Error;
use vec::*;

const HEIGHT: u32 = 256;
const WIDTH: u32 = 256;

fn main() -> Result<(), Box<dyn Error>> {
    let cap = HEIGHT * WIDTH * (std::mem::size_of::<u32>() * 3) as u32;
    let mut buf = String::with_capacity(cap as usize);
    writeln!(&mut buf, "P3\n{} {}\n255", WIDTH, HEIGHT)?;
    for i in (0..HEIGHT).rev() {
        eprint!("\rLines remaining: {} ", i);
        std::io::stderr().flush()?;
        for j in 0..WIDTH {
            let color = Color::new(
                (j as f32 / (HEIGHT - 1) as f32) as f32,
                (i as f32 / (WIDTH - 1) as f32) as f32,
                0.25,
            );
            write_color(&mut buf, &color)?;
        }
    }
    std::fs::write("image.ppm", &buf).unwrap();
    Ok(())
}