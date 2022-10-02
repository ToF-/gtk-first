use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, DrawingArea};
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
            .default_width(400)
            .default_height(400)
            .title("Hello, World!")
            .build();
        let row = gtk::Box::new(gtk::Orientation::Vertical, 20);
        let frame = gtk::Frame::new(None);
        frame.set_size_request(300, 300);
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
        row.add(&frame);
        let button = Button::with_label("Click me!");
        button.connect_clicked(move |_| {
            println!("Hello");
        });
        row.add(&button);
        win.add(&row);
        win.show_all();
    });

    app.run();
}
