use notan::{
	draw::{Draw, DrawShapes},
	prelude::Color,
};

use super::{Behavior, Entity, EntityCommon};
use crate::config::{SHIP_SIZE, SHIP_SPEED, SKY_HEIGHT, WORLD_SIZE};

pub struct Ship {
	/// -1.0 or 1.0
	facing: f32,
}

impl Ship {
	pub fn new() -> Entity {
		Entity(
			EntityCommon {
				x: SHIP_SIZE.0 / 2.0,
				y: SKY_HEIGHT - SHIP_SIZE.1 / 2.0,
			},
			Box::new(Ship { facing: 1.0 }),
		)
	}
}

impl Behavior for Ship {
	fn draw(&self, _ent: &EntityCommon, draw: &mut Draw) -> () {
		draw.rect((-SHIP_SIZE.0 / 2.0, 0.0), SHIP_SIZE)
			.fill_color(Color::BLACK);
		draw.rect(
			(-1.0 * SHIP_SIZE.0 * self.facing / 2.0, -8.0),
			(8.0 * self.facing, 8.0),
		)
		.fill_color(Color::BLACK);
	}

	fn tick(&mut self, ent: &mut EntityCommon, dt: f32) -> () {
		let mut dx = dt * SHIP_SPEED * self.facing;

		let min_x = SHIP_SIZE.0 / 2.0;
		let max_x = WORLD_SIZE - min_x;

		while dx != 0.0 {
			ent.x += dx;
			if ent.x > max_x {
				dx = max_x - ent.x;
				ent.x = max_x;
				self.facing *= -1.0;
			} else if ent.x < min_x {
				dx = min_x - ent.x;
				ent.x = min_x;
				self.facing *= -1.0;
			} else {
				dx = 0.0;
			}
		}
	}
}
