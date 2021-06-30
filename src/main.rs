mod patterns;
mod make_and_show;

use ::image::{ImageBuffer, Rgba};
use num::complex::Complex;
use piston_window::{
    clear, color, image, PistonWindow, Texture, TextureContext, TextureSettings, WindowSettings,
};
use crate::patterns::*;
use crate::make_and_show::make_win_and_show;

pub const WIDTH: u32 = 1920;
pub const HEIGHT: u32 = 1080;

fn main() {
    make_win_and_show("test", mandlebrot);
}