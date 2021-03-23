use std::collections::HashMap;
use gtk::prelude::*;

// Shared state for communication between buttons and drawingarea
pub struct State {
    pub cntPoints: i32,
    pub moveFigureOx: i32,
    pub moveFigureOy: i32,
    pub rotateFigure: i32,
    pub scale: i32,
    pub scaleOx: i32,
    pub scaleOy: i32,
    pub zoom: i32,
    pub moveAxisOx: i32,
    pub moveAxisOy: i32,
    pub rotateAxes: i32,
}

impl State {
    pub fn new(buttons: &HashMap<String, gtk::SpinButton>) -> Self {
        State {
            cntPoints:    buttons.get("cntPoints").unwrap().get_value_as_int(),
            moveFigureOx: buttons.get("moveFigureOx").unwrap().get_value_as_int(),
            moveFigureOy: buttons.get("moveFigureOy").unwrap().get_value_as_int(),
            rotateFigure: buttons.get("rotateFigure").unwrap().get_value_as_int(),
            scale:        buttons.get("scale").unwrap().get_value_as_int(),
            scaleOx:      buttons.get("scaleOx").unwrap().get_value_as_int(),
            scaleOy:      buttons.get("scaleOy").unwrap().get_value_as_int(),
            zoom:         buttons.get("zoom").unwrap().get_value_as_int(),
            moveAxisOx:   buttons.get("moveAxisOx").unwrap().get_value_as_int(),
            moveAxisOy:   buttons.get("moveAxisOy").unwrap().get_value_as_int(),
            rotateAxes:   buttons.get("rotateAxes").unwrap().get_value_as_int(),
        }
    }
}
