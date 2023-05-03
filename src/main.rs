use entity::Entity;
use notan::{draw::*, prelude::*};

mod entity;

#[derive(AppState)]
struct State {
	entities: Vec<Entity>,
}

fn initial_state() -> State {
	State {
		entities: vec![entity::Looper::new()],
	}
}

#[notan_main]
fn main() -> Result<(), String> {
	notan::init_with(initial_state)
		.add_config(DrawConfig)
		.draw(draw)
		.build()
}

fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
	let mut canvas = gfx.create_draw();
	canvas.clear(Color::BLACK);
	for e in &mut state.entities {
		e.tick(app.timer.delta_f32());
		e.draw(&mut canvas);
	}
	gfx.render(&canvas);
}
