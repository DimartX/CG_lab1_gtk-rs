use crate::color::Color;
use crate::point::Point;

// A generic canvas trait
pub trait Canvas {
    fn set_draw_color(&mut self, color: Color);
    fn set_line_width(&mut self, width: f64);
    //fn text_size(&self, text: &str) -> Size;
    fn print_text(&mut self, p: &Point, text: &str);
    fn draw_line(&mut self, points: &[Point]);
    fn draw_polygon(&mut self, points: &[Point]);
    fn width(&self) -> i32;
    fn height(&self) -> i32;
}

pub struct CairoCanvas<'a> {
    cr: &'a cairo::Context,
    size: (i32, i32), // width and height (REDO on refactoring)
}

impl<'a> CairoCanvas<'a> {
    pub fn new(cr: &'a cairo::Context, size: (i32, i32)) -> Self {
        cr.scale(size.0 as f64, size.1 as f64);
        cr.set_source_rgb(1.0, 1.0, 1.0);
        cr.paint();
        Self { cr, size }
    }
}

// Wrapper for all line or shape drawing methods
// TODO: Add convert method
impl<'a> CairoCanvas<'a> {
    pub fn make_path(&mut self, points: &[Point]) {
        let (first, rest) = points
            .split_first()
            .expect("At least two points to make a line");
        self.cr.move_to(first.x() as f64 / self.size.0 as f64,
                        first.y() as f64 / self.size.1 as f64);
        for p in rest {
            self.cr.line_to(p.x() as f64 / self.size.0 as f64,
                            p.y() as f64 / self.size.1 as f64);
        }
    }
}

impl<'a> Canvas for CairoCanvas<'a> {
    fn set_draw_color(&mut self, color: Color) {
        self.cr.set_source_rgb(color.r() as f64 / 255.0,
                               color.g() as f64 / 255.0,
                               color.b() as f64 / 255.0)
    }

    fn set_line_width(&mut self, width: f64) {
        self.cr.set_line_width(width);
    }

    fn print_text(&mut self, p: &Point, text: &str) {
        self.cr.move_to(p.x() as f64 / self.size.0 as f64,
                        p.y() as f64 / self.size.1 as f64);
        self.cr.show_text(text);
    }

    fn draw_line(&mut self, points: &[Point]) {
        if points.len() > 1 {
            self.make_path(points);
            self.cr.stroke();
        }
    }

    fn draw_polygon(&mut self, points: &[Point]) {
        if points.len() > 1 {
            self.make_path(points);
            self.cr.close_path();
            self.cr.stroke();
        }
    }

    fn width(&self) -> i32 {
        self.size.0
    }

    fn height(&self) -> i32 {
        self.size.1
    }
}
