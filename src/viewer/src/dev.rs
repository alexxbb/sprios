use gtk::*;
use gio::prelude::*;
use viewer::ImageViewer;

fn main (){
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