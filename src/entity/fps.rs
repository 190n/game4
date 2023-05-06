use super::{Behavior, Entity, EntityCommon};
use notan::draw::Draw;
use std::collections::VecDeque;

pub struct Fps {
	dts: VecDeque<f32>,
	counter: u32,
}

impl Fps {
	pub fn new() -> Entity {
		Entity(
			EntityCommon { x: 0.0, y: 0.0 },
			Box::new(Fps {
				dts: VecDeque::with_capacity(60),
				counter: 0,
			}),
		)
	}
}

impl Behavior for Fps {
	fn tick(&mut self, _ent: &mut EntityCommon, dt: f32) -> () {
		while self.dts.len() >= 60 {
			self.dts.pop_front();
		}
		self.dts.push_back(dt);
		self.counter += 1;
		self.counter %= 60;
		if self.counter == 0 {
			let frames = self.dts.len();
			let seconds = self.dts.iter().sum::<f32>();
			println!("{:>8.2} fps", frames as f32 / seconds);
		}
	}

	fn draw(&self, _ent: &EntityCommon, _draw: &mut Draw) -> () {}
}
