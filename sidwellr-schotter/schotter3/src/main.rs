use nannou::prelude::*;
use nannou::rand::rngs::StdRng;
use nannou::rand::{Rng, SeedableRng};
use nannou_egui::{self, egui, Egui};

const ROWS: u32 = 22;
const COLS: u32 = 12;
const SIZE: u32 = 30;
const MARGIN: u32 = 35;
const WIDTH: u32 = COLS * SIZE + 2 * MARGIN;
const HEIGHT: u32 = ROWS * SIZE + 2 * MARGIN;
const LINE_WIDTH: f32 = 0.06;

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::wait())
        .run();
}

struct Stone {
    x: f32,
    y: f32,
    x_offset: f32,
    y_offset: f32,
    rotation: f32,
}

impl Stone {
    fn new(x: f32, y: f32) -> Self {
        let x_offset = 0.0;
        let y_offset = 0.0;
        let rotation = 0.0;
        Stone {
            x,
            y,
            x_offset,
            y_offset,
            rotation,
        }
    }
}

struct Model {
    main_window: WindowId,
    ui: Egui,
    random_seed: u64,
    disp_adj: f32,
    rot_adj: f32,
    gravel: Vec<Stone>,
}

fn model(app: &App) -> Model {
    let main_window = app
        .new_window()
        .title(app.exe_name().unwrap())
        .size(WIDTH, HEIGHT)
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    let ui_window = app
        .new_window()
        .title(app.exe_name().unwrap() + " controls")
        .size(280, 130)
        .view(ui_view)
        .raw_event(raw_ui_event)
        .key_pressed(key_pressed)
        .build()
        .unwrap();
    let ui_window_ref = app.window(ui_window).unwrap();
    let ui = Egui::from_window(&ui_window_ref);

    let random_seed = random_range(0, 1000000);
    let disp_adj = 1.0;
    let rot_adj = 1.0;

    let mut gravel = Vec::new();
    for y in 0..ROWS {
        for x in 0..COLS {
            let stone = Stone::new(x as f32, y as f32);
            gravel.push(stone);
        }
    }

    Model {
        main_window,
        ui,
        random_seed,
        disp_adj,
        rot_adj,
        gravel,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    update_ui(model);
    let mut rng = StdRng::seed_from_u64(model.random_seed);
    for stone in &mut model.gravel {
        let factor = stone.y / ROWS as f32;
        let disp_factor = factor * model.disp_adj;
        let rot_factor = factor * model.rot_adj;
        stone.x_offset = disp_factor * rng.gen_range(-0.5..0.5);
        stone.y_offset = disp_factor * rng.gen_range(-0.5..0.5);
        stone.rotation = rot_factor * rng.gen_range(-PI / 4.0..PI / 4.0);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let gdraw = draw
        .scale(SIZE as f32)
        .scale_y(-1.0)
        .x_y(COLS as f32 / -2.0 + 0.5, ROWS as f32 / -2.0 + 0.5);

    draw.background().color(SNOW);

    for stone in &model.gravel {
        let cdraw = gdraw.x_y(stone.x, stone.y);
        cdraw
            .rect()
            .no_fill()
            .stroke(BLACK)
            .stroke_weight(LINE_WIDTH)
            .w_h(1.0, 1.0)
            .x_y(stone.x_offset, stone.y_offset)
            .rotate(stone.rotation);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::R => {
            model.random_seed = random_range(0, 1000000);
        }
        Key::S => match app.window(model.main_window) {
            Some(window) => {
                window.capture_frame(format!("images/{}.png", app.exe_name().unwrap()));
            }
            None => {}
        },
        Key::Up => {
            model.disp_adj += 0.1;
        }
        Key::Down => {
            if model.disp_adj > 0.0 {
                model.disp_adj -= 0.1;
            }
        }
        Key::Right => {
            model.rot_adj += 0.1;
        }
        Key::Left => {
            if model.rot_adj > 0.0 {
                model.rot_adj -= 0.1;
            }
        }
        _other_key => {}
    }
}

fn ui_view(_app: &App, model: &Model, frame: Frame) {
    model.ui.draw_to_frame(&frame).unwrap();
}

fn raw_ui_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.ui.handle_raw_event(event);
}

fn update_ui(model: &mut Model) {
    let ctx = model.ui.begin_frame();
    egui::Window::new("Schotter Control Panel")
        .collapsible(false)
        .show(&ctx, |ui| {
            ui.add(egui::Slider::new(&mut model.disp_adj, 0.0..=5.0).text("Displacement"));
            ui.add(egui::Slider::new(&mut model.rot_adj, 0.0..=5.0).text("Rotation"));
            ui.horizontal(|ui| {
                if ui.add(egui::Button::new("Randomize")).clicked() {
                    model.random_seed = random_range(0, 1000000);
                }
                ui.add_space(20.0);
                ui.add(egui::DragValue::new(&mut model.random_seed));
                ui.label("Seed");
            })
        });
}
