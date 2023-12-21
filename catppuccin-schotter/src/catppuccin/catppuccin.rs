use nannou::color::Srgb;
use rand::seq::SliceRandom;
use std::marker::PhantomData;

pub fn get_random_foreground_color() -> Srgb<u8> {
    let foreground_colors: Vec<Srgb<u8>> = vec![LAVENDER, PINK, RED, GREEN, BLUE];
    *foreground_colors.choose(&mut rand::thread_rng()).unwrap()
}

pub const BASE: nannou::color::Srgb<u8> = nannou::color::Srgb {
    red: 30,
    green: 30,
    blue: 46,
    standard: PhantomData,
};

pub const LAVENDER: nannou::color::Srgb<u8> = nannou::color::Srgb {
    red: 180,
    green: 190,
    blue: 254,
    standard: PhantomData,
};

pub const PINK: nannou::color::Srgb<u8> = nannou::color::Srgb {
    red: 245,
    green: 194,
    blue: 231,
    standard: PhantomData,
};

pub const RED: nannou::color::Srgb<u8> = nannou::color::Srgb {
    red: 243,
    green: 139,
    blue: 168,
    standard: PhantomData,
};

pub const GREEN: nannou::color::Srgb<u8> = nannou::color::Srgb {
    red: 166,
    green: 227,
    blue: 161,
    standard: PhantomData,
};

pub const BLUE: nannou::color::Srgb<u8> = nannou::color::Srgb {
    red: 137,
    green: 180,
    blue: 250,
    standard: PhantomData,
};
