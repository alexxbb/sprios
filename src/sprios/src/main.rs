mod app;

use app::App as SpriosApp;

use gio::prelude::*;

#[allow(non_upper_case_globals)]
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let app = gtk::Application::new(Some("sprios.dev"), Default::default()).expect("Fail to init");
    app.connect_startup(|app| SpriosApp::on_startup(app));
    app.run(&args);
}
