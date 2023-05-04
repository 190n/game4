use entity::Entity;
use notan::{
	draw::*,
	math::{Mat3, Vec2},
	prelude::*,
};

mod entity;

#[derive(AppState)]
struct State {
	entities: Vec<Entity>,
}

fn initial_state() -> State {
	State {
		entities: vec![entity::Looper::new(), entity::Fps::new()],
	}
}

#[notan_main]
fn main() -> Result<(), String> {
	notan::init_with(initial_state)
		.add_config(DrawConfig)
		.add_config(
			notan::app::WindowConfig::new()
				.vsync(true)
				.title("game4")
				.size(400, 400)
				.resizable(true)
				.high_dpi(true),
		)
		.draw(draw)
		.initialize(initialize)
		.build()
}

fn initialize(app: &mut App) {
	app.window().set_size(400, 400);
}

fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
	let mut canvas = gfx.create_draw();
	let (w, h) = gfx.size();
	canvas.clear(Color::BLACK);
	canvas
		.rect((5.0, 5.0), (w as f32 - 10.0, h as f32 - 10.0))
		.color(Color::RED);
	for e in &mut state.entities {
		canvas.transform().clear();
		let trans = Mat3::from_translation(Vec2::new(e.0.x, e.0.y));
		canvas.transform().push(trans);
		e.tick(app.timer.delta_f32());
		e.draw(&mut canvas);
	}
	gfx.render(&canvas);
}
