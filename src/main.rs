use gtk::prelude::*;
use gtk::Application;
use gio::prelude::*;

use std::collections::HashMap;
use std::env::args;
use std::rc::Rc;
use std::cell::RefCell;
use std;

mod canvas;
mod color;
mod figure;
mod point;
mod state;
mod buttons_events;
mod handle_draw;
use crate::figure::Figure;
use crate::canvas::{CairoCanvas, Canvas};
use crate::state::State;

fn build_ui(app: &gtk::Application) {
    // Initialize the UI with Glade XML.
    let glade_src = include_str!("gui_lab1.glade");
    let builder = gtk::Builder::from_string(glade_src);

    // Get handles for the various controls we need to use.
    let window: gtk::Window = builder.get_object("mainWindow")
        .expect("Couldn't get mainWindow");


    // Get handles for all the buttons.
    let mut buttons: HashMap<String, gtk::SpinButton> = HashMap::new();
    for name in &["cntPoints", "moveFigureOx", "moveFigureOy", "rotateFigure",
                  "scale", "scaleOx", "scaleOy",
                  "zoom", "moveAxisOx", "moveAxisOy", "rotateAxes"] {
        buttons.insert(name.to_string(), builder.get_object(name)
                       .expect(&format!("Couldn't get button {}", name)));
    }

    let state = Rc::new(RefCell::new(State::new(&buttons)));

    let drawing_area: gtk::DrawingArea = builder.get_object("drawingArea")
        .expect("Couldn't get drawingArea");
    let drawing = Rc::new(RefCell::new(drawing_area));

    setup_canvas_area(&builder, &state, &drawing);
    crate::buttons_events::setup_buttons_events(&buttons, &state, &drawing);

    window.set_application(Some(app));
    window.show_all();
}

fn setup_canvas_area(
    builder: &gtk::Builder,
    state: &Rc<RefCell<State>>,
    drawing_area: &Rc<RefCell<gtk::DrawingArea>>,
) {
    let draw_box: gtk::Box = builder.get_object("box").expect("Can't get boxx");
    let draw_state = Rc::clone(&state);

    drawing_area.borrow().connect_draw(move |_, cr| {
        let size: (i32, i32) = (draw_box.get_allocated_width(), draw_box.get_allocated_height());
        let mut canvas = CairoCanvas::new(&cr, size);
        canvas.set_line_width(0.001);

        let cur_draw_state = draw_state.borrow();

        let mut circle: Figure = Figure::new(&canvas);

        crate::handle_draw::handle_draw(&mut canvas, &mut circle, &cur_draw_state);

        Inhibit(false)
    });
}

fn main() {
    // Initializing GTK application
    let application = Application::new(
        Some("src.main"),
        gio::ApplicationFlags::NON_UNIQUE,
    ).expect("failed to initialize GTK application");

    // The activation signal is emitted on the activation occurs
    application.connect_activate(|app| build_ui(app));

    // Run the application
    application.run(&args().collect::<Vec<_>>());
}
