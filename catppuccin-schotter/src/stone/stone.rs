use crate::catppuccin::get_random_foreground_color;

pub struct Stone {
    pub x: f32,
    pub y: f32,
    pub x_offset: f32,
    pub y_offset: f32,
    pub rotation: f32,
    pub color: nannou::color::Rgb<u8>,
}

impl Stone {
    pub fn new(x: f32, y: f32) -> Self {
        let x_offset = 0.0;
        let y_offset = 0.0;
        let rotation = 0.0;
        let color = get_random_foreground_color();

        Stone {
            x,
            y,
            x_offset,
            y_offset,
            rotation,
            color,
        }
    }
}
