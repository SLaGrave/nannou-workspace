use nannou::prelude::*;
use nannou::rand::rngs::StdRng;
use nannou::rand::{Rng, SeedableRng};
use nannou_egui::egui::{Button, Checkbox, DragValue, Slider};
use nannou_egui::{self, egui, Egui, FrameCtx};

//////////////////////////////////////////////////
// Global constants
//////////////////////////////////////////////////
const HEIGHT: f32 = 1000.0;
const WIDTH: f32 = 1000.0;
const NUM_TRAVELLERS: usize = 100;

//////////////////////////////////////////////////
// My Structs
//////////////////////////////////////////////////
struct StationaryMass {
    name: String,
    x: f32,
    y: f32,
    mass: f32,
}

impl StationaryMass {
    fn new(name: String, x: f32, y: f32, mass: f32) -> Self {
        StationaryMass { name, x, y, mass }
    }
}

struct Traveller {
    x: f32,
    y: f32,
    x_velocity: f32,
    y_velocity: f32,
}

impl Traveller {
    fn new(x: f32, y: f32, x_velocity: f32, y_velocity: f32) -> Self {
        Traveller {
            x,
            y,
            x_velocity,
            y_velocity,
        }
    }
}

//////////////////////////////////////////////////
// Nannou App Model
//////////////////////////////////////////////////
struct Model {
    main_window: WindowId,
    ui: Egui,
    random_seed: u64,
    speed_scaling: f32,
    debug: bool,
    travellers: Vec<Traveller>,
    stationary_masses: Vec<StationaryMass>,
}

//////////////////////////////////////////////////
// Main function
//////////////////////////////////////////////////
fn main() {
    nannou::app(init)
        .update(update)
        .loop_mode(LoopMode::refresh_sync())
        .run();
}

//////////////////////////////////////////////////
// Init
// Called once, sets up the windows and model
//////////////////////////////////////////////////
fn init(app: &App) -> Model {
    let main_window = app
        .new_window()
        .title(app.exe_name().unwrap())
        .size(WIDTH as u32, HEIGHT as u32)
        .view(view)
        .build()
        .unwrap();

    let ui_window = app
        .new_window()
        .title(app.exe_name().unwrap() + " controls")
        .size(300, 850)
        .view(ui_view)
        .raw_event(ui_raw_event)
        .build()
        .unwrap();
    let ui_window_ref = app.window(ui_window).unwrap();
    let ui = Egui::from_window(&ui_window_ref);

    let random_seed = random_range(0, 1000000);
    let debug = false;

    let mut rng = StdRng::seed_from_u64(random_seed);

    // Setup travellers
    let mut travellers: Vec<Traveller> = Vec::new();
    for _ in 0..NUM_TRAVELLERS {
        travellers.push(Traveller::new(
            rng.gen_range((-WIDTH / 2.0)..(WIDTH / 2.0)),
            rng.gen_range((-HEIGHT / 2.0)..(HEIGHT / 2.0)),
            rng.gen_range(-10.0..10.0),
            rng.gen_range(-10.0..10.0),
        ));
    }

    // Setup stationary_masses
    let names = vec!["nm", "cf", "ok", "ts", "ir"];
    let mut stationary_masses: Vec<StationaryMass> = Vec::new();
    for name in names {
        stationary_masses.push(StationaryMass::new(
            name.to_string(),
            rng.gen_range((-WIDTH / 2.0)..(WIDTH / 2.0)),
            rng.gen_range((-HEIGHT / 2.0)..(HEIGHT / 2.0)),
            rng.gen_range(0.0..50.0),
        ));
    }

    let speed_scaling: f32 = 0.01;

    Model {
        main_window,
        ui,
        debug,
        speed_scaling,
        random_seed,
        travellers,
        stationary_masses,
    }
}

//////////////////////////////////////////////////
// Update
// Called before each frame - update the
// underlying model
//////////////////////////////////////////////////
fn update(app: &App, model: &mut Model, _update: Update) {
    ui_update(model, app);

    // Update the traveller's velocity
    for i in 0..NUM_TRAVELLERS {
        let traveller = model.travellers.get_mut(i).unwrap();
        let mut new_x_velocity: f32 = 0.0;
        let mut new_y_velocity: f32 = 0.0;
        for stationary_mass in &model.stationary_masses {
            if random() {
                new_x_velocity += ((stationary_mass.x - traveller.x) / WIDTH)
                    * stationary_mass.mass
                    * random_range(1, 5) as f32;
                new_y_velocity += ((stationary_mass.y - traveller.y) / HEIGHT)
                    * stationary_mass.mass
                    * random_range(1, 5) as f32;
            }
        }
        new_x_velocity /= model.stationary_masses.len() as f32;
        new_y_velocity /= model.stationary_masses.len() as f32;
        traveller.x_velocity += new_x_velocity * model.speed_scaling;
        traveller.y_velocity += new_y_velocity * model.speed_scaling;
        // Update traveller's position
        traveller.x += traveller.x_velocity * 0.01;
        traveller.y += traveller.y_velocity * 0.01;
    }
}

