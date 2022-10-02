use std::thread;
use glib::Sender;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, DrawingArea, Box};
use gtk::cairo::{Context, Error};
use gtk::glib::{clone, MainContext, PRIORITY_DEFAULT};
use gtk::Orientation::Vertical;

#[derive(Debug)]
struct Point {
    abscissa: f64,
    ordinate: f64,
}

#[derive(Debug)]
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
    let (sender, receiver) = MainContext::channel::<Line>(PRIORITY_DEFAULT);

    let area = DrawingArea::new();
    let a_line = Line {
        start: Point {
            abscissa: 0f64,
            ordinate: 0f64,
        },
        finish: Point {
            abscissa: 100f64,
            ordinate: 100f64,
        },
    };

    draw(a_line, &area);

    area.set_size_request(300, 300);
    let size = area.size_request();

    let button = Button::builder()
        .label("Click me!")
        .build();

    button.connect_clicked(move |_| {
        let sender = sender.clone();
        thread::spawn(move || {
            produce_line(sender, size);
        });
    });

    receiver.attach(
        None,
        clone!(@weak area => @default-return Continue(false),
                    move |line| {
                    draw(line, &area);
                    Continue(true)
                    }
        ),
    );

    let row = Box::builder()
        .orientation(Vertical)
        .spacing(30)
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

fn produce_line(sender: Sender<Line>, size: (i32, i32)) {
    let a_line = Line {
        start: Point {
            abscissa: 0f64,
            ordinate: 0f64,
        },
        finish: Point {
            abscissa: size.0 as f64,
            ordinate: size.1 as f64,
        },
    };
    // Deactivate the button until the operation is done
    sender.send(a_line).expect("Could not send through channel");
}


fn draw(line: Line, area: &DrawingArea) {
    area.unset_draw_func();
    area.set_draw_func(move |_w, c, _x, _y| {
        c.set_source_rgb(0.0, 0.0, 0.0);
        c.draw_line(&line).expect("oops");
    });
}
