use num::Complex;
use crate::{WIDTH, HEIGHT};

pub const COOL_BACKING: bool = true;
fn get_backing (x: u32, y: u32) -> Option<(u8, u8)> {
    if COOL_BACKING {
            let r = (0.3 * x as f32) as u8;
            let b = (0.3 * y as f32) as u8;
            Some((r, b))
    } else {
        None
    }
}

pub fn mandlebrot(x: u32, y: u32) -> [u8; 4] {
    let data = get_backing(x, y);
    const RE_START: f32 = -2.0;
    const RE_END: f32 = 1.0;
    const IM_START: f32 = -1.0;
    const IM_END: f32 = 1.0;

    let calc = || {
        let real = (x as f32 / WIDTH as f32) * (RE_END - RE_START) + RE_START;
        let imaginary = (y as f32 / HEIGHT as f32) * (IM_END - IM_START) + IM_START;

        let c = Complex::new(real, imaginary);

        let mut z = Complex::<f32>::new(0.0, 0.0);
        let mut n = 0;
        while z.norm() < 2.0 && n < 500 {
            z = z.powu(2) + c;
            n += 1;
        }
        n
    };


    let px_value = calc();
    let px_value = ((px_value as f32 / 1000.0) * 255.0) as u8;

    match data {
        None => [px_value, px_value, px_value, 255],
        Some((r, b)) => [r, px_value, b, 255]
    }
}

//from: https://crates.io/crates/image
pub fn julia (x: u32, y: u32) -> [u8; 4] {
    let data = get_backing(x, y);

    let calc = || {
        let scalex = 3.0 / WIDTH as f32;
        let scaley = 3.0 / HEIGHT as f32;

        let cx = y as f32 * scalex - 1.5;
        let cy = x as f32 * scaley - 1.5;

        let c = Complex::new(-0.4, 0.6);
        let mut z = Complex::new(cx, cy);

        let mut i = 0;
        while i < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            i += 1;
        }
        i as u8
    };

    let px_value = calc();
    match data {
        None => [px_value, px_value, px_value, 255],
        Some((r, b)) => [r, px_value, b, 255]
    }
}