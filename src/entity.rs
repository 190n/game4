use notan::draw::*;
use std::collections::VecDeque;

pub trait Behavior {
	fn tick(&mut self, ent: &mut EntityCommon, dt: f32) -> ();
	fn draw(&self, ent: &EntityCommon, canvas: &mut Draw) -> ();
}

pub struct EntityCommon {
	pub x: f32,
	pub y: f32,
}

pub struct Entity(pub EntityCommon, pub Box<dyn Behavior>);

impl Entity {
	pub fn tick(&mut self, dt: f32) -> () {
		self.1.tick(&mut self.0, dt);
	}

	pub fn draw(&self, canvas: &mut Draw) -> () {
		self.1.draw(&self.0, canvas);
	}
}

pub struct Looper {
	t: f32,
}

impl Looper {
	pub fn new() -> Entity {
		Entity(EntityCommon { x: 0.0, y: 0.0 }, Box::new(Looper { t: 0.0 }))
	}
}

impl Behavior for Looper {
	fn tick(&mut self, ent: &mut EntityCommon, dt: f32) -> () {
		self.t += dt;
		self.t %= 2.0 * std::f32::consts::PI;
		ent.x = f32::cos(self.t) * 100.0 + 200.0;
		ent.y = f32::sin(self.t) * 100.0 + 200.0;
	}

	fn draw(&self, _ent: &EntityCommon, canvas: &mut Draw) -> () {
		canvas.rect((-20.0, -20.0), (40.0, 40.0));
	}
}

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

	fn draw(&self, _ent: &EntityCommon, _canvas: &mut Draw) -> () {}
}
