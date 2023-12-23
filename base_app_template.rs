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

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    draw.to_frame(app, &frame).unwrap();
}

fn key_pressed(_app: &App, _model: &mut Model, key: Key) {
    match key {
        _other_key => {}
    }
}
