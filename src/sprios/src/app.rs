use gtk::{ApplicationWindow, Box as GtkBox, BoxExt,
          Button, GtkWindowExt, Image, Label, Orientation,
          Paned, ProgressBar, SpinButton, ProgressBarExt,
          PanedExt, ContainerExt, ButtonExt, ImageExt, SpinButtonExt, WidgetExt};
use gio::ApplicationExt;
use renderer::{render, ImageBuffer};
use std::sync::{Arc, Mutex};
use std::cell::RefCell;
use std::rc::Rc;
use gdk_pixbuf::{Colorspace, Pixbuf, PixbufLoader};

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
