mod make_and_show;
mod patterns;

use crate::make_and_show::make_win_and_show;
use crate::patterns::{Mandlebrot};

pub const WIDTH: u32 = 136;
pub const HEIGHT: u32 = 77;
pub const SCALE: f64 = 10.0;

fn main() {
    make_win_and_show::<Mandlebrot>("test");
}
