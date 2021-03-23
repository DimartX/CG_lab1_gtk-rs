use crate::point::Point;
use crate::color::Color;

use std::f64::consts::PI;
use crate::canvas::{CairoCanvas, Canvas};

struct Axis {
    xoy: Point,
    angle: f64,
}

impl<'a> Axis {
    pub fn new(canvas: &'a CairoCanvas) -> Self {
        Axis {
            xoy: Point::new(
                canvas.width() / 2,
                canvas.height() / 2,
            ),
            angle: 0.0,
        }
    }

    pub fn rotate(&mut self, angle: f64) {
        self.angle += to_radians(angle);
    }

    pub fn draw(&self, canvas: &mut CairoCanvas) {
        let diagonal = ((canvas.width() * canvas.width()
            + canvas.height() * canvas.height()) as f64)
            .sqrt()
            * 2.0;
        let a = Point::new(
            self.xoy.x() + (diagonal * self.angle.cos()) as i32,
            self.xoy.y() + (diagonal * self.angle.sin()) as i32,
        );
        let b = Point::new(
            self.xoy.x() + (diagonal * (self.angle + PI * 0.5).cos()) as i32,
            self.xoy.y() + (diagonal * (self.angle + PI * 0.5).sin()) as i32,
        );
        let c = Point::new(
            self.xoy.x() + (diagonal * (self.angle + PI).cos()) as i32,
            self.xoy.y() + (diagonal * (self.angle + PI).sin()) as i32,
        );
        let d = Point::new(
            self.xoy.x() + (diagonal * (self.angle + PI * 1.5).cos()) as i32,
            self.xoy.y() + (diagonal * (self.angle + PI * 1.5).sin()) as i32,
        );

        canvas.draw_line(&[a, c]);
        canvas.draw_line(&[b, d]);
    }
}


pub struct Figure {
    points: Vec<Point>, // точки, по которым строим окружность
    axis: Axis, // абсциссы, к которым привязана фигура
    offset: Point, // сдвиг к центру
    radius: f64,
    angle: f64,
    parts: u32, // количество точек, по которым строится фигура
}

fn to_radians(angle: f64) -> f64 {
    angle / 180.0 * PI
}

impl Figure {
    pub fn new(canvas: &CairoCanvas) -> Self {
        Figure {
            points: Vec::new(),
            axis: Axis::new(&canvas),
            offset: Point::new(0, 0),
            radius: 0f64,
            angle: 0f64,
            parts: 0u32,
        }
    }

    fn update(&mut self) {
        self.points.clear();
        let mut phi: f64 = self.angle;

        for _ in 0..self.parts as usize {
            phi += 2.0 * PI / (self.parts as f64);
            let x: f64 = (self.radius as f64) * phi.cos() + self.offset.x() as f64;
            let y: f64 = (self.radius as f64) * phi.sin() + self.offset.y() as f64;

            let nx: i32 = (x * self.axis.angle.cos() - y * self.axis.angle.sin()) as i32;
            let ny: i32 = (x * self.axis.angle.sin() + y * self.axis.angle.cos()) as i32;

            self.points.push(Point::new(nx, ny) + self.axis.xoy);
        }
    }

    pub fn parts(&mut self, parts: u32) {
        self.parts = parts;
        self.update();
    }

    pub fn radius(&mut self, radius: f64) {
        self.radius = radius;
        self.update();
    }

    pub fn rotate_axis(&mut self, angle: f64) {
        self.axis.rotate(angle);
        self.update();
    }

    pub fn rotate(&mut self, angle: f64) {
        self.angle += to_radians(angle);
        self.update();
    }

    pub fn shift(&mut self, offset: Point) {
        self.offset = self.offset + offset;
        self.update();
    }

    pub fn draw(&self, canvas: &mut CairoCanvas) {
        canvas.set_draw_color(Color::new(0, 0, 0));

        for i in 0..self.parts as usize {
            canvas.draw_line(
                &[self.points[i % (self.parts as usize)],
                self.points[(i + 1) % (self.parts as usize)]]
            );
        }

        self.axis.draw(canvas);
    }
}
