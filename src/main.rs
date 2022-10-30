pub mod lines;

use std::thread;
use glib::Sender;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Button, DrawingArea};
use gtk::cairo::{Context, Error};
use gtk::glib::{MainContext, PRIORITY_DEFAULT};
use gtk::Orientation::Vertical;
use crate::lines::{create_lines, JunctionFactor, Line, NumberOfPoints};

#[derive(Debug, Copy, Clone)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

trait LineDrawer {
    fn draw_line(&self, line: &Line, size: Size) -> Result<(), Error>;
}

fn scale(n: f64, factor: f64) -> f64 {
    n * factor + factor
}

impl LineDrawer for Context {
    fn draw_line(&self, line: &Line, size: Size) -> Result<(), Error> {
        self.move_to(scale(line.start.abscissa, size.width), scale(line.start.ordinate, size.height));
        self.line_to(scale(line.finish.abscissa, size.width), scale(line.finish.ordinate, size.height));
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
    let (sender, receiver) = MainContext::channel::<Vec<Line>>(PRIORITY_DEFAULT);

    let area = DrawingArea::new();
    let size = Size {
        height: 150f64,
        width: 150f64,
    };
    area.set_size_request(size.width as i32 * 2, size.height as i32 * 2);

    let button = Button::builder()
        .label("Click me!")
        .build();

    button.connect_clicked(move |_| {
        let sender = sender.clone();
        thread::spawn(move || {
            produce_line(sender);
        });
    });

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

    receiver.attach(
        None,
        move |line| {
            draw(line, &area, size);
            Continue(true)
        },
    );

    window.present();
}

fn produce_line(sender: Sender<Vec<Line>>) {
    sender.send(create_lines(NumberOfPoints(72), JunctionFactor(2f64))).expect("Could not send through channel");
}


fn draw(lines: Vec<Line>, area: &DrawingArea, size: Size) {
    area.unset_draw_func();
    area.set_draw_func(move |_w, c, _x, _y| {
        c.set_source_rgb(0.0, 0.0, 0.0);
        lines.iter().for_each(|line| {
            c.draw_line(line, size).expect("oops");
        });
    });
}
