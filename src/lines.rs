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

pub fn create_lines() -> Vec<Line> {
    let a_line = Line {
        start: Point {
            abscissa: 0f64,
            ordinate: 0f64,
        },
        finish: Point {
            abscissa: 1f64,
            ordinate: 1f64,
        },
    };
    let a_line2 = Line {
        start: Point {
            abscissa: 1f64,
            ordinate: 0f64,
        },
        finish: Point {
            abscissa: 0f64,
            ordinate: 1f64,
        },
    };
    vec![a_line, a_line2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn displays_2_lines_for_2_points() {
        // create_lines()
    }
}
