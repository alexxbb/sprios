use gdk_pixbuf::PixbufLoader;
use gio::ApplicationExt;
use glib::{clone};
use glib::signal::Inhibit;
use gtk::{
    ApplicationWindow, Box as GtkBox, BoxExt, Button, ButtonExt, ContainerExt, GtkWindowExt, Image,
    ImageExt, Label, LabelExt, Orientation, Paned, PanedExt, ProgressBar, ProgressBarExt,
    SpinButton, SpinButtonExt, WidgetExt,
};
use num_cpus;
use renderer::{render, RenderStats};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::AtomicPtr;
use std::sync::{Arc};
use threadpool::{ThreadPool};

#[derive(Copy, Clone)]
pub enum Event {
    Progress(u32),
    RenderCompleted(RenderStats),
}


pub struct App {
    pub window: ApplicationWindow,
}

impl App {
    pub fn new(gtk_app: &gtk::Application) -> Rc<App> {
        let window = ApplicationWindow::new(gtk_app);
        window.set_title("SPRIOS");
        window.set_default_size(920, 470);
        gtk::Window::set_default_icon_name("face-smirk");
        Rc::new(App { window })
    }

    pub fn build_ui(&self) {
        let render_btn = Button::new_with_label("Render");
        let split = Paned::new(Orientation::Horizontal);
        // Samples
        let num_samples = SpinButton::new_with_range(1.0, 500.0, 5.0);
        num_samples.set_value(3.0);
        let samples_label = Label::new(Some("Samples"));

        // Bucket size
        let bucket_size = SpinButton::new_with_range(4.0, 100.0, 4.0);
        let bucket_label = Label::new(Some("Bucket"));
        bucket_size.set_value(16.0);

        // Number of threads
        let max_threads = num_cpus::get_physical();
        let num_threads = SpinButton::new_with_range(1.0, max_threads as f64, 1.0);
        let num_threads_label = Label::new(Some("Threads"));
        num_threads.set_value(max_threads as f64);

        // Resolution
        let res_width = SpinButton::new_with_range(10.0, 2048.0, 100.0);
        let res_width_label = Label::new(Some("Width"));
        res_width.set_value(720.0);

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
        samples_box.pack_start(&num_samples, false, false, 3);

        let bucket_box = GtkBox::new(Orientation::Horizontal, 0);
        bucket_box.pack_start(&bucket_label, false, false, 3);
        bucket_box.pack_start(&bucket_size, false, false, 3);

        let res_box = GtkBox::new(Orientation::Horizontal, 0);
        res_box.pack_start(&res_width_label, false, false, 3);
        res_box.pack_start(&res_width, false, false, 3);

        let thread_box = GtkBox::new(Orientation::Horizontal, 0);
        thread_box.pack_start(&num_threads_label, false, false, 3);
        thread_box.pack_start(&num_threads, false, false, 3);

        left_panel.pack_start(&samples_box, false, true, 3);
        left_panel.pack_start(&thread_box, false, true, 3);
        left_panel.pack_start(&bucket_box, false, true, 3);
        left_panel.pack_start(&res_box, false, true, 3);
        left_panel.pack_end(&render_btn, false, true, 3);
        left_panel.pack_end(&progress, false, true, 3);
        split.add1(&left_panel);
        split.add2(&right_panel);

        const ASPECT_RATIO: f32 = 16.0 / 9.0;
        let image_buffer: Vec<u8> = Vec::new();
        let (s, r) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
        let progress_clone = progress.clone();
        let image_buf = Rc::new(RefCell::new(image_buffer));
        let thread_pool = RefCell::new(ThreadPool::new(num_cpus::get_physical()));
        render_btn.connect_clicked(
            clone!(@weak image_buf, @weak res_width, @strong thread_pool => move |_| {
            let image_width = res_width.get_value() as u32;
            let image_height = (image_width as f32 / ASPECT_RATIO) as u32;
            let cap = (image_height * image_width * 3) as usize;
            image_buf.borrow_mut().resize(cap, 0);
            progress_clone.set_fraction(0.0);
            let samples = num_samples.get_value() as u32;
            let bucket_size = bucket_size.get_value() as u32;
            let buffer_ptr = Arc::new(AtomicPtr::new(image_buf.borrow_mut().as_mut_ptr()));
            thread_pool.borrow_mut().set_num_threads(num_threads.get_value() as usize);
            std::thread::spawn(
                clone!(@strong s, @strong thread_pool => move || {
                let stats = render(
                    image_width,
                    image_height,
                    samples,
                    bucket_size,
                    buffer_ptr,
                    Some(&thread_pool.borrow()),
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
                    use gdk_pixbuf::PixbufLoaderExt;
                    use glib::Bytes;
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
