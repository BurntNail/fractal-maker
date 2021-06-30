use crate::{WIDTH, HEIGHT};
extern crate image as img;
use img::{Rgba, ImageBuffer};
use piston_window::{PistonWindow, WindowSettings, TextureContext, Texture, TextureSettings, clear, image};

pub fn make_win_and_show<F>(name: &'static str, f: F)
    where
        F: Fn(u32, u32) -> [u8; 4],
{
    //region calc
    println!("Starting...");

    let mut img = ImageBuffer::new(WIDTH, HEIGHT);
    img.enumerate_pixels_mut().for_each(|(x, y, px)| *px = Rgba(f(x, y)));

    //endregion

    //region window
    println!("Generated mandelbrot image.");
    img.save(format!("{}-{}x{}.png", name, WIDTH, HEIGHT))
        .unwrap_or_else(|e| {
            eprintln!("Error saving image: {}", e);
            std::process::exit(1);
        });

    let mut window: PistonWindow = WindowSettings::new(name, [WIDTH, HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut texture_context = TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into(),
    };

    let texture = Texture::from_image(&mut texture_context, &img, &TextureSettings::new()).unwrap();

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _d| {
            clear([1.0; 4], g);
            image(&texture, c.transform, g);
        });
    }
    //endregion
}