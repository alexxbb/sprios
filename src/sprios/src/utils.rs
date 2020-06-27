use glib::ByteArray;
use glib::Bytes;


#[inline]
pub fn clip(val: f32, min: f32, max: f32) -> f32 {
    let mut x = val;
    if x < min {
        x = min;
    }
    if x > max {
        x = max;
    }
    x
}

pub fn convert_buffer(buf: &Vec<f32>, ss: u32) -> glib::Bytes {
    let gbuf = ByteArray::with_capacity(buf.len());
    let ss = ss as f32;
    for p in buf.chunks(3) {
            let r = (256.0 * clip(p[0].sqrt() / ss, 0.0, 0.999)) as u8;
            let g = (256.0 * clip(p[1].sqrt() / ss, 0.0, 0.999)) as u8;
            let b = (256.0 * clip(p[2].sqrt() / ss, 0.0, 0.999)) as u8;
            gbuf.append(&[r, g, b]);
    }
    gbuf.into_gbytes()
}

