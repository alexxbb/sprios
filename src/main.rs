use std::io::Write;

const HEIGHT: u32 = 256;
const WIDTH: u32 = 256;
use std::thread::sleep;

fn main() {
    println!("P3\n{} {}\n255", WIDTH, HEIGHT);
    for i in 0..HEIGHT {
        sleep(std::time::Duration::from_millis(20));
        eprint!("\rLines remaining: {} ", i);
        std::io::stderr().flush();
        for j in 0..WIDTH {
            let r = i as f32 / (WIDTH - 1) as f32;
            let g = j as f32 / (HEIGHT - 1) as f32;
            let b = 0.25f32;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}