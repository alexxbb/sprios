use anyhow::Result;

use gdk_pixbuf::{Pixbuf, PixbufLoader, PixbufLoaderExt};
use gio::prelude::*;
use gtk::prelude::WidgetExtManual;
use gtk::{AdjustmentExt, Application, ApplicationWindow, Button, ContainerExt, Image, ImageExt, Layout, ScrollableExt, ScrolledWindow, ScrolledWindowExt, Viewport, WidgetExt, Scrollbar, Orientation, RangeExt, BoxExt};
use std::cell::Cell;
use std::rc::Rc;
use gdk::{Cursor, CursorType, WindowExt};

fn set_pan_cursor(window: &ApplicationWindow, active: bool) {
    let screen = window.get_screen().unwrap();
    let root_win = screen.get_root_window().unwrap();
    let mut cursor = None;
    if active {
        cursor = Some(Cursor::new_for_display(&window.get_display().unwrap(), CursorType::Hand2));
    }
    else {
        cursor = Some(Cursor::new_for_display(&window.get_display().unwrap(), CursorType::Arrow));
    }
    root_win.set_cursor(cursor.as_ref());
}

fn init(app: &Application) -> Result<()> {
    use gtk::WidgetExt;
    let buf = Pixbuf::new_from_file("/home/alex/Sandbox/rust/sprios/src/viewer/image.jpeg")?;
    let image_view = Image::new_from_pixbuf(Some(&buf));
    let window = ApplicationWindow::new(app);
    let viewport = Viewport::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
    let scroll = ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
    scroll.set_hexpand(true);
    scroll.set_vexpand(true);
    viewport.add(&image_view);
    scroll.add(&viewport);
    scroll.set_kinetic_scrolling(false);
    scroll.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
    scroll.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
    scroll.set_min_content_width(800);
    scroll.set_min_content_height(600);
    let mmb = Rc::new(Cell::new(Option::<(f64, f64)>::None));
    let cursor = Rc::new(Cell::new((0.0, 0.0)));
    let zoom = Rc::new(Cell::new(0f64));
    scroll.add_events(
        gdk::EventMask::BUTTON_PRESS_MASK
            | gdk::EventMask::BUTTON_RELEASE_MASK
            | gdk::EventMask::POINTER_MOTION_MASK,
    );
    let _mmb = Rc::clone(&mmb);
    scroll.connect_motion_notify_event(move |w, e| {
        let pos = e.get_position();
        if let Some(anchor) = _mmb.get() {
            let vadj = w.get_vadjustment().unwrap();
            let hadj = w.get_hadjustment().unwrap();
            hadj.set_value(hadj.get_value() + (cursor.get().0 - pos.0));
            vadj.set_value(vadj.get_value() + (cursor.get().1 - pos.1));
            w.set_hadjustment(Some(&hadj));
            w.set_vadjustment(Some(&vadj));
        }
        cursor.replace(pos);
        glib::signal::Inhibit(true)
    });
    let _mmb = Rc::clone(&mmb);
    let _win = window.clone();
    scroll.connect_button_press_event(move |_, e| {
        if matches!(e.get_button(), 2) {
            _mmb.replace(Some(e.get_position()));
            set_pan_cursor(&_win, true);
            glib::signal::Inhibit(true)
        } else {
            glib::signal::Inhibit(false)
        }
    });

    let _mmb = Rc::clone(&mmb);
    let _win = window.clone();
    scroll.connect_button_release_event(move |_, e| {
        if matches!(e.get_button(), 2) {
            _mmb.replace(None);
            set_pan_cursor(&_win, false);
            glib::signal::Inhibit(true)
        } else {
            glib::signal::Inhibit(false)
        }
    });
    let _zoom = Rc::clone(&zoom);
    scroll.connect_scroll_event(move |_, e| {
        let (_, scroll) = e.get_scroll_deltas().unwrap();
        let fac = 0.1 * scroll;
        zoom.replace(zoom.get() + fac);
        let z = (1.0 - zoom.get()).max(0.01);
        let w = (buf.get_width() as f64 * z).ceil() as i32;
        let h = (buf.get_height() as f64 * z).ceil() as i32;
        let pb = buf.scale_simple(w, h, gdk_pixbuf::InterpType::Tiles);
        image_view.set_from_pixbuf(pb.as_ref());
        glib::signal::Inhibit(true)
    });
    window.add(&scroll);
    window.set_property("allow-shrink", &false);
    window.show_all();
    Ok(())
}

fn main() {
    let application = Application::new(None, Default::default()).expect("Fail to init");
    application.connect_activate(|app| init(app).unwrap());
    application.run(&[]);
}
