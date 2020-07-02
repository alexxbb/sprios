use anyhow::Result;

use gdk_pixbuf::{Pixbuf, PixbufLoader, PixbufLoaderExt};
use gio::prelude::*;
use gtk::prelude::WidgetExtManual;
use gtk::{
    AdjustmentExt, Application, ApplicationWindow, Button, ContainerExt, Image, ScrollableExt,
    ScrolledWindow, ScrolledWindowExt, Viewport, WidgetExt,
};
use std::cell::Cell;
use std::rc::Rc;

fn init(app: &Application) -> Result<()> {
    use gtk::WidgetExt;
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
    let mmb = Rc::new(Cell::new(Option::<(f64, f64)>::None));
    viewport.add(&image);
    viewport.add_events(
        gdk::EventMask::BUTTON_PRESS_MASK
            | gdk::EventMask::BUTTON_RELEASE_MASK
            | gdk::EventMask::POINTER_MOTION_MASK);
    let _mmb = Rc::clone(&mmb);
    let _vp = viewport.clone();
    viewport.connect_motion_notify_event(move |w, e| {
        let pos = e.get_position();
        if let Some(anchor) = _mmb.get() {
            let dx = pos.0 - anchor.0;
            let dy = pos.1 - anchor.1;
            let vadj = _vp.get_vadjustment().unwrap();
            let hadj = _vp.get_hadjustment().unwrap();
            hadj.set_value(hadj.get_value() + dx * -0.2);
            vadj.set_value(vadj.get_value() + dy * -0.2);
        }
        glib::signal::Inhibit(true)
    });
    let _mmb = Rc::clone(&mmb);
    viewport.connect_button_press_event(move |w, e| {
        if matches!(e.get_button(), 2) {
            _mmb.replace(Some(e.get_position()));
            glib::signal::Inhibit(true)
        } else {
            glib::signal::Inhibit(false)
        }
    });

    let _mmb = Rc::clone(&mmb);
    viewport.connect_button_release_event(move |w, e| {
        if matches!(e.get_button(), 2) {
            _mmb.replace(None);
            glib::signal::Inhibit(true)
        } else {
            glib::signal::Inhibit(false)
        }
    });
    let vp = viewport.clone();
    viewport.connect_scroll_event(move |w, e| {
        let (_, scroll) = e.get_scroll_deltas().unwrap();
        glib::signal::Inhibit(true)
    });
    window.add(&scroll);
    window.show_all();
    Ok(())
}

fn main() {
    let application = Application::new(None, Default::default()).expect("Fail to init");
    application.connect_activate(|app| init(app).unwrap());
    application.run(&[]);
}
