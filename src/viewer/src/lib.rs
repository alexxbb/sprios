use gdk_pixbuf::{Pixbuf, PixbufLoader, PixbufLoaderExt};
use gio::prelude::*;
use gtk::prelude::WidgetExtManual;
use gtk::*;
use std::cell::{Cell, RefCell, RefMut};
use std::rc::Rc;
use gdk::{Cursor, CursorType, WindowExt};

fn set_pan_cursor(window: &ApplicationWindow, active: bool) {
    let screen = window.get_screen().unwrap();
    let root_win = screen.get_root_window().unwrap();
    let mut cursor = None;
    if active {
        cursor = Some(Cursor::new_for_display(&window.get_display().unwrap(), CursorType::Fleur));
    } else {
        cursor = Some(Cursor::new_for_display(&window.get_display().unwrap(), CursorType::Arrow));
    }
    root_win.set_cursor(cursor.as_ref());
}


pub struct ImageViewer {
    inner: Rc<RefCell<InnerImpl>>,
}

#[derive(Debug)]
struct InnerImpl {
    pub(crate) image: Image,
    pub(crate) source_buf: Option<RefCell<Pixbuf>>,
    pub(crate) scrollable: ScrolledWindow,
    pub(crate) zoom: Rc<Cell<f64>>,
    pub(crate) mmb: Rc<Cell<Option<(f64, f64)>>>,
    pub(crate) cursor: Rc<Cell<(f64, f64)>>,
}

impl InnerImpl {
    fn apply_scale(&self) {
        match self.source_buf.as_ref() {
            Some(source_buf) => {
                let zoom = self.zoom.get();
                let z = (1.0 - zoom).max(0.01);
                let w = (source_buf.borrow().get_width() as f64 * z).ceil() as i32;
                let h = (source_buf.borrow().get_height() as f64 * z).ceil() as i32;
                let scaled = source_buf.borrow().scale_simple(w, h, gdk_pixbuf::InterpType::Tiles).expect("Oops");
                self.image.set_from_pixbuf(Some(&scaled));
            }
            None => ()

        }
    }
}

impl ImageViewer {
    pub fn load_file(&self, f: &str) {
        let buf = Pixbuf::new_from_file(f).unwrap();
        self.load_pixbuf(&buf);
    }
    pub fn load_pixbuf(&self, buf: &Pixbuf) {
        self.inner.borrow_mut().scrollable.set_min_content_width(buf.get_width());
        self.inner.borrow_mut().scrollable.set_min_content_height(buf.get_height());
        self.inner.borrow_mut().source_buf.replace(RefCell::new((*buf).clone()));
        let scale = self.inner.borrow().zoom.get();
        if scale != 0.0 {
            self.inner.borrow().apply_scale();
        }
        else {
            self.inner.borrow().image.set_from_pixbuf(Some(&buf));
        }
    }

    pub fn root_widget(&self) -> ScrolledWindow {
        self.inner.borrow().scrollable.clone()
    }
}

impl ImageViewer {
    pub fn new(window: &gtk::ApplicationWindow) -> Self {
        let image = Image::new_from_pixbuf(None);
        let scrollable = ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
        image.set_hexpand(true);
        image.set_vexpand(true);
        scrollable.add(&image);
        scrollable.set_kinetic_scrolling(false);
        scrollable.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
        let mmb = Rc::new(Cell::new(Option::<(f64, f64)>::None));
        let cursor = Rc::new(Cell::new((0.0, 0.0)));
        let zoom = Rc::new(Cell::new(0f64));
        let inner = Rc::new(RefCell::new(InnerImpl {
            image,
            source_buf: None,
            scrollable,
            zoom,
            mmb,
            cursor,
        }));
        inner.borrow().scrollable.add_events(
            gdk::EventMask::BUTTON_PRESS_MASK
                | gdk::EventMask::BUTTON_RELEASE_MASK
                | gdk::EventMask::POINTER_MOTION_MASK,
        );
        let _mmb = Rc::clone(&inner.borrow().mmb);
        let _cursor = Rc::clone(&inner.borrow().cursor);
        inner.borrow().scrollable.connect_motion_notify_event(move |w, e| {
            let pos = e.get_position();
            if let Some(anchor) = _mmb.get() {
                let vadj = w.get_vadjustment().unwrap();
                let hadj = w.get_hadjustment().unwrap();
                hadj.set_value(hadj.get_value() + (_cursor.get().0 - pos.0));
                vadj.set_value(vadj.get_value() + (_cursor.get().1 - pos.1));
                w.set_hadjustment(Some(&hadj));
                w.set_vadjustment(Some(&vadj));
            }
            _cursor.replace(pos);
            glib::signal::Inhibit(true)
        });
        let _mmb = Rc::clone(&inner.borrow().mmb);
        let _win = window.clone();
        inner.borrow().scrollable.connect_button_press_event(move |_, e| {
            if matches!(e.get_button(), 2) {
                _mmb.replace(Some(e.get_position()));
                set_pan_cursor(&_win, true);
                glib::signal::Inhibit(true)
            } else {
                glib::signal::Inhibit(false)
            }
        });

        let _mmb = Rc::clone(&inner.borrow().mmb);
        let _win = window.clone();
        inner.borrow().scrollable.connect_button_release_event(move |_, e| {
            if matches!(e.get_button(), 2) {
                _mmb.replace(None);
                set_pan_cursor(&_win, false);
                glib::signal::Inhibit(true)
            } else {
                glib::signal::Inhibit(false)
            }
        });
        let _zoom = Rc::clone(&inner.borrow().zoom);
        let _view = inner.borrow().image.clone();
        let _inner = Rc::clone(&inner);
        inner.borrow().scrollable.connect_scroll_event(move |_, e| {
            let (_, zoom) = e.get_scroll_deltas().unwrap();
            let fac = 0.2 * zoom;
            let incr = _inner.borrow_mut().zoom.get() + fac;
            _inner.borrow_mut().zoom.replace(incr);
            _inner.borrow().apply_scale();
            let hadj = _inner.borrow().scrollable.get_hadjustment().unwrap();
            let vadj = _inner.borrow().scrollable.get_vadjustment().unwrap();
            let scaled_width = _inner.borrow().image.get_pixbuf().unwrap().get_width() as f64;
            let scaled_heigth = _inner.borrow().image.get_pixbuf().unwrap().get_height() as f64;
            let hv = (scaled_width - hadj.get_page_size()) / 2.0;
            let vv = (scaled_heigth - vadj.get_page_size()) / 2.0;
            hadj.set_value(hv);
            vadj.set_value(vv);
            glib::signal::Inhibit(true)
        });
        Self { inner }
    }
}