//////////////////////////////////////////////////
// View
// Called after update - draws the data
// to the window
//////////////////////////////////////////////////
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    // draw.background().color(BLACK);
    draw.rect()
        .x_y(0.0, 0.0)
        .w_h(WIDTH, HEIGHT)
        .color(Rgba::new(0.05, 0.05, 0.05, 0.01));

    // Draw stationary masses
    // If in debug mode
    if model.debug {
        for sm in &model.stationary_masses {
            draw.text(&sm.name.as_str()).x_y(sm.x, sm.y);
            draw.ellipse()
                .no_fill()
                .stroke(WHITESMOKE)
                .stroke_weight(1.0)
                .radius(sm.mass)
                .x_y(sm.x, sm.y);
        }
    }

    // Draw the travellers
    for i in 0..NUM_TRAVELLERS {
        let traveller = model.travellers.get(i).unwrap();
        if i % 2 == 0 {
            draw.ellipse()
                .color(MAROON)
                .radius(5.0)
                .x_y(traveller.x, traveller.y);
        } else {
            if i % 3 == 0 {
                draw.ellipse()
                    .color(MEDIUMBLUE)
                    .radius(5.0)
                    .x_y(traveller.x, traveller.y);
            } else {
                draw.ellipse()
                    .color(MEDIUMTURQUOISE)
                    .radius(5.0)
                    .x_y(traveller.x, traveller.y);
            }
        }
    }

    // Draw everything to the app
    draw.to_frame(app, &frame).unwrap();
}

//////////////////////////////////////////////////
// UI functions
//////////////////////////////////////////////////
fn ui_view(_app: &App, model: &Model, frame: Frame) {
    model.ui.draw_to_frame(&frame).unwrap();
}

fn ui_raw_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.ui.handle_raw_event(event);
}

fn ui_update(model: &mut Model, app: &App) {
    let ctx = model.ui.begin_frame();
    egui::Window::new("Main Controls").show(&ctx, |ui| {
        ui.add(DragValue::new(&mut model.random_seed));
        ui.add(Checkbox::new(&mut model.debug, "Debug"));
        ui.horizontal(|ui| {
            ui.label("Speed scaling");
            ui.add_space(10.0);
            ui.add(Slider::new(&mut model.speed_scaling, 0.001..=1.0));
        });
        if ui.add(Button::new("Capture")).clicked() {
            match app.window(model.main_window) {
                Some(window) => {
                    window.capture_frame(format!("images/{}.png", app.exe_name().unwrap()));
                }
                None => {}
            }
        }
    });

    // egui::Window::new("Traveller").show(&ctx, |ui| {
    //     ui.horizontal(|ui| {
    //         ui.label("X pos");
    //         ui.add_space(10.0);
    //         ui.label(model.traveller.x);
    //     });
    //     ui.horizontal(|ui| {
    //         ui.label("Y pos");
    //         ui.add_space(10.0);
    //         ui.label(model.traveller.y);
    //     });
    //     ui.horizontal(|ui| {
    //         ui.label(model.traveller.x_velocity);
    //         ui.add_space(10.0);
    //         ui.label(model.traveller.y_velocity);
    //     });
    // });

    for i in 0..model.stationary_masses.len() {
        create_stationary_mass_window(&ctx, model.stationary_masses.get_mut(i).unwrap());
    }
}

// Creates the inner UI window for each StationaryMass
fn create_stationary_mass_window(ctx: &FrameCtx, stationary_mass: &mut StationaryMass) {
    egui::Window::new(stationary_mass.name.clone()).show(&ctx, |ui| {
        ui.horizontal(|ui| {
            ui.label("X pos");
            ui.add_space(10.0);
            ui.add(egui::Slider::new(
                &mut stationary_mass.x,
                (-WIDTH / 2.0)..=(WIDTH / 2.0),
            ));
        });
        ui.horizontal(|ui| {
            ui.label("Y pos");
            ui.add_space(10.0);
            ui.add(egui::Slider::new(
                &mut stationary_mass.y,
                (-HEIGHT / 2.0)..=(HEIGHT / 2.0),
            ));
        });
        ui.horizontal(|ui| {
            ui.label("Mass");
            ui.add_space(10.0);
            ui.add(egui::Slider::new(&mut stationary_mass.mass, 0.0..=50.0));
        });
    });
}
