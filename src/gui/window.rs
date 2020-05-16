use gtk::{Application, ApplicationWindow, Image};
use gtk::prelude::*;
use gio::prelude::*;


fn build_ui(app: &gtk::Application) {
    let window = ApplicationWindow::new(app);
    window.set_title("Hello");
    // window.set_default_size(800, 400);

    let image = Image::new_from_file("image.ppm");
    window.add(&image);
    window.show_all()
}

fn main() {
    let app = gtk::Application::new(Some("sprios.dev"), Default::default()).expect("Fail to init");
    app.connect_activate(|app|{
        build_ui(app);
    });
    app.run(&[]);
}