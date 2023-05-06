use notan::{
	draw::{Draw, DrawShapes},
	prelude::Color,
};

use super::{Behavior, Entity, EntityCommon};
use crate::config::{SKY_HEIGHT, WORLD_SIZE};

pub struct Backdrop;

impl Backdrop {
	pub fn new() -> Entity {
		Entity(EntityCommon { x: 0.0, y: 0.0 }, Box::new(Backdrop))
	}
}

impl Behavior for Backdrop {
	fn draw(&self, _ent: &EntityCommon, draw: &mut Draw) -> () {
		// sky
		draw.rect((0.0, 0.0), (WORLD_SIZE, SKY_HEIGHT))
			.color(Color::from_hex(0x80a0ffff));
		// ocean
		draw.rect((0.0, SKY_HEIGHT), (WORLD_SIZE, WORLD_SIZE - SKY_HEIGHT))
			.color(Color::from_hex(0x002080ff));
	}

	fn tick(&mut self, _ent: &mut EntityCommon, _dt: f32) -> () {}
}
