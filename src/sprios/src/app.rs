use gtk::{ApplicationWindow, Box as GtkBox, BoxExt,
          Button, GtkWindowExt, Image, Label, LabelExt, Orientation,
          Paned, ProgressBar, SpinButton, ProgressBarExt,
          PanedExt, ContainerExt, ButtonExt, ImageExt, SpinButtonExt, WidgetExt};
use gio::ApplicationExt;
use renderer::{render, ImageBuffer};
use std::sync::{Arc, Mutex};
use std::rc::Rc;
use gdk_pixbuf::{PixbufLoader};

#[derive(Copy, Clone)]
pub enum Event {
    Progress((u32, u32)),
    Done,
}

pub struct App {
    pub window: ApplicationWindow,
}

impl App {
    pub fn new(gtk_app: &gtk::Application) -> Rc<App> {
        let window = ApplicationWindow::new(gtk_app);
        window.set_title("SPRIOS");
        window.set_default_size(800, 400);
        Rc::new(App { window })
    }

    pub fn build_ui(&self) {
        let render_btn = Button::new_with_label("Render");
        let split = Paned::new(Orientation::Horizontal);
        let num_samples = SpinButton::new_with_range(1.0, 200.0, 5.0);
        let samples_label = Label::new(Some("Samples"));
        let stat_label = Label::new(None);
        let image = Image::new();
        let progress = ProgressBar::new();
        progress.set_show_text(true);
        // progress.set_fraction(0.0);

        let right_panel = GtkBox::new(Orientation::Vertical, 0);
        let status_box = GtkBox::new(Orientation::Horizontal, 0);
        status_box.pack_start(&stat_label, false, false, 3);
        right_panel.pack_start(&image, true, false, 3);
        right_panel.pack_start(&status_box, false, false, 3);

        let left_panel = GtkBox::new(Orientation::Vertical, 0);
        let samples_box = GtkBox::new(Orientation::Horizontal, 0);
        samples_box.pack_start(&samples_label, false, false, 3);
        samples_box.pack_start(&num_samples, false, false, 3);

        left_panel.pack_start(&samples_box, false, true, 3);
        left_panel.pack_end(&render_btn, false, true, 3);
        left_panel.pack_end(&progress, false, true, 3);
        split.add1(&left_panel);
        split.add2(&right_panel);

        const ASPECT_RATIO: f32 = 16.0 / 9.0;
        let image_width: u32 = 480;
        let image_height: u32 = (image_width as f32 / ASPECT_RATIO) as u32;
        let cap = (image_height * image_width * 3) as usize;
        let buf = Arc::new(Mutex::new(ImageBuffer::new(
            image_width,
            image_height,
            Vec::with_capacity(cap),
        )));
        let buf_rc = Arc::clone(&buf);
        let image_c = image.clone();
        let (s, r) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
        let progress_clone = progress.clone();
        render_btn.connect_clicked(move |_| {
            buf_rc.lock().unwrap().clear();
            progress_clone.set_fraction(0.0);
            let samples = num_samples.get_value() as u32;
            let buf_rc2 = Arc::clone(&buf_rc);
            let s = s.clone();
            let s2 = s.clone();
            std::thread::spawn(move || {
                render(image_width, image_height, samples, buf_rc2, move |prog, pix| {
                    s2.send(Event::Progress((prog, pix)));
                });
                s.send(Event::Done);
            });
        });
        r.attach(None, move |event| {
            match event {
                Event::Progress(val) => {
                    let frac = val.0 as f64 / 100 as f64;
                    progress.set_fraction(frac);
                    stat_label.set_text(&format!("Pixel Time: {} ms", val.1));
                }
                Event::Done => {
                    use gdk_pixbuf::PixbufLoaderExt;
                    use glib::Bytes;
                    let bytes = Bytes::from(buf.lock().unwrap().as_ref());
                    let loader = PixbufLoader::new_with_type("pnm").unwrap();
                    loader.write(format!("P6\n{} {}\n255\n", image_width, image_height).as_bytes());
                    loader
                        .write_bytes(&bytes)
                        .expect("Could not write to buffer");
                    loader.close();
                    image_c.set_from_pixbuf(loader.get_pixbuf().as_ref());
                }
            }
            glib::Continue(true)
        });

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
    }
}

impl Drop for App {
    fn drop(&mut self) {
        eprintln!("Dropping app");
    }
}
