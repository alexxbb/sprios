#![allow(dead_code)]
#![allow(unused)]

mod camera;
mod color;
mod hittable;
mod material;
mod ray;
mod sphere;
mod utils;
mod vec;

use gdk_pixbuf::{Colorspace, Pixbuf};
use getopts;
use gio::prelude::*;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Image, Orientation};
use rand::Rng;
use std::rc::Rc;
use std::time::Instant;

use crate::material::{Lambertian, Metal};
use crate::utils::Clip;
use camera::Camera;
use hittable::Hittable;
use ray::Ray;
use sphere::Sphere;
use std::cell::RefCell;
use std::io::{BufWriter, Read, Write};
use std::ops::DerefMut;
use vec::*;

type World = Vec<Rc<dyn Hittable>>;

fn ray_color(ray: &Ray, world: &World, depth: u32) -> Color {
    if depth == 0 {
        // Max recursion depth reached
        return Color::ZERO;
    }

    if let Some(rec) = world.hit(ray, 0.001, f32::INFINITY) {
        if let Some(ray) = rec.mat.scatter(ray, &rec) {
            return rec.mat.color() * ray_color(&ray, world, depth - 1);
        }
        return Color::ZERO;
    }
    let dir = ray.direction.unit();
    let t = 0.5 * (dir.y + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

#[allow(non_upper_case_globals)]
fn build_ui(app: &gtk::Application) {
    let window = ApplicationWindow::new(app);
    window.set_title("Hello");
    // window.set_default_size(800, 400);

    let render_btn = Button::new_with_label("Render");
    const aspect_ratio: f32 = 16.0 / 9.0;
    const samples_per_pixel: u32 = 2;
    let image_width: u32 = 386;
    let image_height: u32 = (image_width as f32 / aspect_ratio) as u32;
    let cap = (image_height * image_width * 3) as usize;
    let mut buf = Rc::new(RefCell::new(ImageBuffer::new(
        image_width,
        image_height,
        Vec::with_capacity(cap),
    )));
    let buf_rc = Rc::clone(&buf);
    let image = Image::new();
    let image_c = image.clone();
    render_btn.connect_clicked(move |_| {
        buf_rc.borrow_mut().clear();
        render(image_width, image_height, samples_per_pixel, buf_rc.clone());
        assert_eq!(buf_rc.borrow().as_ref().len(), cap);
        buf_rc.borrow_mut().debug();
        use glib::Bytes;
        let bytes = Bytes::from(buf_rc.borrow().as_ref());
        let buf = Pixbuf::new_from_bytes(
            &bytes,
            Colorspace::Rgb,
            false,
            8 as i32,
            image_width as i32,
            image_height as i32,
            3,
        );
        image_c.set_from_pixbuf(Some(&buf));
    });

    use gtk::BoxExt;
    let vb = gtk::Box::new(Orientation::Vertical, 0);
    vb.pack_start(&render_btn, true, true, 3);
    vb.pack_end(&image, true, true, 3);
    window.add(&vb);
    window.show_all()
}

struct ImageBuffer {
    inner: Vec<u8>,
    width: u32,
    height: u32,
}

impl AsRef<[u8]> for ImageBuffer {
    fn as_ref(&self) -> &[u8] {
        &self.inner
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
        let mut f = File::create("image.ppm").expect("Could not create ppm");
        let mut buf = BufWriter::with_capacity(self.inner.len(), &f);

        writeln!(buf, "P3\n{} {}\n255", self.width, self.height);

        for i in 0..self.inner.len() - 2 {
            buf.write_fmt(format_args!(
                "{} {} {}\n",
                self.inner[i],
                self.inner[i + 1],
                self.inner[i + 2]
            ));
        }
        buf.flush();
    }
}

#[allow(non_upper_case_globals)]
fn render(width: u32, height: u32, samples: u32, mut buf: Rc<RefCell<ImageBuffer>>) {
    const max_depth: u32 = 10;
    let camera = Camera::new();
    let mut world = World::new();
    // Globe sphere
    world.push(Rc::new(Sphere::new(
        (0.0, -100.5, -1.0),
        100.0,
        Rc::new(Lambertian {
            color: (0.5, 0.5, 0.5).into(),
        }),
    )));
    // Red
    world.push(Rc::new(Sphere::new(
        (-1.0, 0.0, -1.0),
        0.5,
        Rc::new(Metal {
            color: (0.9, 0.1, 0.1).into(),
        }),
    )));
    // Green
    world.push(Rc::new(Sphere::new(
        (1.0, 0.0, -1.0),
        0.5,
        Rc::new(Metal {
            color: (0.1, 0.9, 0.1).into(),
        }),
    )));
    // Blue
    world.push(Rc::new(Sphere::new(
        (0.0, 0.0, -1.0),
        0.5,
        Rc::new(Metal {
            color: (0.1, 0.1, 0.9).into(),
        }),
    )));

    let mut buf = buf.borrow_mut();
    let mut buf = buf.deref_mut();
    let mut rng = rand::thread_rng();
    for i in (0..height).rev() {
        let cur_line = height - i;
        let prog = (cur_line as f32 / height as f32) * 100.0;
        eprint!("\rRendering: {}%", prog as u32);
        use std::io::Write;
        std::io::stderr().flush().unwrap();
        for j in 0..width {
            let mut pixel_color = Color::ZERO;
            for _ in 0..samples {
                let u = (j as f32 + rng.gen::<f32>()) / (width - 1) as f32;
                let v = (i as f32 + rng.gen::<f32>()) / (height - 1) as f32;
                let ray = camera.get_ray(u, v);
                pixel_color += &ray_color(&ray, &world, max_depth);
            }
            buf.write_color(&pixel_color, samples);
        }
    }
}

#[allow(non_upper_case_globals)]
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let app = gtk::Application::new(Some("sprios.dev"), Default::default()).expect("Fail to init");
    app.connect_activate(|app| {
        build_ui(app);
    });
    app.run(&args);
}
