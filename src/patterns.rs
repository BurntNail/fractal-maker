use crate::{HEIGHT, WIDTH};
use num::Complex;

// pub const COOL_BACKING: bool = false;
// fn get_backing(x: u32, y: u32) -> Option<(f32, f32)> {
//     if COOL_BACKING {
//         let r = (0.3 * x as f32
//         let b = (0.3 * y as f32) as u8;
//         Some((r, b))
//     } else {
//         None
//     }
// }


pub trait Pattern {
    fn get_px (x: u32, y: u32) -> [f32; 4];
}

pub struct Mandlebrot;
pub struct Julia;

impl Pattern for Mandlebrot {
    fn get_px (x: u32, y: u32) -> [f32; 4] {
        const RE_START: f32 = -2.0;
        const RE_END: f32 = 1.0;
        const IM_START: f32 = -1.0;
        const IM_END: f32 = 1.0;
    
        let calc = || {
            let real = (x as f32 / WIDTH as f32) * (RE_END - RE_START) + RE_START;
            let imaginary = (y as f32 / HEIGHT as f32) * (IM_END - IM_START) + IM_START;
    
            let ccomplex_n = Complex::new(real, imaginary);
    
            let mut mutable_cmplex_n = Complex::<f32>::new(0.0, 0.0);
            let mut result = 0;
            while mutable_cmplex_n.norm() < 2.0 && result < 500 {
                mutable_cmplex_n = mutable_cmplex_n * mutable_cmplex_n + ccomplex_n;
                result += 1;
            }
            result
        };
    
        let px_value = calc() as f32 / 100.0;
    
        [px_value, px_value, px_value, 1.0]    
    }
}

impl Pattern for Julia {
    //from: https://crates.io/crates/image
    fn get_px (x: u32, y: u32) -> [f32; 4] {
        let calc = || {
            let scalex = 3.0 / WIDTH as f32;
            let scaley = 3.0 / HEIGHT as f32;
    
            let cx = y as f32 * scalex - 1.5;
            let cy = x as f32 * scaley - 1.5;
    
            let cmplex_n = Complex::new(-0.4, 0.6);
            let mut mutable_cmplex_n = Complex::new(cx, cy);
    
            let mut result = 0;
            while result < 255 && mutable_cmplex_n.norm() <= 2.0 {
                mutable_cmplex_n = mutable_cmplex_n * mutable_cmplex_n + cmplex_n;
                result += 1;
            }
            result as u8
        };
    
        let px_value = calc() as f32 / 256.0;
        [px_value, px_value, px_value, 1.0]
    
    }
}