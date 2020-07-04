use anyhow::Result;

use gdk_pixbuf::{Pixbuf, PixbufLoader, PixbufLoaderExt};
use gio::prelude::*;
use gtk::prelude::WidgetExtManual;
use gtk::{AdjustmentExt, Application, ApplicationWindow, Button, ContainerExt, Image, ScrollableExt, ScrolledWindow, ScrolledWindowExt, Viewport, WidgetExt, ImageExt};
use std::cell::Cell;
use std::rc::Rc;

fn init(app: &Application) -> Result<()> {
    use gtk::WidgetExt;
    let window = ApplicationWindow::new(app);
    // let loader = PixbufLoader::new_with_type("jpeg")?;
    let scroll = ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
    let viewport = Viewport::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
    scroll.set_kinetic_scrolling(true);
    scroll.set_hexpand(true);
    scroll.set_vexpand(true);
    scroll.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
    scroll.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
    scroll.set_min_content_width(800);
    scroll.set_min_content_height(600);
    scroll.add(&viewport);
    let buf = Pixbuf::new_from_file("/home/alex/Sandbox/rust/sprios/src/viewer/image.jpeg")?;
    let image = Image::new_from_pixbuf(Some(&buf));
    let mmb = Rc::new(Cell::new(Option::<(f64, f64)>::None));
    let zoom = Rc::new(Cell::new(0f64));
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
    let _zoom = Rc::clone(&zoom);
    viewport.connect_scroll_event(move |w, e| {
        let (_, scroll) = e.get_scroll_deltas().unwrap();
        let fac = 0.1 * scroll;
        zoom.replace(zoom.get() + fac);
        let z = (1.0 - zoom.get()).max(0.01);
        let w = (buf.get_width() as f64 * z).ceil() as i32;
        let h = (buf.get_height() as f64 * z).ceil() as i32;
        let pb = buf.scale_simple(w, h, gdk_pixbuf::InterpType::Tiles);
        image.set_from_pixbuf(pb.as_ref());
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
