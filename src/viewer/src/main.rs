use anyhow::Result;

use gio::prelude::*;
use gtk::{Application, ApplicationWindow, Button, ContainerExt, Image, ScrolledWindow, ScrolledWindowExt, Viewport, WidgetExt, ScrollableExt, AdjustmentExt};
use gdk_pixbuf::{PixbufLoader, PixbufLoaderExt, Pixbuf};
use gtk::prelude::WidgetExtManual;


fn init(app: &Application) -> Result<()> {
    use gtk::{WidgetExt};
    let window = ApplicationWindow::new(app);
    // let loader = PixbufLoader::new_with_type("jpeg")?;
    let scroll = ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
    scroll.set_hexpand(true);
    scroll.set_vexpand(true);
    let viewport = Viewport::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
    scroll.add(&viewport);
    scroll.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
    scroll.set_min_content_width(800);
    scroll.set_min_content_height(600);
    let buf = Pixbuf::new_from_file("/home/alex/Sandbox/rust/sprios/src/viewer/image.jpeg")?;
    let image = Image::new_from_pixbuf(Some(&buf));
    viewport.add(&image);
    viewport.add_events(gdk::EventMask::BUTTON_PRESS_MASK);
    let vp = viewport.clone();
    viewport.connect_scroll_event(move |w, e| {
        let (_, y) = e.get_scroll_deltas().unwrap();
        let adj = vp.get_vadjustment().unwrap();
        adj.set_value(adj.get_value() + y * 10.0);
        glib::signal::Inhibit(true)
    });
    viewport.connect_button_press_event(|w, e| {
        if matches!(e.get_button(), 2){
            println!("Mid clicked");
            glib::signal::Inhibit(true)
        }
        else {
            glib::signal::Inhibit(false)
        }
    });
    window.add(&scroll);
    window.show_all();
    Ok(())
}

fn main() {
    let application = Application::new(
        None,
        Default::default()).expect("Fail to init");
    application.connect_activate(|app| init(app).unwrap());
    application.run(&[]);
}
