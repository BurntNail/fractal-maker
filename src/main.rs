mod make_and_show;
mod patterns;

use crate::make_and_show::make_win_and_show;
use crate::patterns::*;

pub const WIDTH: u32 = 1920;
pub const HEIGHT: u32 = 1080;
pub const SCALE: f64 = 0.5;

fn main() {
    make_win_and_show("test", mandlebrot);
}
