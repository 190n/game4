use notan::{
	draw::*,
	math::{Mat3, Vec2},
	prelude::*,
};

use crate::entity::{Backdrop, Entity, Fps};

#[derive(AppState)]
pub struct Manager {
	pub entities: Vec<Entity>,
}

impl Manager {
	pub fn new() -> Manager {
		Manager {
			entities: vec![Fps::new(), Backdrop::new()],
		}
	}

	pub fn draw(app: &mut App, gfx: &mut Graphics, manager: &mut Manager) {
		let mut draw = gfx.create_draw();
		draw.clear(Color::BLACK);
		for e in &mut manager.entities {
			draw.transform().clear();
			let trans = Mat3::from_translation(Vec2::new(e.0.x, e.0.y));
			draw.transform().push(trans);
			e.tick(app.timer.delta_f32());
			e.draw(&mut draw);
		}
		gfx.render(&draw);
	}
}
