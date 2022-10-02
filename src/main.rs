use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, DrawingArea};
use gtk::cairo::{Context, Error};

struct Point {
    abscissa: f64,
    ordinate: f64,
}

struct Line {
    start: Point,
    finish: Point,
}

trait LineDrawer {
    fn draw_line(&self, line: &Line) -> Result<(), Error>;
}

impl LineDrawer for Context {
    fn draw_line(&self, line: &Line) -> Result<(), Error> {
        self.move_to(line.start.abscissa, line.start.ordinate);
        self.line_to(line.finish.abscissa, line.finish.ordinate);
        self.stroke()
    }
}

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
        area.connect_draw(move |_w, c| {
            c.set_source_rgb(0.0, 0.0, 0.0);
            c.set_line_width(10.0);

            let a_line = Line {
                start: Point {
                    abscissa: 0f64,
                    ordinate: 0f64,
                },
                finish: Point {
                    abscissa: 200f64,
                    ordinate: 200f64,
                },
            };

            c.draw_line(&a_line).unwrap();
            gtk::Inhibit(false)
        });
        frame.add(&area);
        win.add(&frame);
        win.show_all();
    });

    app.run();
}
