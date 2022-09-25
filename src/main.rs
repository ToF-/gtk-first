use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, DrawingArea};

fn main() {
    let app = Application::builder()
        .application_id("org.example.HelloWorld")
        .build();

    app.connect_activate(|app| {
        // We create the main window.
        let win = ApplicationWindow::builder()
            .application(app)
            .default_width(320)
            .default_height(200)
            .title("Hello, World!")
            .build();
        let frame = gtk::Frame::new(None);
        let area = DrawingArea::new();
        area.connect_draw(move|w, c|{
            c.rectangle(1.0, 1.0, 100.0, 200.0);
            c.fill();
            gtk::Inhibit(false)
        });
        frame.add(&area);
        win.add(&frame);
        win.show_all();
    });

    app.run();
}
