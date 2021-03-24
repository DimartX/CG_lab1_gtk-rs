use crate::point::{self, Point, Point_f64};
use crate::color::Color;

use std::f64::consts::PI;
use crate::canvas::{CairoCanvas, Canvas};

struct Axis {
    xoy: Point,
    angle: f64,
    zoom: u32,
    real_zoom: f64,
    center: Point,
}

impl<'a> Axis {
    pub fn new(canvas: &'a CairoCanvas) -> Self {
        Axis {
            xoy: Point::new(
                canvas.width() / 2,
                canvas.height() / 2,
            ),
            angle: 0.0,
            zoom: 100,
            real_zoom: 1.0,
            center: Point::new(
                canvas.width() / 2,
                canvas.height() / 2,
            ),
        }
    }

    pub fn rotate(&mut self, angle: f64) {
        self.angle = to_radians(angle);
    }

    pub fn zoom(&mut self, zoom: u32) {
        self.zoom = zoom;
        self.real_zoom = 2_f64.powi(100 - self.zoom as i32);
    }

    pub fn shift(&mut self, offset: Point) {
        self.xoy = self.center + offset;
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

        self.draw_strokes(canvas);
        // Find intersection with borders
        // let  = point::intersect_rect_line(&self.corner, (&a, &c));
        // let [b, d] = point::intersect_rect_line(&self.corner, (&b, &d));

        canvas.draw_line(&[a, c]);
        canvas.draw_line(&[b, d]);
    }

    fn draw_strokes(&self, canvas: &mut CairoCanvas) {
        const MAG: u32 = 100;

        let step = MAG;// - (MAG / 2) * self.zoom % MAG;
        let astep = Point_f64::new(
            step as f64 * self.angle.cos(),
            step as f64 * self.angle.sin()
        );
        let bstep = Point_f64::new(
            step as f64 * (PI * 0.5 + self.angle).cos(),
            step as f64 * (PI * 0.5 + self.angle).sin(),
        );

        const STROKE_LEN: f64 = 4.0;
        let aperp = Point_f64::new(
            STROKE_LEN * (PI * 0.5 + self.angle).cos(),
            STROKE_LEN * (PI * 0.5 + self.angle).sin(),
        );

        let bperp = Point_f64::new(
            STROKE_LEN * (PI * 1.0 + self.angle).cos(),
            STROKE_LEN * (PI * 1.0 + self.angle).sin(),
        );

        let pt: Point_f64 = Point_f64::from(self.xoy.clone());

        let lu = Point_f64::new(0.0, 0.0);
        let rd = Point_f64::new(canvas.width() as f64, canvas.height() as f64);

        // GoGo paint it out!
        let apt = self.draw_axis(canvas, pt, astep, aperp, lu, rd, 1.0);
        let apt = self.point_arrow(apt - astep, apt, lu, rd);
        canvas.draw_line(&[
            Point::from(apt + bperp * 2.0 + aperp),
            Point::from(apt),
        ]);
        canvas.draw_line(&[
            Point::from(apt + bperp * 2.0 - aperp),
            Point::from(apt),
        ]);

        self.draw_axis(canvas, pt, -astep, aperp, lu, rd, -1.0);

        self.draw_axis(canvas, pt, bstep, bperp, lu, rd, -1.0);

        let bpt = self.draw_axis(canvas, pt, -bstep, bperp, lu, rd, 1.0);
        let bpt = self.point_arrow(bpt + bstep, bpt, lu, rd);
        canvas.draw_line(&[
            Point::from(bpt + aperp * 2.0 + bperp),
            Point::from(bpt),
        ]);
        canvas.draw_line(&[
            Point::from(bpt + aperp * 2.0 - bperp),
            Point::from(bpt),
        ]);
    }

    fn draw_axis(&self, canvas: &mut CairoCanvas,
                 pt: Point_f64, step: Point_f64, perp: Point_f64,
                 lu: Point_f64, rd: Point_f64, sign: f64
    ) -> Point_f64 {
        let mut pt = pt;
        let num = 100.0 / 2_f64.powi(100 - self.zoom as i32);
        for i in 1..40 {
            pt = pt + step;
            if !point::is_in_rectangle(lu, rd, pt) {
                break;
            }
            canvas.draw_line(&[ Point::from(pt + perp), Point::from(pt - perp) ]);
            canvas.print_text(&Point::from(pt + perp * 4.0),
                              &(format!("{:.1}", sign * (i as f64 * num))));
        }
        pt
    }

    fn point_arrow(&self, pre_pt: Point_f64, pt: Point_f64,
                  lu: Point_f64, rd: Point_f64) -> Point_f64 {
        let mut l = pre_pt;
        let mut r = pt;

        const EPS: f64 = 0.001;
        while (r - l).x().abs() > EPS || (r - l).y().abs() > EPS {
            let mid = (l + r) * 0.5;

            if !point::is_in_rectangle(lu, rd, mid) {
                r = mid;
            } else {
                l = mid;
            }
        }
        r
    }
}


pub struct Figure {
    points: Vec<Point>, // точки, по которым строим окружность
    axis: Axis, // абсциссы, к которым привязана фигура
    offset: Point, // сдвиг к центру
    radius: f64, // радиус "круга"
    angle: f64, // угол, на который повёрнута фигура
    parts: u32, // количество точек, по которым строится фигура
    scale_ox: f64,
    scale_oy: f64,
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
            radius: 100f64,
            angle: 0f64,
            parts: 0u32,
            scale_ox: 100f64,
            scale_oy: 100f64,
        }
    }

    // Calculate figure points
    fn update(&mut self) {
        self.points.clear();
        let mut phi: f64 = self.angle;

        for _ in 0..self.parts as usize {
            phi += 2.0 * PI / (self.parts as f64);
            //
            let x: f64 = (self.radius * self.axis.real_zoom) * phi.cos() + self.offset.x() as f64;
            let y: f64 = (self.radius * self.axis.real_zoom) * phi.sin() + self.offset.y() as f64;

            let nx: i32 = (
                (x * self.axis.angle.cos() -
                 y * self.axis.angle.sin()) * self.scale_ox / 100.0) as i32;
            let ny: i32 = (
                (x * self.axis.angle.sin() +
                 y * self.axis.angle.cos()) * self.scale_oy / 100.0) as i32;

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

    pub fn shift_axis(&mut self, offset: Point) {
        self.axis.shift(offset);
        self.update();
    }

    pub fn zoom(&mut self, zoom: u32) {
        self.axis.zoom(zoom);
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

    pub fn scaleOx(&mut self, scale: f64) {
        self.scale_ox = scale;
    }

    pub fn scaleOy(&mut self, scale: f64) {
        self.scale_oy = scale;
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
