use crate::patterns::Pattern;
use crate::{HEIGHT, SCALE, WIDTH};
extern crate image as img;
use piston_window::{
    clear, rectangle, PistonWindow, RenderEvent, Transformed, WindowSettings,
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::sync::mpsc::channel;
use std::thread;

pub fn make_win_and_show<F>(name: &'static str)
where
    F: Pattern,
{
    let (tx, rx) = channel();

    thread::spawn(move || {
        (0..WIDTH).into_par_iter().for_each_with(tx, |tx, x| {
        // (0..WIDTH).into_iter().for_each(|x| {
            for y in 0..HEIGHT {
                tx.send((x, y, F::get_px(x, y))).unwrap();
            }
            // std::thread::sleep(std::time::Duration::from_secs(5));
        });
    });

    let mut the_list = Vec::with_capacity((WIDTH * HEIGHT) as usize);
    let mut window: PistonWindow = WindowSettings::new(name, [((WIDTH as f64) * SCALE) as u32, ((HEIGHT as f64) * SCALE) as u32])
        .exit_on_esc(true)
        .vsync(true)
        .build()
        .unwrap();
    println!("Made window");

    while let Some(e) = window.next() {
        println!("Event: {:?}", &e);
        while let Ok(stuff) = rx.recv() {
            the_list.push(stuff);
        }

        if let Some(_r) = e.render_args() {
            window.draw_2d(&e, |c, g, _d| {
                println!("Rendering!");
                clear([1.0; 4], g);
                for (x, y, px) in &the_list {
                    rectangle(
                        *px,
                        [0.0, 0.0, 10.0, 10.0],
                        c.transform.trans(*x as f64 * SCALE, *y as f64 * SCALE),
                        g,
                    );
                }
            });
        }
    }
}