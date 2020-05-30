use crate::worlds::*;
use gdk_pixbuf::{PixbufLoader};
use gio::ApplicationExt;
use glib::{clone};
use glib::signal::Inhibit;
use gtk::{
    ApplicationWindow, Box as GtkBox, BoxExt, Button, ButtonExt, ContainerExt, GtkWindowExt, Image,
    ImageExt, Label, LabelExt, Orientation, Paned, PanedExt, ProgressBar, ProgressBarExt,
    SpinButton, SpinButtonExt, WidgetExt,
};
use gdk_pixbuf::PixbufLoaderExt;
use glib::Bytes;
use num_cpus;
use renderer::{render, RenderStats, SettingsBuilder, Camera, Vec3, Point3};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::AtomicPtr;
use std::sync::{Arc};
use threadpool::{ThreadPool};

static LOGO: &[u8;33647] = &include_bytes!("../rust-logo.png");

#[derive(Copy, Clone)]
pub enum Event {
    Progress(u32),
    RenderCompleted(RenderStats),
}


pub struct App {
    pub window: ApplicationWindow,
    world: Arc<World>,
}

impl App {
    pub fn new(gtk_app: &gtk::Application) -> Rc<App> {
        let window = ApplicationWindow::new(gtk_app);
        window.set_title("SPRIOS");
        window.set_default_size(920, 470);
        gtk::Window::set_default_icon_name("face-smirk");
        let world = world_book();
        Rc::new(App { window, world: Arc::new(world) })
    }

