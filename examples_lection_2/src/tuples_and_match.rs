#[derive(Clone, Copy)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Clone, Copy)]
pub struct Point(i32, i32);

pub fn moved_point(mut point: Point, direction: Direction) -> Point {
    match direction {
        Direction::Left => point.0 -= 1,
        Direction::Right => point.0 += 1,
        Direction::Up => point.1 += 1,
        Direction::Down => point.1 -= 1,
    }
    point
}

#[test]
fn test_moved_point() {
    let initial = Point(0, 0);

    let moved = moved_point(initial, Direction::Down);//
    let expected = Point(0, -1);

    assert_eq!(moved.0, expected.0);
    assert_eq!(moved.1, expected.1);

    let moved = moved_point(initial, Direction::Down);
    let expected = Point(0, -1);

    assert_eq!(moved.0, expected.0);
    assert_eq!(moved.1, expected.1);
}

pub enum Shape {
    Rectangle { width: f64, height: f64 },
    Circle(f64),
    Point,
}

impl Shape {
    pub fn area(&self) -> f64 {
        match self {
            Shape::Rectangle { width, height } => width * height,
            Shape::Circle(rad) => {
                use std::f64::consts::PI;
                rad * rad * PI
            }
            Shape::Point => 0.0,
        }
    }
}

const EPS: f64 = f64::EPSILON;

#[test]
fn test_shape_area() {
    let rect = Shape::Rectangle { width: 10.0, height: 2.0 };
    assert!((rect.area() - 20.0).abs() < EPS);

    let circle = Shape::Circle(2.0);
    let expected = 12.566370614359172;
    assert!((circle.area() - expected).abs() < EPS);

    let point = Shape::Point;
    assert!((point.area() - 0.0).abs() < EPS);
}