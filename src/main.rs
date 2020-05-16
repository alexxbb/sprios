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
use gtk::{Application, ApplicationWindow, Button, Image};
use rand::Rng;
use std::rc::Rc;
use std::time::Instant;

use crate::material::{Lambertian, Metal};
use camera::Camera;
use hittable::Hittable;
use ray::Ray;
use sphere::Sphere;
use std::cell::RefCell;
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

fn build_ui(app: &gtk::Application) {
    let window = ApplicationWindow::new(app);
    window.set_title("Hello");
    // window.set_default_size(800, 400);

    let render_btn = Button::new_with_label("Render");
    const image_width: u32 = 386;
    const aspect_ratio: f32 = 16.0 / 9.0;
    const samples_per_pixel: u32 = 10;
    let image_height: u32 = (image_width as f32 / aspect_ratio) as u32;
    let cap = image_height * image_width * (std::mem::size_of::<u32>() * 3) as u32;
    let mut buf = Rc::new(RefCell::new(ImageBuffer {
        inner: Vec::with_capacity(cap as usize),
    }));
    render_btn.connect_clicked(move |_| {
        render(image_width, image_height, samples_per_pixel, buf.clone());
    });

    // let image = Image::new_from_file("image.ppm");

    let buf = Pixbuf::new(
        Colorspace::Rgb,
        true,
        8,
        image_width as i32,
        image_height as i32,
    );

    window.add(&render_btn);
    window.show_all()
}

struct ImageBuffer {
    inner: Vec<f32>,
}

impl AsRef<[f32]> for ImageBuffer {
    fn as_ref(&self) -> &[f32] {
        &self.inner
    }
}

impl ImageBuffer {
    pub fn write_color(&mut self, clr: &Color, samples_per_pixel: u32) {
        let scale = 1.0 / samples_per_pixel as f32;

        let r = (clr.x * scale).sqrt();
        let g = (clr.y * scale).sqrt();
        let b = (clr.z * scale).sqrt();

        self.inner.push(r);
        self.inner.push(g);
        self.inner.push(b);
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
    println!("Render done");
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