    pub fn build_ui(&self) {
        let render_btn = Button::new_with_label("Render");
        let split = Paned::new(Orientation::Horizontal);
        // Samples
        let num_samples = SpinButton::new_with_range(1.0, 32.0, 1.0);
        num_samples.set_value(3.0);
        let samples_label = Label::new(Some("Samples"));

        // Bucket size
        let bucket_size = SpinButton::new_with_range(4.0, 100.0, 4.0);
        let bucket_label = Label::new(Some("Bucket"));
        bucket_size.set_value(16.0);

        // Number of threads
        let max_threads = num_cpus::get();
        let num_threads = SpinButton::new_with_range(1.0, max_threads as f64, 1.0);
        let num_threads_label = Label::new(Some("Threads"));
        num_threads.set_value(max_threads as f64);

        // Resolution
        let res_width = SpinButton::new_with_range(10.0, 2048.0, 100.0);
        let res_width_label = Label::new(Some("Width"));
        res_width.set_value(720.0);

        // Camera
        let aperture = SpinButton::new_with_range(0.0, 2.0, 0.1);
        let aperture_label = Label::new(Some("Aperture"));
        aperture.set_value(0.2);

        let fov = SpinButton::new_with_range(10.0, 50.0, 1.0);
        let fov_label = Label::new(Some("FOV"));
        fov.set_value(35.0);



        // Logo
        let logo = Image::new();
        let loader = PixbufLoader::new_with_type("png").unwrap();
        loader.write_bytes(&Bytes::from_static(LOGO)).unwrap();
        loader.close().unwrap();
        let pixbuf = loader.get_pixbuf().unwrap();
        let pixbuf = pixbuf.scale_simple(120, 120, gdk_pixbuf::InterpType::Bilinear);
        logo.set_from_pixbuf(pixbuf.as_ref());

        let stat_label = Label::new(None);
        let gtk_image = Image::new();
        let progress = ProgressBar::new();
        progress.set_show_text(true);

        let right_panel = GtkBox::new(Orientation::Vertical, 0);
        let status_box = GtkBox::new(Orientation::Horizontal, 0);
        status_box.pack_start(&stat_label, false, false, 3);
        right_panel.pack_start(&gtk_image, true, false, 3);
        right_panel.pack_start(&status_box, false, false, 3);

        let left_panel = GtkBox::new(Orientation::Vertical, 0);

        let samples_box = GtkBox::new(Orientation::Horizontal, 0);
        samples_box.pack_start(&samples_label, false, false, 3);
        samples_box.pack_start(&num_samples, true, true, 3);

        let bucket_box = GtkBox::new(Orientation::Horizontal, 0);
        bucket_box.pack_start(&bucket_label, false, false, 3);
        bucket_box.pack_start(&bucket_size, true, true, 3);

        let res_box = GtkBox::new(Orientation::Horizontal, 0);
        res_box.pack_start(&res_width_label, false, false, 3);
        res_box.pack_start(&res_width, true, true, 3);

        let aperture_box = GtkBox::new(Orientation::Horizontal, 0);
        aperture_box.pack_start(&aperture_label, false, false, 3);
        aperture_box.pack_start(&aperture, true, true, 3);

        let fov_box = GtkBox::new(Orientation::Horizontal, 0);
        fov_box.pack_start(&fov_label, false, false, 3);
        fov_box.pack_start(&fov, true, true, 3);

        let thread_box = GtkBox::new(Orientation::Horizontal, 0);
        thread_box.pack_start(&num_threads_label, false, false, 3);
        thread_box.pack_start(&num_threads, true, true, 3);

        left_panel.pack_start(&samples_box, false, true, 3);
        left_panel.pack_start(&thread_box, false, true, 3);
        left_panel.pack_start(&bucket_box, false, true, 3);
        left_panel.pack_start(&res_box, false, true, 3);
        left_panel.pack_start(&fov_box, false, true, 3);
        left_panel.pack_start(&aperture_box, false, true, 3);
        left_panel.pack_end(&render_btn, false, true, 3);
        left_panel.pack_end(&progress, false, true, 3);
        left_panel.pack_end(&logo, false, true, 3);
        split.add1(&left_panel);
        split.add2(&right_panel);

        const ASPECT_RATIO: f32 = 16.0 / 9.0;
        let image_buffer: Vec<u8> = Vec::new();
        let (s, r) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
        let progress_clone = progress.clone();
        let image_buf = Rc::new(RefCell::new(image_buffer));
        let thread_pool = RefCell::new(ThreadPool::new(num_cpus::get_physical()));
        let world = Arc::clone(&self.world);
        render_btn.connect_clicked(
            clone!(@weak image_buf,
                     @weak res_width,
                     @strong thread_pool,
                     @strong world,
                     @weak fov,
                     @weak aperture => move |_| {
            let settings = SettingsBuilder::new()
                            .bucket(bucket_size.get_value() as u32)
                            .size(res_width.get_value() as u32, None)
                            .samples(num_samples.get_value() as u32).build();

            let cap = (settings.width * settings.height * 3) as usize;
            image_buf.borrow_mut().resize(cap, 0);
            progress_clone.set_fraction(0.0);
            let buffer_ptr = Arc::new(AtomicPtr::new(image_buf.borrow_mut().as_mut_ptr()));
            let lookfrom = Point3::new(3.0, 0.8, 0.0);
            let lookat = Point3::new(0.0, 0.0, -1.0);
            let foc_dist = (&lookfrom - &lookat).length();

            let camera = Arc::new(Camera::new(
                lookfrom,
                lookat,
                Vec3::new(0.0, 1.0, 0.0),
                fov.get_value() as u32,
                settings.width as f32 / settings.height as f32,
                aperture.get_value() as f32,
                foc_dist));

            thread_pool.borrow_mut().set_num_threads(num_threads.get_value() as usize);
            std::thread::spawn(
                clone!(@strong s, @strong thread_pool, @strong world => move || {
                let stats = render(
                    settings,
                    buffer_ptr,
                    Some(&thread_pool.borrow()),
                    world,
                    camera,
                    clone!(@strong s => move |prog| {
                        s.send(Event::Progress(prog)).unwrap();
                    }),
                );
                s.send(Event::RenderCompleted(stats)).unwrap();
            }));
        }));
        r.attach(None, clone!(@strong image_buf, @strong gtk_image => move |event| {
            match event {
                Event::Progress(val) => {
                    let frac = val as f64 / 100 as f64;
                    progress.set_fraction(frac);
                }
                Event::RenderCompleted(stat) => {
                    let bytes = Bytes::from(&image_buf.borrow().as_ref());
                    let loader = PixbufLoader::new_with_type("pnm").unwrap();
                    let image_width = res_width.get_value() as u32;
                    let image_height = (image_width as f32 / ASPECT_RATIO) as u32;
                    loader.write(format!("P6\n{} {}\n255\n", image_width, image_height).as_bytes()).unwrap();
                    loader
                        .write_bytes(&bytes)
                        .expect("Could not write to buffer");
                    loader.close().unwrap();
                    gtk_image.set_from_pixbuf(loader.get_pixbuf().as_ref());
                    stat_label.set_text(&format!(
                        "Time: {:.4} sec | FPS: {:.4} | MRays/s: {:.2}",
                        stat.render_time, stat.fps, stat.mrays
                    ));
                }
            }
            glib::Continue(true)
        }));

        self.window.add(&split);
    }

    fn on_activate(&self) {
        self.build_ui();
        self.window.show_all();
    }

    pub fn on_startup(gtk_app: &gtk::Application) {
        let app = App::new(gtk_app);

        let app_c = Rc::clone(&app);
        gtk_app.connect_activate(move |_| {
            app_c.on_activate();
        });
        let app_c = gtk_app.clone();
        app.window.connect_key_press_event(move |_w, key| {
            if matches!(key.get_keyval(), gdk::enums::key::Escape) {
                app_c.quit();
            }
            Inhibit(false)
        });
    }
}
