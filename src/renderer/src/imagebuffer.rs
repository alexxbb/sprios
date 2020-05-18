use crate::utils::Clip;
use crate::vec::Color;
use std::io::BufWriter;
use std::io::Write;

pub struct ImageBuffer {
    inner: Vec<u8>,
    width: u32,
    height: u32,
}

impl AsRef<[u8]> for ImageBuffer {
    fn as_ref(&self) -> &[u8] {
        &self.inner
    }
}

impl<'a> Into<&'a [u8]> for &'a ImageBuffer {
    fn into(self) -> &'a [u8] {
        self.inner.as_slice()
    }
}

impl ImageBuffer {
    pub fn new(width: u32, height: u32, buf: impl Into<Vec<u8>>) -> ImageBuffer {
        ImageBuffer {
            inner: buf.into(),
            width,
            height,
        }
    }
    pub fn len(&self) -> usize {
        self.inner.len()
    }
    pub fn write_color(&mut self, clr: &Color, samples_per_pixel: u32) {
        let scale = 1.0 / samples_per_pixel as f32;

        let r = (clr.x * scale).sqrt();
        let g = (clr.y * scale).sqrt();
        let b = (clr.z * scale).sqrt();

        self.inner.push((256.0 * r.clip(0.0, 0.999)) as u8);
        self.inner.push((256.0 * g.clip(0.0, 0.999)) as u8);
        self.inner.push((256.0 * b.clip(0.0, 0.999)) as u8);
    }

    pub fn clear(&mut self) {
        self.inner.clear()
    }

    pub fn debug(&self) {
        use std::fs::File;
        let f = File::create("image.ppm").expect("Could not create ppm");
        let mut buf = BufWriter::with_capacity(self.inner.len(), &f);

        writeln!(buf, "P3\n{} {}\n255", self.width, self.height).unwrap();

        for mut i in 0..self.inner.len() / 3 {
            i *= 3;
            buf.write_fmt(format_args!(
                "{} {} {}\n",
                self.inner[i],
                self.inner[i + 1],
                self.inner[i + 2]
            ))
            .unwrap();
        }
        buf.flush().unwrap();
    }
}
