#[derive(Debug)]
pub struct Point {
    pub abscissa: f64,
    pub ordinate: f64,
}

#[derive(Debug)]
pub struct Line {
    pub start: Point,
    pub finish: Point,
}

#[derive(Debug, Copy, Clone)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

pub fn create_lines(size: Size) -> Vec<Line> {
    let a_line = Line {
        start: Point {
            abscissa: 0f64,
            ordinate: 0f64,
        },
        finish: Point {
            abscissa: size.width,
            ordinate: size.height,
        },
    };
    let a_line2 = Line {
        start: Point {
            abscissa: size.width,
            ordinate: 0f64,
        },
        finish: Point {
            abscissa: 0f64,
            ordinate: size.height,
        },
    };
    vec![a_line, a_line2]
}
