use nannou::prelude::*;

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 1000;

struct Model {}

fn main() {
    nannou::app(init)
        .update(update)
        .loop_mode(LoopMode::refresh_sync())
        .run();
}

fn init(app: &App) -> Model {
    let _window = app
        .new_window()
        .title(app.exe_name().unwrap())
        .size(WIDTH, HEIGHT)
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(_app: &App, _model: &Model, _frame: Frame) {}

fn key_pressed(_app: &App, _model: &mut Model, _key: Key) {}
