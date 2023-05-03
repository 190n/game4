use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use std::time::Duration;

mod entity;

pub fn main() {
	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();

	let window = video_subsystem
		.window("rust-sdl2 demo", 800, 600)
		.position_centered()
		.build()
		.unwrap();

	let mut canvas = window.into_canvas().build().unwrap();

	let mut ents = vec![entity::Looper::new()];

	canvas.clear();
	canvas.present();
	let mut event_pump = sdl_context.event_pump().unwrap();
	let mut i = 0;
	'running: loop {
		i = (i + 1) % 255;
		canvas.set_draw_color(Color::BLACK);
		canvas.clear();

		for e in &mut ents {
			e.tick(1.0 / 60.0);
			e.draw(&mut canvas);
		}

		for event in event_pump.poll_iter() {
			match event {
				Event::Quit { .. }
				| Event::KeyDown {
					keycode: Some(Keycode::Escape),
					..
				} => break 'running,
				_ => {},
			}
		}
		// The rest of the game loop goes here...

		canvas.present();
		std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
	}
}
