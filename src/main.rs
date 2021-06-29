use num::complex::Complex;
use ::image::{ImageBuffer, Rgba, Rgb};
use piston_window::{
    PistonWindow,
    WindowSettings,
    image,
    clear,
    color,
    Texture,
    TextureContext,
    TextureSettings
};
use std::error::Error;

const RE_START: f32 = -2.0;
const RE_END: f32 = 1.0;
const IM_START: f32 = -1.0;
const IM_END: f32 = 1.0;

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;

fn main() {
    make_win_and_show("test", mandlebrot);
}

pub fn make_win_and_show<F> (name: &'static str, f: F)
where F: Fn(u32, u32) -> u8
{
    //region calc
    println!("Starting...");

    let mut img = ImageBuffer::new(WIDTH, HEIGHT);
    img.enumerate_pixels_mut().for_each(|(x, y, px)| {
        let px_value = f(x, y);

        *px = Rgba([px_value, px_value, px_value, 255]);
    });


    //endregion

    //region window
    println!("Generated mandelbrot image.");
    img.save(format!("{}-{}x{}", name, WIDTH, HEIGHT));

    let mut window: PistonWindow = WindowSettings::new(name, [WIDTH, HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut texture_context = TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into()
    };

    let texture = Texture::from_image(
        &mut texture_context,
        &img,
        &TextureSettings::new()
    ).unwrap();

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _d| {
            clear(color::WHITE, g);
            image(&texture, c.transform ,g);
        });
    }
    //endregion
}

fn mandlebrot (x: u32, y: u32) -> u8{
    let real = (x as f32 / WIDTH as f32) * (RE_END - RE_START) + RE_START;
    let imaginary = (y as f32 / HEIGHT as f32) * (IM_END - IM_START) + IM_START;
    let px_value = mandelbrot_calc_base(Complex::new(real, imaginary), 500);

    let px_value = ((px_value as f32 / 1000.0) * 255.0) as u8;
    px_value
}

fn mandelbrot_calc_base (c: Complex<f32>, max_iter: u32) -> u32 {
    let mut z = Complex::<f32>::new(0.0, 0.0);
    let mut n = 0;
    while z.norm() < 2.0 && n < max_iter {
        z = z.powu(2) + c;
        n += 1;
    }
    return n;
}