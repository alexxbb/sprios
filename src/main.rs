#![allow(dead_code)]
#![allow(unused)]

mod camera;
mod color;
mod hittable;
mod imagebuffer;
mod material;
mod ray;
mod sphere;
mod utils;
mod vec;

use gdk_pixbuf::{Colorspace, Pixbuf, PixbufLoader};
use getopts;
use gio::prelude::*;
use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, Box as GtkBox, BoxExt, Button, Image, ImageBuilder, Label,
    Orientation, Paned, ProgressBar, SpinButton,
};
use rand::Rng;
use std::rc::Rc;
use std::time::Instant;

use crate::material::{Lambertian, Metal};
use crate::utils::Clip;
use camera::Camera;
use hittable::Hittable;
use imagebuffer::ImageBuffer;
use ray::Ray;
use sphere::Sphere;
use std::cell::RefCell;
use std::io::{Read, Write};
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};
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
    window.set_title("SPRIOS");
    window.set_default_size(800, 400);

    let render_btn = Button::new_with_label("Render");
    let split = Paned::new(Orientation::Horizontal);
    let num_samples = SpinButton::new_with_range(1.0, 200.0, 5.0);
    let samples_label = Label::new(Some("Samples"));
    let image = Image::new();
    let progress = ProgressBar::new();
    progress.set_fraction(0.5);

    let left_panel = GtkBox::new(Orientation::Vertical, 0);
    let samples_box = GtkBox::new(Orientation::Horizontal, 0);
    samples_box.pack_start(&samples_label, false, false, 3);
    samples_box.pack_start(&num_samples, false, false, 3);

    left_panel.pack_start(&samples_box, false, true, 3);
    left_panel.pack_end(&render_btn, false, true, 3);
    left_panel.pack_end(&progress, false, true, 3);
    split.add1(&left_panel);
    split.add2(&image);

    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    let image_width: u32 = 386;
    let image_height: u32 = (image_width as f32 / ASPECT_RATIO) as u32;
    let cap = (image_height * image_width * 3) as usize;
    let mut buf = Arc::new(Mutex::new(ImageBuffer::new(
        image_width,
        image_height,
        Vec::with_capacity(cap),
    )));
    let buf_rc = Arc::clone(&buf);
    let image_c = image.clone();
    render_btn.connect_clicked(move |_| {
        buf_rc.lock().unwrap().clear();
        let samples = num_samples.get_value() as u32;
        let buf_rc2 = Arc::clone(&buf_rc);
        std::thread::spawn(move || {
            render(image_width, image_height, samples, buf_rc2);
        })
        .join();
        use gdk_pixbuf::PixbufLoaderExt;
        use glib::Bytes;
        let bytes = Bytes::from(buf_rc.lock().unwrap().as_ref());
        let loader = PixbufLoader::new_with_type("pnm").unwrap();
        loader.write(format!("P6\n{} {}\n255\n", image_width, image_height).as_bytes());
        loader
            .write_bytes(&bytes)
            .expect("Could not write to buffer");
        loader.close();
        image_c.set_from_pixbuf(loader.get_pixbuf().as_ref());
    });

    window.add(&split);
    window.show_all()
}

fn render(width: u32, height: u32, samples: u32, mut buf: Arc<Mutex<ImageBuffer>>) {
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

    let mut buf = buf.lock().unwrap();
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
