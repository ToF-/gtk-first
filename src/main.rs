use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, DrawingArea, Box};
use gtk::cairo::{Context, Error};
use gtk::Orientation::Vertical;

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

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let area = DrawingArea::new();
    area.set_draw_func(move |_w, c, _x, _y| {
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
    });


    area.set_size_request(300, 300);

    let button = Button::builder()
        .label("Click me!")
        .build();

    button.connect_clicked(move |_| {
        println!("Hello");
    });

    let row = Box::builder()
        .orientation(Vertical)
        .build();
    row.append(&area);
    row.append(&button);

    let window = ApplicationWindow::builder()
        .application(app)
        .default_width(400)
        .default_height(400)
        .title("Hello, World!")
        .child(&row)
        .build();

    // Present window
    window.present();
}
