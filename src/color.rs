use crate::vec::Color;
use crate::utils::Clip;


pub fn write_color(writer: &mut impl std::fmt::Write, clr: &Color, samples_per_pixel: u32) -> std::fmt::Result {
    let scale = 1.0 / samples_per_pixel as f32;

    let r = (clr.x * scale).sqrt();
    let g = (clr.y * scale).sqrt();
    let b = (clr.z * scale).sqrt();


    writeln!(writer, "{} {} {}",
             (256.0 * r.clip(0.0, 0.999)) as u32,
             (256.0 * g.clip(0.0, 0.999)) as u32,
             (256.0 * b.clip(0.0, 0.999)) as u32,
    )
}
