pub mod lines;

use std::thread;
use glib::Sender;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Button, DrawingArea};
use gtk::cairo::{Context, Error};
use gtk::glib::{MainContext, PRIORITY_DEFAULT};
use gtk::Orientation::Vertical;
use crate::lines::{create_lines, Line, Size};

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
    let (sender, receiver) = MainContext::channel::<Vec<Line>>(PRIORITY_DEFAULT);

    let area = DrawingArea::new();
    let size = Size {
        height: 300f64,
        width: 300f64,
    };
    area.set_size_request(size.width as i32, size.height as i32);

    let button = Button::builder()
        .label("Click me!")
        .build();

    button.connect_clicked(move |_| {
        let sender = sender.clone();
        thread::spawn(move || {
            produce_line(sender, size);
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
            draw(line, &area);
            Continue(true)
        },
    );

    window.present();
}

fn produce_line(sender: Sender<Vec<Line>>, size: Size) {
    sender.send(create_lines(size)).expect("Could not send through channel");
}


fn draw(lines: Vec<Line>, area: &DrawingArea) {
    area.unset_draw_func();
    area.set_draw_func(move |_w, c, _x, _y| {
        c.set_source_rgb(0.0, 0.0, 0.0);
        lines.iter().for_each(|line| {
            c.draw_line(line).expect("oops");
        });
    });
}
