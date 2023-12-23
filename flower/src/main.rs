use lazy_static::lazy_static;

use nannou::prelude::*;

use chrono::Utc;

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 1000;
const NUM_POINTS: usize = 1000;
lazy_static! {
    static ref GROWTH_COLORS: Vec<Srgb<u8>> = {
        let mut v = Vec::new();
        v.push(SNOW);
        v.push(SNOW);
        v.push(SNOW);
        v
    };
    static ref NUM_GROWTHS: usize = GROWTH_COLORS.len();
}

struct Growth {
    center_x: f32,
    center_y: f32,
    points: Vec<MyPoint>,
}

impl Growth {
    fn new(color: Srgb<u8>) -> Self {
        let center_x = random_range(-1.0 * WIDTH as f32 / 4.0, WIDTH as f32 / 4.0);
        let center_y = random_range(-1.0 * HEIGHT as f32 / 4.0, HEIGHT as f32 / 4.0);
        let mut points: Vec<MyPoint> = Vec::new();
        for _ in 0..NUM_POINTS {
            points.push(MyPoint::new(color));
        }
        Growth {
            center_x,
            center_y,
            points,
        }
    }
}

struct MyPoint {
    // Struct which will move along the line
    // y = sin(ax) * b
    a: f32,
    b: f32,
    x: f32,
    y: f32,
    color: Srgb<u8>,
}

impl MyPoint {
    fn new(color: Srgb<u8>) -> Self {
        let a = random_range(0.1, 1.5);
        let b = random_range(0.5, 7.0);
        MyPoint {
            a,
            b,
            x: 0.0,
            y: 0.0,
            color,
        }
    }

    fn advance(&mut self) {
        let adj = random_range(0.1, 1.0);
        self.x -= adj;
        self.y = f32::sin(self.a * self.x) * self.b;
    }
}

struct Model {
    growths: Vec<Growth>,
}

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

    let mut growths: Vec<Growth> = Vec::new();
    for i in 0..*NUM_GROWTHS {
        let color = *GROWTH_COLORS.get(i).unwrap();
        growths.push(Growth::new(color));
    }

    Model { growths }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for i in 0..*NUM_GROWTHS {
        for j in 0..NUM_POINTS {
            model
                .growths
                .get_mut(i)
                .unwrap()
                .points
                .get_mut(j)
                .unwrap()
                .advance();
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    // draw.background().color(BLACK);
    draw.rect()
        .w_h(WIDTH as f32, HEIGHT as f32)
        .color(Rgba::new(0.0, 0.0, 0.0, 0.03));

    // Draw each point
    let rad = 2.0 * PI / NUM_POINTS as f32;

    for i in 0..*NUM_GROWTHS {
        let growth = model.growths.get(i).unwrap();
        for j in 0..NUM_POINTS {
            let pdraw = draw
                .x_y(growth.center_x, growth.center_y)
                .rotate(rad * j as f32);
            let point = growth.points.get(j).unwrap();
            pdraw
                .ellipse()
                .radius(1.0)
                .x_y(point.x, point.y)
                .color(point.color);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}

fn key_pressed(app: &App, _model: &mut Model, key: Key) {
    match key {
        Key::S => {
            let now = Utc::now().format("%Y-%m-%dT%H_%M_%S%.3fZ");
            app.main_window().capture_frame(format!(
                "images/{}_{}.png",
                app.exe_name().unwrap(),
                now
            ));
        }
        _other_key => {}
    }
}
