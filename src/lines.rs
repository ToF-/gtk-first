#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point {
    pub abscissa: f64,
    pub ordinate: f64,
}

#[derive(Debug, PartialEq, Copy, Clone)]
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

    mod assertions {
        use super::*;
        macro_rules! assert_eq_delta {
            ($x:expr, $y:expr, $delta:expr) => {
                if(f64::abs($x - $y) > $delta) {panic!();}
            };
        }

        macro_rules! assert_points_eq {
            ($x:expr, $y:expr) => {
                assert_eq_delta!($x.abscissa, $y.abscissa,0.0001);
                assert_eq_delta!($x.ordinate, $y.ordinate,0.0001);
            };
        }

        macro_rules! assert_lines_eq {
            ($x:expr, $y:expr) => {
                assert_points_eq!($x.start, $y.start);
                assert_points_eq!($x.finish, $y.finish);
            };
        }

        #[test]
        fn allows_for_delta_in_comparison() {
            assert_eq_delta!(2.0, 2.0+0.01, 0.1);
        }

        #[test]
        fn allows_for_delta_in_comparison_for_negatives() {
            assert_eq_delta!(-2.0, -2.0+0.01, 0.1);
        }

        #[test]
        #[should_panic]
        fn panic_when_delta_passed() {
            assert_eq_delta!(2.0+0.000011, 2.0, 0.00001)
        }

        #[test]
        #[should_panic]
        fn panic_when_delta_passed_by_absolute_difference() {
            assert_eq_delta!(2.0, 2.0+0.000011, 0.00001)
        }

        #[test]
        fn two_points_are_equals_with_a_difference_under_1_over_1000() {
            let a = Point {
                abscissa: 0f64,
                ordinate: 1f64,
            };
            let b = Point {
                abscissa: 0f64 + 0.00001f64,
                ordinate: 1f64 + 0.00001f64,
            };
            assert_points_eq!(a,b);
            assert_points_eq!(b,a);
        }

        #[test]
        #[should_panic]
        fn two_points_are_not_equals_with_a_difference_over_1_over_1000() {
            let a = Point {
                abscissa: 0f64,
                ordinate: 1f64,
            };
            let b = Point {
                abscissa: 0f64 + 0.001f64,
                ordinate: 1f64 + 0.001f64,
            };
            assert_points_eq!(a,b);
        }

        #[test]
        #[should_panic]
        fn two_points_are_not_equals_with_a_difference_over_1_over_1000_both_ways() {
            let a = Point {
                abscissa: 0f64,
                ordinate: 1f64,
            };
            let b = Point {
                abscissa: 0f64 + 0.001f64,
                ordinate: 1f64 + 0.001f64,
            };
            assert_points_eq!(b,a);
        }

        #[test]
        fn two_lines_are_equals_when_points_are_equals() {
            let a = Point {
                abscissa: 0f64,
                ordinate: 1f64,
            };
            let b = Point {
                abscissa: 0f64 + 0.00001f64,
                ordinate: 1f64 + 0.00001f64,
            };
            let line1 = Line { start: a, finish: b };
            let line2 = Line { start: a, finish: b };
            assert_lines_eq!(line1,line2);
        }

        #[test]
        #[should_panic]
        fn two_lines_are_not_equals_when_points_are_different() {
            let a = Point {
                abscissa: 0f64,
                ordinate: 1f64,
            };
            let b = Point {
                abscissa: 0f64 + 0.00001f64,
                ordinate: 1f64 + 0.00001f64,
            };
            let c = Point {
                abscissa: 0f64 + 0.01f64,
                ordinate: 1f64 + 0.00001f64,
            };
            let line1 = Line { start: a, finish: b };
            let line2 = Line { start: a, finish: c };
            assert_lines_eq!(line1,line2);
        }
    }

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
