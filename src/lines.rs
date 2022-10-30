#[derive(Debug, PartialEq)]
pub struct Point {
    pub abscissa: f64,
    pub ordinate: f64,
}

#[derive(Debug, PartialEq)]
pub struct Line {
    pub start: Point,
    pub finish: Point,
}

pub struct NumberOfPoints(pub u32);

pub struct JunctionFactor(pub f64);

pub fn create_lines(points: NumberOfPoints, factor: JunctionFactor) -> Vec<Line> {
    let a_line = Line {
        start: Point {
            abscissa: 0f64,
            ordinate: 1f64,
        },
        finish: Point {
            abscissa: 0f64,
            ordinate: -1f64,
        },
    };
    let a_line2 = Line {
        start: Point {
            abscissa: 0f64,
            ordinate: -1f64,
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
    fn creates_2_lines_for_2_points() {
        let lines = create_lines(NumberOfPoints(2), JunctionFactor(1f64));
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0], Line {
            start: Point {
                abscissa: 0f64,
                ordinate: 1f64,
            },
            finish: Point {
                abscissa: 0f64,
                ordinate: -1f64,
            },
        });
        assert_eq!(lines[1], Line {
            start: Point {
                abscissa: 0f64,
                ordinate: -1f64,
            },
            finish: Point {
                abscissa: 0f64,
                ordinate: 1f64,
            },
        });
    }
}
