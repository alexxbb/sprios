mod app;

use app::{App as SpriosApp};

use gtk::prelude::*;
use gio::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;


#[allow(non_upper_case_globals)]
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let app = gtk::Application::new(Some("sprios.dev"), Default::default()).expect("Fail to init");
    app.connect_startup(move |app| unsafe {
        SpriosApp::on_startup(app)
    });
    app.run(&args);
}
