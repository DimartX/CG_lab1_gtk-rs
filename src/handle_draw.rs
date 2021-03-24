use crate::figure::Figure;
use crate::state::State;
use crate::point::Point;
use crate::canvas::CairoCanvas;

use std::cell::Ref;

pub fn handle_draw(canvas: &mut CairoCanvas, circle: &mut Figure, state: &Ref<State>) {
    circle.parts(state.cntPoints as u32);
    circle.radius(state.scale as f64);
    circle.rotate(state.rotateFigure as f64);
    circle.rotate_axis(state.rotateAxes as f64);
    circle.shift(Point::new(state.moveFigureOx, state.moveFigureOy));
    circle.zoom(state.zoom as u32);
    circle.scaleOx(state.scaleOx as f64);
    circle.scaleOy(state.scaleOy as f64);
    circle.shift_axis(Point::new(state.moveAxisOx, state.moveAxisOy));
    circle.draw(canvas);
}
