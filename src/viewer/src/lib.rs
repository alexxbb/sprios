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
    pub(crate) zoom_buf: Option<RefCell<Pixbuf>>,
    pub(crate) scrollable: ScrolledWindow,
    pub(crate) zoom: Rc<Cell<f64>>,
    pub(crate) mmb: Rc<Cell<Option<(f64, f64)>>>,
    pub(crate) cursor: Rc<Cell<(f64, f64)>>,
}

impl InnerImpl {
    fn scale_buf(&self, zoom: f64) {
        match self.zoom_buf.as_ref() {
            Some(zoom_buf) => {
                let fac = 0.1 * zoom;
                let incr = self.zoom.get() + fac;
                self.zoom.replace(incr);
                let z = (1.0 - incr).max(0.01);
                let w = (zoom_buf.borrow().get_width() as f64 * z).ceil() as i32;
                let h = (zoom_buf.borrow().get_height() as f64 * z).ceil() as i32;
                let scaled = zoom_buf.borrow().scale_simple(w, h, gdk_pixbuf::InterpType::Tiles).expect("Oops");
                zoom_buf.replace(scaled);
                // Something is wrong here
                todo!();
                self.image.set_from_pixbuf(Some(&zoom_buf.borrow()))
            }
            None => ()
        }
    }
}

impl ImageViewer {
    pub fn load_file(&self, f: &str) {
        let buf = Pixbuf::new_from_file(f).unwrap();
        self.inner.borrow().image.set_from_pixbuf(Some(&buf));
        self.inner.borrow_mut().zoom_buf.replace(RefCell::new(buf.clone()));
    }
    pub fn load_pixbuf(&self, buf: Option<&Pixbuf>) {
        self.inner.borrow().image.set_from_pixbuf(buf);
        if let Some(b) = buf.as_ref() {
            self.inner.borrow_mut().zoom_buf.replace(RefCell::new((*b).clone()));
        }
    }

    pub fn root_widget(&self) -> ScrolledWindow {
        self.inner.borrow().scrollable.clone()
    }
}

impl ImageViewer {
    pub fn new(window: &gtk::ApplicationWindow) -> Self {
        let image = Image::new_from_pixbuf(None);
        let viewport = Viewport::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
        let scrollable = ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
        scrollable.set_hexpand(true);
        scrollable.set_vexpand(true);
        viewport.add(&image);
        scrollable.add(&viewport);
        scrollable.set_kinetic_scrolling(false);
        scrollable.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
        scrollable.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
        scrollable.set_min_content_width(800);
        scrollable.set_min_content_height(600);
        let mmb = Rc::new(Cell::new(Option::<(f64, f64)>::None));
        let cursor = Rc::new(Cell::new((0.0, 0.0)));
        let zoom = Rc::new(Cell::new(0f64));
        let inner = Rc::new(RefCell::new(InnerImpl {
            image,
            zoom_buf: None,
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
            _inner.borrow().scale_buf(zoom);
            glib::signal::Inhibit(true)
        });
        Self { inner }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn run() {
        let application = Application::new(None, Default::default()).expect("Fail to init");
        application.connect_activate(|app| {
            let win = ApplicationWindow::new(app);
            let view = ImageViewer::new(&win);
            view.load_file("/home/alex/Sandbox/rust/sprios/src/viewer/image.jpeg");
            win.add(&view.root_widget());
            win.show_all();
        });
        application.run(&[]);
    }
}

