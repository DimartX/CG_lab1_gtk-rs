use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }
}

impl ops::Add for Point {
    type Output = Point;

    fn add(self, _rhs: Point) -> Point {
        Point::new (
            self.x() + _rhs.x(),
            self.y() + _rhs.y(),
        )
    }
}
