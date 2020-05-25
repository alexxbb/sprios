
#[cfg(not(feature="command"))]
mod app;

#[cfg(not(feature="command"))]
fn gui() {
    use app::App as SpriosApp;
    use gio::prelude::*;
    let args: Vec<String> = std::env::args().collect();
    let app = gtk::Application::new(Some("sprios.dev"), Default::default()).expect("Fail to init");
    app.connect_startup(|app| SpriosApp::on_startup(app));
    app.run(&args);
}

#[cfg(feature="command")]
fn cmd() {
    use renderer::{render};
    use std::io::BufWriter;
    use std::io::Write;
    use std::sync::atomic::AtomicPtr;
    use std::sync::Arc;


    let args:Vec<String> = std::env::args().collect();
    let mut opts = getopts::Options::new();
    let aspect_ratio: f32 = 16.0 / 9.0;
    opts.optopt("w", "width", "Image width", "WIDTH");
    opts.optopt("s", "samples", "Pixel samples", "SAMPLES");
    opts.optopt("t", "threads", "Number of threads", "THREADS");
    opts.optopt("b", "bucket", "Bucket size", "BUCKET");
    opts.optflag("h", "help", "print help");

    let args = match opts.parse(args) {
        Ok(m) => m,
        Err(e) => {panic!(e.to_string())}
    };

    if args.opt_present("h") {
        println!("{}", opts.short_usage("SPRIOS"));
        return
    }

    let image_width: u32 = match args.opt_str("w") {
        Some(s) => {s.parse().unwrap()},
        None => 720
    };
    let image_height: u32 = (image_width as f32 / aspect_ratio) as u32;
    let samples_per_pixel: u32 = match args.opt_str("s") {
        Some(s) => {s.parse().unwrap()},
        None=> 10
    };
    let bucket: u32 = match args.opt_str("b") {
        Some(s) => {s.parse().unwrap()},
        None=> 32
    };
    let num_threads: usize = match args.opt_str("t") {
        Some(s) => {s.parse().unwrap()},
        None=> num_cpus::get_physical()
    };

    let pool = threadpool::ThreadPool::new(num_threads);

    let mut img_buf:Vec<u8> = Vec::new();
    img_buf.resize((image_width * image_height * 3) as usize, 0);
    let img_ptr = Arc::new(AtomicPtr::new(img_buf.as_mut_ptr()));

    let stat = render(image_width, image_height, samples_per_pixel, bucket, img_ptr, Some(&pool), |prog| {
        eprint!("\rRendering: {}%", prog);
        std::io::stderr().flush().unwrap();
    });

    eprintln!("\nSaving image.ppm");
    use std::fs::File;
    let f = File::create("image.ppm").expect("Could not create ppm");
    let mut buf = BufWriter::with_capacity(img_buf.len(), &f);

    writeln!(buf, "P3\n{} {}\n255", image_width, image_height).unwrap();

    for mut i in 0..img_buf.len() / 3 {
        i *= 3;
        buf.write_fmt(format_args!(
            "{} {} {}\n",
            img_buf[i + 0],
            img_buf[i + 1],
            img_buf[i + 2]
        )).unwrap();
    }
    buf.flush().unwrap();
    eprintln!("{:?}", stat);
}

fn main() {
    #[cfg(feature="command")]
    cmd();
    #[cfg(not(feature="command"))]
    gui();
}
