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

#[inline]
fn convert(v: f32) -> u8 {
    (256.0 * clip(v, 0.0, 0.999)) as u8
}

pub fn convert_buffer(buf: &Vec<f32>, ss: u32) -> glib::Bytes {
    let gbuf = ByteArray::with_capacity(buf.len());
    let ss = ss as f32;

    for p in buf.chunks(3) {
        gbuf.append(&[
            convert((p[0] / ss).sqrt()),
            convert((p[1] / ss).sqrt()),
            convert((p[2] / ss).sqrt())
        ]);
    }
    gbuf.into_gbytes()
}

